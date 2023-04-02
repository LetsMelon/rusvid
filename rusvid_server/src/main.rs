use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::DefaultBodyLimit;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::routing::{any, get, post};
use axum::Router;
use fern::Dispatch;
use log::LevelFilter;
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

use crate::logic::{status, video};
use crate::status_types::SharedItemList;

mod logic;
mod render_task;
mod status_types;

#[tokio::main]
async fn main() {
    Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let (tx, rx) = mpsc::unbounded_channel();
    let shared_item_list = SharedItemList::default();

    tokio::spawn({
        let shared_list = Arc::clone(&shared_item_list);
        move || async move { render_task::renderer(rx, shared_list).await }
    }());

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
                    .layer(RequestBodyLimitLayer::new(1 * 1024 * 1024)),
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
