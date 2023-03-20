use std::fmt::Debug;

mod third_party;

pub use third_party::*;

#[derive(Debug)]
pub enum EaseType {
    In,
    Out,
    InOut,
}

impl Default for EaseType {
    fn default() -> Self {
        EaseType::In
    }
}
