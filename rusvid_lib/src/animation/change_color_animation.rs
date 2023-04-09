use rusvid_core::holder::likes::ColorLike;
use rusvid_core::prelude::Pixel;

use super::{Animation, EaseType, FunctionType};

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ChangeColorAnimation {
    object_id: String,

    curve: FunctionType,
    ease: EaseType,

    /// Range `[start_frame, end_frame)`
    start_frame: usize,

    /// Range `[start_frame, end_frame)`
    end_frame: usize,

    start_color: Pixel,
    end_color: Pixel,
}

impl ChangeColorAnimation {
    pub fn new<I: Into<String> + Clone>(
        id: &I,
        frames: (usize, usize),
        colors: (Pixel, Pixel),
        curve: FunctionType,
        ease: EaseType,
    ) -> Self {
        Self {
            curve,
            ease,
            object_id: id.clone().into(),
            start_frame: frames.0,
            end_frame: frames.1,
            start_color: colors.0,
            end_color: colors.1,
        }
    }

    pub fn color_at_frame(&self, frame: usize) -> ColorLike {
        let frame_delta = (self.end_frame() - self.start_frame() - 1) as f32;
        let my_frame = (frame - self.start_frame()) as f32;

        let percentage = my_frame / frame_delta;

        let delta = self
            .end_color
            .to_raw()
            .iter()
            .map(|v| *v as f32)
            .zip(self.start_color.to_raw().iter().map(|v| *v as f32))
            .map(|(end_color, start_color)| {
                start_color + (end_color - start_color) * (self.curve.delta(self.ease, percentage))
            })
            .map(|delta| delta as u8)
            .collect::<Vec<_>>();

        ColorLike::Color(Pixel::new(delta[0], delta[1], delta[2], delta[3]))
    }
}

impl Animation for ChangeColorAnimation {
    fn object_id(&self) -> &str {
        &self.object_id
    }

    type OUTPUT = ColorLike;
    fn get_value(&self, frame: usize) -> Self::OUTPUT {
        let frame_delta = (self.end_frame() - self.start_frame() - 1) as f32;
        let my_frame = (frame - self.start_frame()) as f32;

        let percentage = my_frame / frame_delta;

        let delta = self
            .end_color
            .to_raw()
            .iter()
            .map(|v| *v as f32)
            .zip(self.start_color.to_raw().iter().map(|v| *v as f32))
            .map(|(end_color, start_color)| {
                start_color + (end_color - start_color) * (self.curve.delta(self.ease, percentage))
            })
            .map(|delta| delta as u8)
            .collect::<Vec<_>>();

        ColorLike::Color(Pixel::new(delta[0], delta[1], delta[2], delta[3]))
    }

    fn start_frame(&self) -> usize {
        self.start_frame
    }

    fn end_frame(&self) -> usize {
        self.end_frame
    }
}
