use std::fmt::Debug;

mod third_party;

use serde::{Deserialize, Serialize};
pub use third_party::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
