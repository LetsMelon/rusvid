use num_derive::FromPrimitive;
use r2d2_redis::redis::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

// TODO add 'Errored(err?)' state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, FromPrimitive)]
pub enum ItemStatus {
    Pending = 1,
    Processing,
    Finish,
    InDeletion,
}

impl Default for ItemStatus {
    fn default() -> Self {
        ItemStatus::Pending
    }
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ItemStatusResponse {
    pub id: String,
    pub status: ItemStatus,
}
