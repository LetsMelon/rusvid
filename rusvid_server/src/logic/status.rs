use axum::extract::Path;
use axum::Json;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::status_types::{ItemList, SharedItemList};

pub async fn list_all_items(shared_list: SharedItemList) -> Result<Json<ItemList>, ApiError> {
    let items = shared_list.read()?.clone();

    Ok(Json(items))
}

pub async fn single_status(
    Path(id): Path<String>,
    shared_list: SharedItemList,
) -> Result<Json<Value>, ApiError> {
    let item = shared_list.read()?.list.get(&id).cloned();

    match item {
        Some(status) => Ok(Json(json!({ "id": id, "status": status}))),
        None => Err(ApiError::NotFound),
    }
}
