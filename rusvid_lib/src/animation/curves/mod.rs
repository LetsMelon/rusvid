use std::fmt::Debug;

use anyhow::Result;
use rusvid_core::point::Point;

// #[rustfmt::skip]
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

/// ```rust
/// use rusvid_lib::animation::curves::Linear;
/// use rusvid_lib::animation::curves::Function;
/// use rusvid_lib::types::Point;
///
/// let linear = Linear::new(30, 90, Point::new(100.0, 100.0), Point::new(300.0, 300.0)).unwrap();
///
/// assert_eq!(linear.calc(30), Point::new(100.0, 100.0));
/// assert_eq!(linear.calc(60), Point::new(200.0, 200.0));
/// assert_eq!(linear.calc(90), Point::new(300.0, 300.0));
/// ```
pub trait Function: std::fmt::Debug {
    fn new(start_frame: usize, end_frame: usize, start: Point, end: Point) -> Result<Self>
    where
        Self: Sized;

    fn new_with_ease_type(
        start_frame: usize,
        end_frame: usize,
        start: Point,
        end: Point,
        ease_type: EaseType,
    ) -> Result<Self>
    where
        Self: Sized,
    {
        let mut obj = Self::new(start_frame, end_frame, start, end)?;
        obj.set_ease_type(ease_type);

        Ok(obj)
    }

    fn start_frame(&self) -> usize;
    fn end_frame(&self) -> usize;
    fn delta_frame(&self) -> usize;
    fn start(&self) -> Point;
    fn end(&self) -> Point;

    fn set_ease_type(&mut self, _ease_type: EaseType) {}

    fn calc_raw(&self, frame_number: usize) -> Point;
    fn calc(&self, frame_number: usize) -> Point {
        if frame_number <= self.start_frame() {
            return self.start();
        } else if frame_number >= self.end_frame() {
            return self.end();
        }
        self.calc_raw(frame_number)
    }

    /// Raw instantaneous rate of change at the point `frame_number`
    fn delta_raw(&self, frame_number: usize) -> Point;
    fn delta(&self, frame_number: usize) -> Point {
        if frame_number <= self.start_frame() || frame_number > self.end_frame() {
            return Point::default();
        }
        self.delta_raw(frame_number)
    }

    fn calc_ease_in(&self, frame_number: usize) -> Point;
    fn calc_ease_out(&self, frame_number: usize) -> Point;
    fn calc_ease_in_out(&self, frame_number: usize) -> Point;
}
