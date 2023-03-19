pub mod change_color_animation;
pub mod curves;
pub mod position_animation;
pub mod set_color_animation;

use self::change_color_animation::ChangeColorAnimation;
use self::position_animation::PositionAnimation;
use self::set_color_animation::SetColorAnimation;

#[derive(Debug)]
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

pub trait Animation: std::fmt::Debug {
    fn object_id(&self) -> &str;

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
    fn new() -> Self
    where
        Self: Sized,
    {
        Self::new_with_ease_type(Default::default())
    }

    fn new_with_ease_type(ease_type: crate::animation::curves::EaseType) -> Self
    where
        Self: Sized;

    fn get_ease_type(&self) -> &crate::animation::curves::EaseType;

    fn delta_ease_in(&self, delta: f32) -> f32;
    fn delta_ease_out(&self, delta: f32) -> f32;
    fn delta_ease_in_out(&self, delta: f32) -> f32;

    fn delta(&self, delta: f32) -> f32 {
        assert!(delta >= 0.0);
        assert!(delta <= 1.0);

        match self.get_ease_type() {
            crate::animation::curves::EaseType::In => self.delta_ease_in(delta),
            crate::animation::curves::EaseType::Out => self.delta_ease_out(delta),
            crate::animation::curves::EaseType::InOut => self.delta_ease_in_out(delta),
        }
    }
}

// TODO remove prelude
pub mod prelude {
    pub use super::curves::*;
    pub use super::position_animation::PositionAnimation;
    pub use super::{Animation, Function};
}
