use axum::body::StreamBody;
use axum::extract::{Multipart, Path};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use rusvid_lib::composition::Composition;
use rusvid_lib::core::holder::utils::random_id;
use s3::Bucket;
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::io::ReaderStream;

use crate::error::ApiError;
use crate::render_task::Message;
use crate::status_types::{ItemStatus, SharedItemList};
use crate::util::{format_file_path, format_s3_file_path};

pub async fn upload_video(
    mut multipart: Multipart,
    tx: UnboundedSender<Message>,
    list: SharedItemList,
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

        let out = serde_yaml::from_slice::<Composition>(&file)?;

        tx.send(Message {
            composition: out,
            id: id.clone(),
        })?;

        let mut headers = HeaderMap::new();
        headers.insert(header::ETAG, id.clone().parse().unwrap());

        list.write()?.list.insert(id, ItemStatus::default());

        Ok((StatusCode::CREATED, headers))
    } else {
        Err(ApiError::FileNotFound)
    }
}

pub async fn download_video(
    Path(id): Path<String>,
    shared_list: SharedItemList,
    bucket: Bucket,
) -> Result<impl IntoResponse, ApiError> {
    let item = shared_list.read()?.list.get(&id).cloned();

    match item {
        Some(stat) => match stat {
            ItemStatus::Finish => (),
            _ => return Err(ApiError::VideoInProcess),
        },
        None => return Err(ApiError::FileNotFound),
    };

    // TODO don't save the file to the local file system before sending it
    let local_file_path = format_file_path(&id);
    let mut async_output_file = tokio::fs::File::create(&local_file_path).await?;

    bucket
        .get_object_stream(format_s3_file_path(&id), &mut async_output_file)
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
        HeaderValue::from_str("video/mp4").unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str("attachment; filename=\"video.mp4\"").unwrap(),
    );

    Ok((headers, body))
}

pub async fn delete_video(
    Path(id): Path<String>,
    shared_list: SharedItemList,
    bucket: Bucket,
) -> Result<impl IntoResponse, ApiError> {
    let item = shared_list.write().unwrap().list.remove(&id);
    match item {
        Some(ItemStatus::Pending) => (),
        Some(ItemStatus::Processing) => {
            shared_list
                .write()?
                .list
                .insert(id.clone(), ItemStatus::InDeletion);
        }
        Some(ItemStatus::Finish) => {
            bucket.delete_object(format_s3_file_path(&id)).await?;
        }
        Some(ItemStatus::InDeletion) => {
            bucket.delete_object(format_s3_file_path(&id)).await?;
        }
        None => (),
    };

    Ok(StatusCode::OK)
}
