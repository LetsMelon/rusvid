#![feature(once_cell_try)]

use std::future::ready;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, Instant};

use ::redis::Client;
use axum::extract::MatchedPath;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{any, get};
use axum::Router;
use error::ApiError;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use r2d2::Pool;
use s3::creds::Credentials;
use s3::{Bucket, Region};
use tokio::sync::mpsc;
use tower::ServiceBuilder;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{debug, info, info_span, Span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rusvid_server=debug,tower_http=debug,rusvid_lib=info,rusvid_core=info,rusvid_video_encoder=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (_main_server, _metrics_server) =
        tokio::join!(start_main_server(8080), start_metrics_server(8085));
}

async fn start_main_server(port: u16) {
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
        .route_layer(middleware::from_fn(track_metrics))
        .fallback(|| async { ApiError::NotFound });

    let addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    info!("main server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn start_metrics_server(port: u16) {
    let recorder_handle = setup_metrics_recorder();
    let app = Router::new().route("/metrics", get(move || ready(recorder_handle.render())));

    let addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    info!("metrics server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    response
}
