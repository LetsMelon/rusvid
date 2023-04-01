use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

use crate::{ItemList, SharedItemList};

pub async fn list_all_items(shared_list: SharedItemList) -> Json<ItemList> {
    let items = shared_list.read().unwrap().clone();

    Json(items)
}

pub async fn single_status(
    Path(id): Path<String>,
    shared_list: SharedItemList,
) -> Result<Json<Value>, StatusCode> {
    let item = shared_list.read().unwrap().list.get(&id).cloned();

    match item {
        Some(status) => Ok(Json(json!({ "id": id, "status": status}))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
