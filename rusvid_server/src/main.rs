use std::net::SocketAddr;

use axum::http::{HeaderValue, Method, StatusCode};
use axum::routing::{any, get};
use axum::Router;
use fern::Dispatch;
use log::LevelFilter;
use r2d2_redis::RedisConnectionManager;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod error;
mod logic;
mod render_task;
mod routes;
mod status_types;
mod util;

#[tokio::main]
async fn main() {
    Dispatch::new()
        .level(LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    let access_key = "access_key_123";
    let secret_key = "access_secrect_key_123";

    // let client = Client::open("redis://127.0.0.1/").unwrap();
    // let conn = client.get_multiplexed_tokio_connection().await.unwrap();
    let client = RedisConnectionManager::new("redis://127.0.0.1/").unwrap();
    let pool = r2d2_redis::r2d2::Pool::builder().build(client).unwrap();

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
