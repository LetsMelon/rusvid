use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::{HeaderValue, Method, StatusCode};
use axum::routing::{any, get};
use axum::Router;
use fern::Dispatch;
use log::LevelFilter;
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::status_types::SharedItemList;

mod logic;
mod render_task;
mod routes;
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
        .nest(
            "/status",
            routes::status::router(Arc::clone(&shared_item_list)),
        )
        .nest(
            "/video",
            routes::video::router(tx, Arc::clone(&shared_item_list)),
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
