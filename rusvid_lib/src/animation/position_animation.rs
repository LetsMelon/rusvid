use rusvid_core::prelude::Point;

use super::{Animation, Function};

#[derive(Debug)]
pub struct PositionAnimation {
    curve: Box<dyn Function>,
    object_id: String,

    /// Range `[start_frame, end_frame)`
    start_frame: usize,
    /// Range `[start_frame, end_frame)`
    end_frame: usize,

    start_position: Point,
    end_position: Point,
}

impl PositionAnimation {
    pub fn new<T: Function + 'static, I: Into<String> + Clone>(
        id: &I,
        frames: (usize, usize),
        positions: (Point, Point),
        curve: T,
    ) -> Self {
        Self {
            curve: Box::new(curve),
            object_id: id.clone().into(),
            start_frame: frames.0,
            end_frame: frames.1,
            start_position: positions.0,
            end_position: positions.1,
        }
    }

    pub fn position(&self, frame: usize) -> Point {
        let frame_delta = (self.end_frame() - self.start_frame() - 1) as f32;
        let my_frame = (frame - self.start_frame()) as f32;

        let percentage = my_frame / frame_delta;

        println!("\t\t{frame}: {percentage:.3}");

        let distance_delta = self.end_position - self.start_position;

        self.start_position + distance_delta * (self.curve.delta(percentage) as f64)
    }
}

impl Animation for PositionAnimation {
    fn object_id(&self) -> &str {
        &self.object_id
    }

    fn start_frame(&self) -> usize {
        self.start_frame
    }

    fn end_frame(&self) -> usize {
        self.end_frame
    }
}
