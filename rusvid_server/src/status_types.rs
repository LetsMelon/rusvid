use std::collections::HashMap;

use num_derive::FromPrimitive;
use r2d2_redis::redis::{FromRedisValue, ToRedisArgs};
// use redis::{ErrorKind, FromRedisValue, RedisError, ToRedisArgs};
use serde::Serialize;

#[derive(Debug, Default, Clone, Copy, Serialize, PartialEq, FromPrimitive)]
pub enum ItemStatus {
    #[default]
    Pending = 1,
    Processing,
    Finish,
    InDeletion,
}

impl ToRedisArgs for ItemStatus {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + r2d2_redis::redis::RedisWrite,
    {
        let number = *self as usize;

        number.write_redis_args(out)
    }
}

impl FromRedisValue for ItemStatus {
    fn from_redis_value(v: &r2d2_redis::redis::Value) -> r2d2_redis::redis::RedisResult<Self> {
        let number = usize::from_redis_value(v)?;

        let element =
            num::FromPrimitive::from_usize(number).ok_or(r2d2_redis::redis::RedisError::from((
                r2d2_redis::redis::ErrorKind::TypeError,
                "Serialization Error with num",
            )))?;

        Ok(element)
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ItemList {
    pub list: HashMap<String, ItemStatus>,
}
