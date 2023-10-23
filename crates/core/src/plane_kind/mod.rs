use crate::plane_kind::error::PlaneError;

pub mod error;
pub mod plane;

pub type PlaneResult<T> = Result<T, PlaneError>;

/// Used as resolution and coordinates
pub type SIZE = u32;
