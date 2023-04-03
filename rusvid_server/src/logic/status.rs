use axum::extract::Path;
use axum::Json;
use r2d2_redis::r2d2::Pool;
use r2d2_redis::redis::{Commands, ConnectionLike, FromRedisValue};
use r2d2_redis::RedisConnectionManager;
use serde_json::{json, Value};

use crate::error::ApiError;
use crate::status_types::{ItemList, ItemStatus};

pub async fn list_all_items(
    redis_pool: Pool<RedisConnectionManager>,
) -> Result<Json<ItemList>, ApiError> {
    let mut connection = redis_pool.get().unwrap();
    let keys: Vec<String> = connection.keys("*").unwrap();

    let mut new_list = ItemList::default();

    let out = connection
        .req_command(r2d2_redis::redis::Cmd::new().arg("MGET").arg(keys.clone()))
        .unwrap();

    let key_parsed_values_pairs = out
        .as_sequence()
        .unwrap()
        .iter()
        .zip(keys)
        .map(|(value, key)| (key, ItemStatus::from_redis_value(value)));

    for (key, value) in key_parsed_values_pairs {
        if let Ok(value) = value {
            new_list.list.insert(key, value);
        } else {
            println!("error with key: {key}, value: {value:?}");
        }
    }

    Ok(Json(new_list))
}

pub async fn single_status(
    Path(id): Path<String>,
    redis_pool: Pool<RedisConnectionManager>,
) -> Result<Json<Value>, ApiError> {
    let mut connection = redis_pool.get().unwrap();

    let item: Option<ItemStatus> = connection.get(&id).unwrap();

    match item {
        Some(status) => Ok(Json(json!({ "id": id, "status": status}))),
        None => Err(ApiError::NotFound),
    }
}
