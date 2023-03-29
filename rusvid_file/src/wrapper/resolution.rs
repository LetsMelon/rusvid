use serde::{Deserialize, Serialize};

use super::TranslateIntoRusvidGeneric;

#[derive(Deserialize, Serialize, Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Resolution {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

impl TranslateIntoRusvidGeneric for Resolution {
    type OUTPUT = rusvid_lib::resolution::Resolution;

    fn translate(&self) -> Self::OUTPUT {
        rusvid_lib::resolution::Resolution::Custom(self.width, self.height)
    }
}
