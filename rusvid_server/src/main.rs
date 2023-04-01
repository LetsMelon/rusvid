use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use axum::body::StreamBody;
use axum::extract::{DefaultBodyLimit, Multipart, Path};
use axum::http::{header, HeaderMap, HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{any, get, post};
use axum::{Json, Router};
use fern::Dispatch;
use log::LevelFilter;
use rusvid_lib::composition::Composition;
use rusvid_lib::prelude::holder::utils::random_id;
use rusvid_lib::prelude::EmbeddedRenderer;
use rusvid_lib::renderer::Renderer;
use serde::Serialize;
use serde_json::json;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio_util::io::ReaderStream;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

#[derive(Debug)]
struct SharedData {
    composition: Composition,
    id: String,
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
enum ItemStatus {
    #[default]
    Pending,
    Processing,
    Finish,
}

#[derive(Debug, Default, Clone, Serialize)]
struct ItemList {
    list: HashMap<String, ItemStatus>,
}

type SharedItemList = Arc<RwLock<ItemList>>;

#[tokio::main]
async fn main() {
    Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let (tx, mut rx): (UnboundedSender<SharedData>, UnboundedReceiver<SharedData>) =
        mpsc::unbounded_channel();
    let shared_item_list = SharedItemList::default();

    let items = Arc::clone(&shared_item_list);
    let _render_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("{}: {:?}", message.id, message.composition);

            items
                .write()
                .unwrap()
                .list
                .insert(message.id.clone(), ItemStatus::Processing);

            let mut renderer = EmbeddedRenderer::new(format!("{}.mp4", message.id));
            renderer.render(message.composition).unwrap();

            items
                .write()
                .unwrap()
                .list
                .insert(message.id.clone(), ItemStatus::Finish);
        }
    });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", any(|| async { StatusCode::OK }))
        .route(
            "/upload",
            post({
                let shared_state = tx.clone();
                let shared_list = shared_item_list.clone();

                move |multipart| accept_form(multipart, shared_state, shared_list)
            })
            .layer(
                ServiceBuilder::new()
                    .layer(DefaultBodyLimit::disable())
                    .layer(RequestBodyLimitLayer::new(1 * 1024 * 1024))
                    .layer(CompressionLayer::new()),
            ),
        )
        .route(
            "/status/all",
            get({
                let shared_list = shared_item_list.clone();

                move || get_all_videos(shared_list)
            })
            .layer(CompressionLayer::new()),
        )
        .route(
            "/status/id/:id",
            get({
                let shared_list = shared_item_list.clone();

                move |path| get_status_video(path, shared_list)
            })
            .layer(CompressionLayer::new()),
        )
        .route(
            "/video/id/:id",
            get({
                let shared_list = shared_item_list.clone();
                move |path| get_video(path, shared_list)
            })
            .layer(CompressionLayer::new()),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin("*".parse::<HeaderValue>().unwrap())
                        .allow_methods([Method::GET, Method::POST]),
                ),
        )
        .fallback(|| async { StatusCode::NOT_FOUND });

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn accept_form(
    mut multipart: Multipart,
    tx: UnboundedSender<SharedData>,
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
    tx.send(SharedData {
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

async fn get_video(Path(id): Path<String>, list: SharedItemList) -> impl IntoResponse {
    let item = list.read().unwrap().list.get(&id).cloned();
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

async fn get_all_videos(list: SharedItemList) -> Json<ItemList> {
    let items = list.read().unwrap().clone();

    Json(items)
}

async fn get_status_video(
    Path(id): Path<String>,
    list: SharedItemList,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let item = list.read().unwrap().list.get(&id).cloned();

    match item {
        Some(status) => Ok(Json(json!({ "id": id, "status": status}))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
