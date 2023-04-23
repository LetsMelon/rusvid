pub mod change_color_animation;
pub mod curves;
pub mod position_animation;
pub mod set_color_animation;

use self::change_color_animation::ChangeColorAnimation;
pub use self::curves::FunctionType;
use self::position_animation::PositionAnimation;
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

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum RangeType {
    /// similar to `start..end`
    Exclusive,

    /// similar to `start..=end`
    Inclusive,
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Range {
    start: usize,
    end: usize,

    typ: RangeType,
}

impl Range {
    pub fn new(start: usize, end: usize, r#type: RangeType) -> Self {
        Range {
            start,
            end,
            typ: r#type,
        }
    }

    pub fn as_std_range(&self) -> std::ops::Range<usize> {
        match self.typ {
            RangeType::Exclusive => self.start..self.end,
            RangeType::Inclusive => self.start..(self.end + 1),
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    /// Frame end inclusive
    /// ```rust
    /// use rusvid_lib::animation::{Range, RangeType};
    ///
    /// assert!((0..100).contains(&99));
    /// assert!(!(0..100).contains(&100));
    ///
    /// assert!(Range::new(0, 100, RangeType::Exclusive).contains(99));
    /// assert!(!Range::new(0, 100, RangeType::Exclusive).contains(100));
    /// ```
    pub fn end_bound(&self) -> usize {
        self.as_std_range().end
    }

    /// ```rust
    /// use rusvid_lib::animation::{Range, RangeType};
    ///
    /// assert_eq!((0..100).contains(&99), true);
    /// assert_eq!((0..100).contains(&100), false);
    ///
    /// assert_eq!(Range::new(0, 100, RangeType::Exclusive).contains(99), true);
    /// assert_eq!(Range::new(0, 100, RangeType::Exclusive).contains(100), false);
    /// ```
    pub fn contains(&self, value: usize) -> bool {
        self.as_std_range().contains(&value)
    }

    pub fn len(&self) -> usize {
        self.as_std_range().len()
    }

    pub fn percentage(&self, frame: usize) -> f32 {
        let frame_delta = (self.len() - 1) as f32;
        let current = (frame - self.start()) as f32;

        current / frame_delta
    }
}

impl From<std::ops::Range<usize>> for Range {
    fn from(value: std::ops::Range<usize>) -> Self {
        Range::new(value.start, value.end, RangeType::Exclusive)
    }
}

impl From<std::ops::RangeInclusive<usize>> for Range {
    fn from(value: std::ops::RangeInclusive<usize>) -> Self {
        Range::new(*value.start(), *value.end(), RangeType::Inclusive)
    }
}

impl From<(usize, usize)> for Range {
    fn from(value: (usize, usize)) -> Self {
        (value.0..value.1).into()
    }
}
