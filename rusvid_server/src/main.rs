#![feature(once_cell_try)]

use std::net::SocketAddr;
use std::time::Duration;

use ::redis::Client;
use axum::extract::MatchedPath;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use axum::response::Response;
use axum::routing::{any, get};
use axum::Router;
use error::ApiError;
// use fern::Dispatch;
// use log::LevelFilter;
use r2d2::Pool;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{debug, info_span, Span};
use tracing_subscriber::fmt::format::FmtSpan;

mod error;
mod logic;
mod macros;
mod redis;
mod render_task;
mod routes;
mod status_types;
mod util;

#[tokio::main]
async fn main() {
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or("rusvid_server=debug,axum::rejection=trace".to_string());

    tracing_subscriber::fmt()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let access_key = "access_key_123";
    let secret_key = "access_secret_key_123";
    let redis_url = "redis://127.0.0.1/";

    let client = Client::open(redis_url).unwrap();
    let pool = Pool::builder().build(client).unwrap();

    let bucket = Bucket::new(
        "test-from-rust",
        Region::Custom {
            region: "home".to_string(),
            endpoint: "http://localhost:9000".to_string(),
        },
        Credentials::new(Some(access_key), Some(secret_key), None, None, None).unwrap(),
    )
    .unwrap()
    .with_path_style();

    let (tx, rx) = mpsc::unbounded_channel();

    tokio::spawn({
        let cloned_bucket = bucket.clone();
        // let cloned_multi_redis = conn.clone();
        let redis_pool = pool.clone();

        move || async move { render_task::renderer(rx, cloned_bucket, redis_pool).await }
    }());

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", any(|| async { StatusCode::OK }))
        .nest("/status", routes::status::router(pool.clone()))
        .nest("/video", routes::video::router(tx, bucket, pool.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(|request: &Request<_>| {
                            let matched_path = request
                                .extensions()
                                .get::<MatchedPath>()
                                .map(MatchedPath::as_str);

                            info_span!(
                                "http_request",
                                method = ?request.method(),
                                matched_path,
                            )
                        })
                        .on_response(|response: &Response, _latency: Duration, span: &Span| {
                            let _enter = span.enter();

                            debug!("response.status: {:?}", response.status());
                        })
                        .on_failure(
                            |error: ServerErrorsFailureClass, _latency: Duration, span: &Span| {
                                let _enter = span.enter();

                                tracing::error!("err: {:?}", error);
                            },
                        ),
                )
                .layer(
                    CorsLayer::new()
                        .allow_origin("*".parse::<HeaderValue>().unwrap())
                        .allow_methods([Method::GET, Method::POST]),
                ),
        )
        .fallback(|| async { ApiError::NotFound });

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
