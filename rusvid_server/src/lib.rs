#![feature(once_cell_try)]

use std::future::ready;
use std::time::{Duration, Instant};

use ::redis::Client;
use axum::extract::MatchedPath;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{any, get};
use axum::{middleware, Router};
use error::ApiError;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use r2d2::Pool;
use render_task::Message;
use s3::Bucket;
use tokio::sync::mpsc::UnboundedSender;
use tower::ServiceBuilder;
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing::{debug, info_span, Span};

pub mod env_helper;
pub mod error;
pub mod logic;
pub mod macros;
pub mod redis;
pub mod render_task;
pub mod routes;
pub mod status_types;
pub mod util;

pub fn make_http_server(
    pool: Pool<Client>,
    bucket: Bucket,
    sender: UnboundedSender<Message>,
) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello from rusvid server" }))
        .route("/health", any(|| async { StatusCode::OK }))
        .nest("/status", routes::status::router(pool.clone()))
        .nest(
            "/video",
            routes::video::router(sender, bucket, pool.clone()),
        )
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
        .fallback(|| async { ApiError::NotFound })
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

pub fn make_metrics_server() -> Router {
    let recorder_handle = setup_metrics_recorder();

    Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
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
