use axum::extract::Path;
use axum::Json;
use r2d2_redis::r2d2::Pool;
use r2d2_redis::redis::{Commands, ConnectionLike, FromRedisValue};
use r2d2_redis::RedisConnectionManager;
use rusvid_core::server::ItemStatusResponse;

use crate::error::ApiError;
use crate::redis::{key_for_video_status, video_status_prefix};
use crate::status_types::{ItemList, ItemStatus};

pub async fn list_all_items(
    redis_pool: Pool<RedisConnectionManager>,
) -> Result<Json<ItemList>, ApiError> {
    let mut connection = redis_pool.get()?;
    let keys: Vec<String> = connection.keys(format!("{}*", video_status_prefix()))?;

    let mut new_list = ItemList::default();

    let out =
        connection.req_command(r2d2_redis::redis::Cmd::new().arg("MGET").arg(keys.clone()))?;

    let key_parsed_values_pairs = out
        .as_sequence()
        // TODO prober error handling
        .ok_or(ApiError::UnknownError)?
        .iter()
        .zip(keys)
        .map(|(value, key)| (key, ItemStatus::from_redis_value(value)));

    for (key, value) in key_parsed_values_pairs {
        if let Ok(value) = value {
            let key = key.replace(video_status_prefix(), "");

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
) -> Result<Json<ItemStatusResponse>, ApiError> {
    let mut connection = redis_pool
        .get()
        .map_err(|err| ApiError::RedisR2D2Error(err))?;

    let item: Option<ItemStatus> = connection.get(key_for_video_status(&id))?;

    match item {
        Some(status) => {
            let response = ItemStatusResponse::new_with_status(id, status);

            Ok(Json(response))
        }
        None => Err(ApiError::NotFound),
    }
}
