use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::status_types::{ItemList, SharedItemList};

pub async fn list_all_items(shared_list: SharedItemList) -> Result<Json<ItemList>, ApiError> {
    match shared_list.read() {
        Ok(items) => Ok(Json(items.clone())),
        Err(_) => Err(ApiError::LockError),
    }
}

pub async fn single_status(
    Path(id): Path<String>,
    shared_list: SharedItemList,
) -> Result<Json<Value>, ApiError> {
    let item = match shared_list.read() {
        Ok(value) => value.list.get(&id).cloned(),
        Err(_) => return Err(ApiError::LockError),
    };

    match item {
        Some(status) => Ok(Json(json!({ "id": id, "status": status}))),
        None => Err(ApiError::NotFound),
    }
}
