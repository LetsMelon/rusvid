use rusvid_core::holder::likes::ColorLike;
use serde::{Deserialize, Serialize};

use super::Animation;

#[derive(Debug, Serialize, Deserialize)]
pub struct SetColorAnimation {
    object_id: String,

    frame: usize,

    color_like: Option<ColorLike>,
}

impl SetColorAnimation {
    pub fn new<I: Into<String> + Clone>(id: &I, frame: usize, color: Option<ColorLike>) -> Self {
        Self {
            object_id: id.clone().into(),
            frame,
            color_like: color,
        }
    }
}

impl Animation for SetColorAnimation {
    fn object_id(&self) -> &str {
        &self.object_id
    }

    type OUTPUT = Option<ColorLike>;
    fn get_value(&self, _: usize) -> Self::OUTPUT {
        self.color_like.clone()
    }

    fn start_frame(&self) -> usize {
        self.frame
    }

    fn end_frame(&self) -> usize {
        self.frame
    }
}
