use std::sync::Arc;

use axum::routing::get;
use axum::Router;

use crate::logic::status;
use crate::status_types::SharedItemList;

pub fn router(shared_list: SharedItemList) -> Router {
    Router::new()
        .route(
            "/all",
            get({
                let cloned_shared_list = Arc::clone(&shared_list);
                move || status::list_all_items(cloned_shared_list)
            }),
        )
        .route(
            "/id/:id",
            get({
                let cloned_shared_list = Arc::clone(&shared_list);

                move |path| status::single_status(path, cloned_shared_list)
            }),
        )
}
