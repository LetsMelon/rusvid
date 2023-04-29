use rusvid_core::prelude::Point;

use super::{Animation, EaseType, FunctionType, Range};

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PositionAnimation {
    object_id: String,

    curve: FunctionType,
    ease: EaseType,

    frame_range: Range,

    start_position: Point,
    end_position: Point,
}

impl PositionAnimation {
    pub fn new<I: Into<String> + Clone>(
        id: &I,
        frames: impl Into<Range>,
        positions: (Point, Point),
        curve: FunctionType,
        ease: EaseType,
    ) -> Self {
        Self {
            curve,
            ease,
            object_id: id.clone().into(),
            frame_range: frames.into(),
            start_position: positions.0,
            end_position: positions.1,
        }
    }
}

impl Animation for PositionAnimation {
    fn object_id(&self) -> &str {
        &self.object_id
    }

    type OUTPUT = Point;
    fn get_value(&self, frame: usize) -> Self::OUTPUT {
        let percentage = self.frame_range.percentage(frame);

        let distance_delta = self.end_position - self.start_position;

        self.start_position + distance_delta * (self.curve.delta(self.ease, percentage) as f64)
    }

    fn start_frame(&self) -> usize {
        self.frame_range.start()
    }

    fn end_frame(&self) -> usize {
        self.frame_range.end_bound()
    }
}
