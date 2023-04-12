use std::mem::variant_count;

// TODO add 'Errored(err?)' state
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum ItemStatus {
    Pending = 1,
    Processing,
    Finish,
    InDeletion,
    EncounteredError,
}

impl ItemStatus {
    /// Returns `true` if the resource is ok otherwise the functions returns `false`.
    ///
    /// But for example this does not mean that the resource is finish rendered to be downloaded.
    pub fn is_ok(&self) -> bool {
        match self {
            ItemStatus::Pending | ItemStatus::Processing | ItemStatus::Finish => true,
            ItemStatus::InDeletion | ItemStatus::EncounteredError => false,
        }
    }

    /// Wrapper around [`.is_ok()`](ItemStatus::is_ok) but the output is negated.
    ///
    /// For more infos see [`.is_ok()`](ItemStatus::is_ok).
    pub fn is_not_ok(&self) -> bool {
        !self.is_ok()
    }

    /// Parses an `ItemStatus`-variant as an `u32`. Can be parsed back into an `ItemStatus` via [`.from_u32(value)`](ItemStatus::from_u32)
    ///
    /// ```rust
    /// let status = ItemStatus::Pending;
    /// assert_eq!(status.as_u32(), 1);
    /// ```
    #[inline(always)]
    pub fn as_u32(&self) -> u32 {
        *self as u32
    }

    /// Parses an u32 as an `Option<ItemStatus>`. Can be parsed back into an `u32` via [`.as_u32()`](ItemStatus::as_u32)
    ///
    /// ```rust
    /// let status = 2;
    /// assert_eq!(ItemStatus::from_u32(status), Some(ItemStatus::Processing));
    /// ```
    pub fn from_u32(value: u32) -> Option<Self> {
        const_assert_eq!(variant_count::<ItemStatus>(), 5);
        match value {
            1 => Some(ItemStatus::Pending),
            2 => Some(ItemStatus::Processing),
            3 => Some(ItemStatus::Finish),
            4 => Some(ItemStatus::InDeletion),
            5 => Some(ItemStatus::EncounteredError),
            _ => None,
        }
    }
}

impl Default for ItemStatus {
    fn default() -> Self {
        ItemStatus::Pending
    }
}

#[cfg(feature = "redis")]
impl redis::ToRedisArgs for ItemStatus {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        let number = self.as_u32();

        number.write_redis_args(out)
    }
}

#[cfg(feature = "redis")]
impl redis::FromRedisValue for ItemStatus {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        let number = u32::from_redis_value(v)?;

        ItemStatus::from_u32(number).ok_or(redis::RedisError::from((
            redis::ErrorKind::TypeError,
            "Serialization Error",
            format!("Number: '{number}'"),
        )))
    }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ItemStatusResponse {
    id: String,
    status: ItemStatus,
}

impl ItemStatusResponse {
    #[inline]
    pub fn new(id: String) -> Self {
        Self::new_with_status(id, ItemStatus::default())
    }

    #[inline]
    pub fn new_with_status(id: String, status: ItemStatus) -> Self {
        ItemStatusResponse { id, status }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn status(&self) -> ItemStatus {
        self.status
    }
}

#[cfg(test)]
mod tests {
    use crate::server::ItemStatus;

    #[test]
    fn as_and_from() {
        use strum::IntoEnumIterator;

        for status in ItemStatus::iter() {
            let as_u32 = status.as_u32();
            let from_u32 = ItemStatus::from_u32(as_u32);

            assert!(from_u32.is_some());
            assert_eq!(from_u32, Some(status));
        }
    }
}
