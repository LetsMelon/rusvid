use rusvid_core::holder::likes::ColorLike;
use rusvid_core::prelude::Pixel;

use super::{Animation, EaseType, FunctionType, Range};

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ChangeColorAnimation {
    object_id: String,

    curve: FunctionType,
    ease: EaseType,

    frame_range: Range,

    start_color: Pixel,
    end_color: Pixel,
}

impl ChangeColorAnimation {
    pub fn new<I: Into<String> + Clone>(
        id: &I,
        frames: impl Into<Range>,
        colors: (Pixel, Pixel),
        curve: FunctionType,
        ease: EaseType,
    ) -> Self {
        Self {
            curve,
            ease,
            object_id: id.clone().into(),
            frame_range: frames.into(),
            start_color: colors.0,
            end_color: colors.1,
        }
    }
}

impl Animation for ChangeColorAnimation {
    fn object_id(&self) -> &str {
        &self.object_id
    }

    type OUTPUT = ColorLike;
    fn get_value(&self, frame: usize) -> Self::OUTPUT {
        let percentage = self.frame_range.percentage(frame);

        let delta = self
            .end_color
            .to_raw_float()
            .zip(self.start_color.to_raw_float())
            .map(|(end_color, start_color)| {
                start_color + (end_color - start_color) * (self.curve.delta(self.ease, percentage))
            })
            .map(|delta| delta as u8)
            .collect::<Vec<_>>();

        ColorLike::Color(Pixel::new(delta[0], delta[1], delta[2], delta[3]))
    }

    fn start_frame(&self) -> usize {
        self.frame_range.start()
    }

    fn end_frame(&self) -> usize {
        self.frame_range.end_bound()
    }
}
