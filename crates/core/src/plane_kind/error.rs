use thiserror::Error;

use crate::plane_kind::SIZE;

#[derive(Error, Debug)]
pub enum PlaneError {
    #[error("{0} must be greater than 0")]
    ValueGreaterZero(&'static str),

    #[error("width * height must equal data.len()")]
    ArrayCapacityError,

    #[error("width * height must smaller than {}", SIZE::MAX)]
    CapacityError,

    #[error("Error in crate 'image': {0:?}")]
    ImageError(#[from] image::ImageError),

    #[error("Error in crate 'tiny-skia'")]
    TinySkiaError,

    #[error("Can't get item at coordinates x: {0}, y: {1}")]
    OutOfBound2d(u32, u32),

    #[error("Error from 'std::io': '{0:?}'")]
    IoError(#[from] std::io::Error),

    #[error("Encoding error: {0:?}")]
    EncodingError(String),
}

impl PlaneError {
    pub fn same_variant(&self, other: &PlaneError) -> bool {
        match (self, other) {
            (PlaneError::ValueGreaterZero(_), PlaneError::ValueGreaterZero(_))
            | (PlaneError::ArrayCapacityError, PlaneError::ArrayCapacityError)
            | (PlaneError::CapacityError, PlaneError::CapacityError)
            | (PlaneError::ImageError(_), PlaneError::ImageError(_))
            | (PlaneError::TinySkiaError, PlaneError::TinySkiaError)
            | (PlaneError::OutOfBound2d(_, _), PlaneError::OutOfBound2d(_, _))
            | (PlaneError::IoError(_), PlaneError::IoError(_))
            | (PlaneError::EncodingError(_), PlaneError::EncodingError(_)) => true,
            _ => false,
        }
    }
}

impl PartialEq for PlaneError {
    fn eq(&self, other: &Self) -> bool {
        self.same_variant(other)
    }
}

impl Eq for PlaneError {}

impl From<png::EncodingError> for PlaneError {
    fn from(value: png::EncodingError) -> Self {
        PlaneError::EncodingError(format!("{:?}", value))
    }
}
