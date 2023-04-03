use axum::routing::get;
use axum::Router;
use r2d2_redis::r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

use crate::logic::status;

pub fn router(redis_pool: Pool<RedisConnectionManager>) -> Router {
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
