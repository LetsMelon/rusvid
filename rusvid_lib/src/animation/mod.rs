pub mod change_color_animation;
pub mod curves;
pub mod position_animation;
pub mod set_color_animation;

mod range;

use self::change_color_animation::ChangeColorAnimation;
pub use self::curves::FunctionType;
use self::position_animation::PositionAnimation;
pub use self::range::{Range, RangeType};
use self::set_color_animation::SetColorAnimation;

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum AnimationType {
    Position(PositionAnimation),
    SetColor(SetColorAnimation),
    ChangeColor(ChangeColorAnimation),
}

impl AnimationType {
    pub fn check_variant(&self, other: &Self) -> bool {
        match (self, other) {
            (AnimationType::Position(_), AnimationType::Position(_)) => true,
            (AnimationType::SetColor(_), AnimationType::SetColor(_)) => true,
            (AnimationType::ChangeColor(_), AnimationType::ChangeColor(_)) => true,
            _ => false,
        }
    }
}

impl Animation for AnimationType {
    fn object_id(&self) -> &str {
        match self {
            AnimationType::Position(animation) => animation.object_id(),
            AnimationType::SetColor(animation) => animation.object_id(),
            AnimationType::ChangeColor(animation) => animation.object_id(),
        }
    }

    type OUTPUT = Result<(), String>;
    fn get_value(&self, _: usize) -> Self::OUTPUT {
        todo!()
    }

    fn start_frame(&self) -> usize {
        match self {
            AnimationType::Position(animation) => animation.start_frame(),
            AnimationType::SetColor(animation) => animation.start_frame(),
            AnimationType::ChangeColor(animation) => animation.start_frame(),
        }
    }

    fn end_frame(&self) -> usize {
        match self {
            AnimationType::Position(animation) => animation.end_frame(),
            AnimationType::SetColor(animation) => animation.end_frame(),
            AnimationType::ChangeColor(animation) => animation.end_frame(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
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

pub trait Animation: std::fmt::Debug {
    fn object_id(&self) -> &str;

    type OUTPUT;

    fn get_value(&self, frame: usize) -> Self::OUTPUT;

    /// Animation duration: [start_frame, end_frame)
    fn start_frame(&self) -> usize;
    /// Animation duration: [start_frame, end_frame)
    fn end_frame(&self) -> usize;

    /// Returns `true` if the animation hasn't started yet, otherwise `false`.
    fn status_pending(&self, frame_count: usize) -> bool {
        frame_count < self.start_frame()
    }

    /// Returns `true` if the animation has finished, otherwise `false`.
    fn status_finish(&self, frame_count: usize) -> bool {
        frame_count >= self.end_frame()
    }

    /// Returns `true` if the animation is currently running, otherwise `false`.
    ///
    /// If [`self.status_pending`] returns `true` than this function returns `false`,
    /// if [`self.status_finish`] returns `true`than this functions returns `false`.
    /// otherwise it returns `true`.
    fn status_running(&self, frame_count: usize) -> bool {
        !(self.status_pending(frame_count) || self.status_finish(frame_count))
    }
}

pub trait Function: std::fmt::Debug {
    fn delta_ease_in(&self, delta: f32) -> f32;
    fn delta_ease_out(&self, delta: f32) -> f32;
    fn delta_ease_in_out(&self, delta: f32) -> f32;
}
