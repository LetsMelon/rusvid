use core::fmt::Debug;

use rusvid_core::plane::PlaneError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EffectError {
    #[error("error occurred in `Plane`: {0:?}")]
    Plane(#[from] PlaneError),

    #[error("{message}: {value:?}")]
    SizeError {
        message: &'static str,
        value: Box<dyn Debug>,
    },
}
