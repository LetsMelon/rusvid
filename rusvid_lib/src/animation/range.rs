#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum RangeType {
    /// similar to `start..end`
    Exclusive,

    /// similar to `start..=end`
    Inclusive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Range {
    start: usize,
    end: usize,

    typ: RangeType,
}

impl Range {
    pub fn new(start: usize, end: usize, typ: RangeType) -> Self {
        Range { start, end, typ }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_std_range() {
        assert_eq!(
            Range::from(0..100),
            Range::new(0, 100, RangeType::Exclusive)
        );
        assert_eq!(
            (0..100).start,
            Range::new(0, 100, RangeType::Exclusive).start()
        );
        assert_eq!(
            (0..100).end,
            Range::new(0, 100, RangeType::Exclusive).end_bound()
        );

        assert_eq!(
            Range::from(0..=100),
            Range::new(0, 100, RangeType::Inclusive)
        );
        assert_eq!(
            *(0..=100).start(),
            Range::new(0, 100, RangeType::Inclusive).start()
        );
        assert_eq!(
            *(0..=100).end(),
            Range::new(0, 100, RangeType::Inclusive).end_bound()
        );

        assert_eq!(
            Range::from((0, 100)),
            Range::new(0, 100, RangeType::Exclusive)
        );
        assert_eq!(0, Range::new(0, 100, RangeType::Exclusive).start());
        assert_eq!(101, Range::new(0, 100, RangeType::Exclusive).end_bound());
    }
}
