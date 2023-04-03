use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use axum::Router;
use s3::Bucket;
use tokio::sync::mpsc::UnboundedSender;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::limit::RequestBodyLimitLayer;

use crate::logic::video;
use crate::render_task::Message;
use crate::status_types::SharedItemList;

// 8MB
const UPLOAD_LIMIT: usize = 8 * 1024 * 1024;

pub fn router(tx: UnboundedSender<Message>, shared_list: SharedItemList, bucket: Bucket) -> Router {
    Router::new()
        .route(
            "/upload",
            post({
                let shared_state = tx.clone();
                let cloned_shared_list = shared_list.clone();

                move |multipart| video::upload_video(multipart, shared_state, cloned_shared_list)
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
                let cloned_shared_list = shared_list.clone();
                let cloned_bucket = bucket.clone();

                move |path| video::download_video(path, cloned_shared_list, cloned_bucket)
            })
            .layer(CompressionLayer::new())
            .delete({
                let cloned_shared_list = shared_list.clone();
                let cloned_bucket = bucket.clone();

                move |path| video::delete_video(path, cloned_shared_list, cloned_bucket)
            }),
        )
}
