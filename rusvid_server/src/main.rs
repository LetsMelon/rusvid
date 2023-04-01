use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::routing::{any, get, post};
use axum::Router;
use fern::Dispatch;
use log::LevelFilter;
use rusvid_lib::composition::Composition;
use rusvid_lib::renderer::embedded::EmbeddedRenderer;
use rusvid_lib::renderer::Renderer;
use serde::Serialize;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

use crate::handler::{status, video};

mod handler;

#[derive(Debug)]
pub struct SharedData {
    composition: Composition,
    id: String,
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub enum ItemStatus {
    #[default]
    Pending,
    Processing,
    Finish,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ItemList {
    list: HashMap<String, ItemStatus>,
}

pub type SharedItemList = Arc<RwLock<ItemList>>;

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
            "/status/all",
            get({
                let shared_list = shared_item_list.clone();

                move || status::list_all_items(shared_list)
            })
            .layer(CompressionLayer::new()),
        )
        .route(
            "/status/id/:id",
            get({
                let shared_list = shared_item_list.clone();

                move |path| status::single_status(path, shared_list)
            })
            .layer(CompressionLayer::new()),
        )
        .route(
            "/video/upload",
            post({
                let shared_state = tx.clone();
                let shared_list = shared_item_list.clone();

                move |multipart| video::upload_video(multipart, shared_state, shared_list)
            })
            .layer(
                ServiceBuilder::new()
                    .layer(DefaultBodyLimit::disable())
                    .layer(RequestBodyLimitLayer::new(1 * 1024 * 1024))
                    .layer(CompressionLayer::new()),
            ),
        )
        .route(
            "/video/id/:id",
            get({
                let shared_list = shared_item_list.clone();
                move |path| video::download_video(path, shared_list)
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
