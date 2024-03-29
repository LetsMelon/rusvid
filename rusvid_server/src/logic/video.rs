use std::sync::OnceLock;

use axum::body::StreamBody;
use axum::extract::{Multipart, Path};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use r2d2::Pool;
use redis::{Client, Commands, RedisError};
use rusvid_lib::composition::Composition;
use rusvid_lib::core::holder::utils::random_id;
use s3::Bucket;
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::io::ReaderStream;
use tracing::instrument;

use crate::error::ApiError;
use crate::redis::key_for_video_status;
use crate::render_task::Message;
use crate::status_types::ItemStatus;
use crate::util::{format_file_path, format_s3_file_path};

static HEADER_VALUE_CONTENT_TYPE: OnceLock<HeaderValue> = OnceLock::new();
static HEADER_VALUE_CONTENT_DISPOSITION: OnceLock<HeaderValue> = OnceLock::new();

#[instrument(skip_all)]
pub async fn upload_video(
    mut multipart: Multipart,
    tx: UnboundedSender<Message>,
    redis_pool: Pool<Client>,
) -> Result<impl IntoResponse, ApiError> {
    let mut file = None;
    while let Some(field) = multipart.next_field().await? {
        let field_name = field.name().and_then(|value| Some(value.to_string()));
        let field_data = field.bytes().await?;

        if let Some(name) = field_name {
            if name == "file" {
                file = Some(field_data);
                break;
            }
        }
    }

    if let Some(file) = file {
        let id = random_id();

        let mut connection = redis_pool.get()?;
        let _: () = connection.set(key_for_video_status(&id), ItemStatus::default())?;

        tx.send(Message {
            composition: serde_yaml::from_slice::<Composition>(&file)?,
            id: id.clone(),
        })?;

        let mut headers = HeaderMap::new();
        headers.insert("x-video-id", id.clone().parse()?);

        let body = Json(json!({ "id": id, "status": ItemStatus::default() }));

        Ok((StatusCode::CREATED, headers, body))
    } else {
        Err(ApiError::UploadError("file".to_string()))
    }
}

pub async fn download_video(
    Path(id): Path<String>,
    bucket: Bucket,
    redis_pool: Pool<Client>,
) -> Response {
    let result = download_video_function(id, bucket, redis_pool).await;

    match result {
        Ok(ok) => ok.into_response(),
        Err(err) => err.into_response(),
    }
}

async fn download_video_function(
    id: String,
    bucket: Bucket,
    redis_pool: Pool<Client>,
) -> Result<impl IntoResponse, ApiError> {
    let mut connection = redis_pool.get()?;
    let item: Option<ItemStatus> = connection.get(key_for_video_status(&id))?;

    // TODO remove them?
    drop(connection);
    drop(redis_pool);

    match item {
        Some(stat) => match stat {
            ItemStatus::Finish => (),
            ItemStatus::Pending | ItemStatus::Processing => return Err(ApiError::VideoInProcess),
            ItemStatus::InDeletion => return Err(ApiError::ResourceNotFound(id)),
            ItemStatus::EncounteredError => return Err(ApiError::VideoEncounteredError),
        },
        None => return Err(ApiError::ResourceNotFound(id)),
    };

    // TODO don't save the file to the local file system before sending it
    let local_file_path = format_file_path(&id);
    let mut async_output_file = tokio::fs::File::create(&local_file_path).await?;

    bucket
        .get_object_to_writer(format_s3_file_path(&id), &mut async_output_file)
        .await?;

    async_output_file.flush().await?;
    drop(async_output_file);

    let file = tokio::fs::File::open(&local_file_path).await?;

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    tokio::fs::remove_file(local_file_path).await?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HEADER_VALUE_CONTENT_TYPE
            .get_or_try_init(|| HeaderValue::from_str("video/mp4"))
            .cloned()?,
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HEADER_VALUE_CONTENT_DISPOSITION
            .get_or_try_init(|| HeaderValue::from_str("attachment; filename=\"video.mp4\""))
            .cloned()?,
    );

    Ok((headers, body))
}

// TODO check if this function really deletes a resource from the server
#[instrument(skip(bucket, redis_pool))]
pub async fn delete_video(
    Path(id): Path<String>,
    bucket: Bucket,
    redis_pool: Pool<Client>,
) -> Result<impl IntoResponse, ApiError> {
    let mut connection = redis_pool.get()?;

    let item: Result<ItemStatus, RedisError> = connection.get_del(key_for_video_status(&id));

    match item {
        Ok(ItemStatus::Processing) => {
            let _: () = connection.set(key_for_video_status(&id), ItemStatus::InDeletion)?;
        }
        Ok(ItemStatus::Finish) | Ok(ItemStatus::InDeletion) => {
            bucket.delete_object(format_s3_file_path(&id)).await?;
        }
        Ok(ItemStatus::Pending) | Ok(ItemStatus::EncounteredError) | Err(_) => (),
    };

    Ok(StatusCode::OK)
}
