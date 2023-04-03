use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;
use r2d2_redis::r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use s3::Bucket;
use tokio::sync::mpsc::UnboundedSender;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::limit::RequestBodyLimitLayer;

use crate::logic::video;
use crate::render_task::Message;

// 8MB
const UPLOAD_LIMIT: usize = 8 * 1024 * 1024;

pub fn router(
    tx: UnboundedSender<Message>,
    bucket: Bucket,
    redis_pool: Pool<RedisConnectionManager>,
) -> Router {
    Router::new()
        .route(
            "/upload",
            post({
                let shared_state = tx.clone();
                let cloned_redis_pool = redis_pool.clone();

                move |multipart| video::upload_video(multipart, shared_state, cloned_redis_pool)
            })
            .layer(
                ServiceBuilder::new()
                    .layer(DefaultBodyLimit::disable())
                    .layer(RequestBodyLimitLayer::new(UPLOAD_LIMIT)),
            ),
        )
        .route(
            "/id/:id",
            get({
                let cloned_bucket = bucket.clone();
                let cloned_redis_pool = redis_pool.clone();

                move |path| video::download_video(path, cloned_bucket, cloned_redis_pool)
            })
            .layer(CompressionLayer::new())
            .delete({
                let cloned_bucket = bucket.clone();
                let cloned_redis_pool = redis_pool.clone();

                move |path| video::delete_video(path, cloned_bucket, cloned_redis_pool)
            }),
        )
}
