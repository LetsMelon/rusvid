use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use ::redis::Client;
use opentelemetry_otlp::WithExportConfig;
use r2d2::Pool;
use rusvid_server::{env_helper, make_http_server, make_metrics_server, render_task};
use s3::creds::Credentials;
use s3::{Bucket, Region};
use tokio::sync::mpsc;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(env_helper::exporter_endpoint()),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .unwrap();
    let telemetry_layer = tracing_opentelemetry::layer().with_tracer(tracer);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rusvid_server=debug,tower_http=debug,rusvid_lib=debug,rusvid_core=debug,rusvid_video_encoder=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .with(telemetry_layer)
        .init();

    let api_port = env_helper::api_port();
    let metrics_port = env_helper::metrics_port();

    let (_main_server, _metrics_server) = tokio::join!(
        start_main_server(api_port),
        start_metrics_server(metrics_port)
    );
}

async fn start_main_server(port: u16) {
    let access_key = env_helper::s3_access_key();
    let secret_key = env_helper::s3_secret_key();
    let redis_url = format!("redis://{}/", env_helper::redis_url());

    let client = Client::open(redis_url).unwrap();
    let pool = Pool::builder().build(client).unwrap();

    let bucket = Bucket::new(
        &env_helper::s3_bucket(),
        Region::Custom {
            region: env_helper::s3_region(),
            endpoint: format!("http://{}", env_helper::s3_url()),
        },
        Credentials::new(Some(&access_key), Some(&secret_key), None, None, None).unwrap(),
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

    let app = make_http_server(pool, bucket, tx);

    let addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    info!("main server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn start_metrics_server(port: u16) {
    let app = make_metrics_server();

    let addr: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
    info!("metrics server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
