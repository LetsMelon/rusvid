use axum::routing::get;
use axum::Router;
use r2d2::Pool;
use redis::Client;

use crate::logic::status;

pub fn router(redis_pool: Pool<Client>) -> Router {
    Router::new()
        .route(
            "/all",
            get({
                let cloned_redis_pool = redis_pool.clone();

                move || status::list_all_items(cloned_redis_pool)
            }),
        )
        .route(
            "/id/:id",
            get({
                let cloned_redis_pool = redis_pool.clone();

                move |path| status::single_status(path, cloned_redis_pool)
            }),
        )
}
