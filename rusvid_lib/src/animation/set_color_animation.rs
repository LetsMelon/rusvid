use rusvid_core::holder::likes::ColorLike;

use super::Animation;

#[derive(Debug)]
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

    pub fn color_like(&self) -> &Option<ColorLike> {
        &self.color_like
    }
}

impl Animation for SetColorAnimation {
    fn object_id(&self) -> &str {
        &self.object_id
    }

    fn start_frame(&self) -> usize {
        self.frame
    }

    fn end_frame(&self) -> usize {
        self.frame
    }
}
