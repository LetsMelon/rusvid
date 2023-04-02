use axum::body::StreamBody;
use axum::extract::{Multipart, Path};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use rusvid_lib::composition::Composition;
use rusvid_lib::core::holder::utils::random_id;
use tokio::sync::mpsc::UnboundedSender;
use tokio_util::io::ReaderStream;

use crate::render_task::Message;
use crate::status_types::{ItemStatus, SharedItemList};

pub async fn upload_video(
    mut multipart: Multipart,
    tx: UnboundedSender<Message>,
    list: SharedItemList,
) -> impl IntoResponse {
    let mut file = None;
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if name == "file" {
            file = Some(data);
            break;
        }
    }

    let id = random_id();

    let out = serde_yaml::from_slice::<Composition>(&file.unwrap()).unwrap();
    tx.send(Message {
        composition: out,
        id: id.clone(),
    })
    .unwrap();

    // let mut renderer = EmbeddedRenderer::new("out.mp4");
    // renderer.render(out.unwrap()).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::ETAG, id.clone().parse().unwrap());

    list.write().unwrap().list.insert(id, ItemStatus::default());

    (StatusCode::CREATED, headers)
}

pub async fn download_video(
    Path(id): Path<String>,
    shared_list: SharedItemList,
) -> impl IntoResponse {
    let item = shared_list.read().unwrap().list.get(&id).cloned();
    match item {
        Some(stat) => match stat {
            ItemStatus::Finish => (),
            _ => {
                return Err((
                    StatusCode::PROCESSING,
                    "Video is still being processed".to_string(),
                ))
            }
        },
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("File not found with id: {id}"),
            ))
        }
    };

    let file = match tokio::fs::File::open(format!("{id}.mp4")).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {err}"))),
    };

    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

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
