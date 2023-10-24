use core::fmt::Debug;

use rhai::{LexError, ParseError};
use rusvid_core::plane_kind::error::PlaneError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RhaiError {
    #[error("Error encountered when tokenizing the script text. {0:?}")]
    Lex(LexError),

    #[error("Error when parsing a script. {0:?}")]
    Parse(ParseError),
}

#[derive(Error, Debug)]
pub enum EffectError {
    #[error("error occurred in `Plane`: {0:?}")]
    Plane(#[from] PlaneError),

    #[error("error occurred in rhai: {0:?}")]
    Rhai(#[from] RhaiError),

    #[error("{message}: {value}")]
    SizeError { message: &'static str, value: u32 },
}
