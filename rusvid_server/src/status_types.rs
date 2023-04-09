use std::collections::HashMap;

pub use rusvid_core::server::ItemStatus;
// use redis::{ErrorKind, FromRedisValue, RedisError, ToRedisArgs};
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]
pub struct ItemList {
    pub list: HashMap<String, ItemStatus>,
}
