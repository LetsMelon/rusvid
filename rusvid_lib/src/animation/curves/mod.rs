use anyhow::Result;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Sub};

pub mod linear;

/// ```rust
/// use rusvid_lib::animation::curves::linear::Linear;
/// use rusvid_lib::animation::curves::Function;
/// use rusvid_lib::animation::curves::Points::Point1d;
///
/// let linear = Linear::new(30, 90, Point1d(100.0), Point1d(300.0)).unwrap();
///
/// assert_eq!(linear.calc(30), Point1d(100.0));
/// assert_eq!(linear.calc(60), Point1d(200.0));
/// assert_eq!(linear.calc(90), Point1d(300.0));
/// ```
pub trait Function {
    fn new(start_frame: usize, end_frame: usize, start: Points, end: Points) -> Result<Self>
    where
        Self: Sized;

    fn start_frame(&self) -> usize;
    fn end_frame(&self) -> usize;
    fn start(&self) -> Points;
    fn end(&self) -> Points;

    fn calc_raw(&self, frame_number: usize) -> Points;
    fn calc(&self, frame_number: usize) -> Points {
        if frame_number < self.start_frame() {
            return self.start();
        } else if frame_number > self.end_frame() {
            return self.end();
        }
        self.calc_raw(frame_number)
    }

    /// Raw instantaneous rate of change at the point `frame_number`
    fn delta_raw(&self, frame_number: usize) -> Points;
    fn delta(&self, frame_number: usize) -> Points {
        if frame_number < self.start_frame() {
            return Points::default();
        } else if frame_number > self.end_frame() {
            return Points::default();
        }
        self.delta_raw(frame_number)
    }

    fn internal_debug(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

#[derive(Debug, Clone, Copy)]
pub enum Points {
    Point1d(f64),
    Point2d(f64, f64),
}

impl Default for Points {
    fn default() -> Self {
        Points::Point2d(0.0, 0.0)
    }
}
impl PartialEq<Self> for Points {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Points::Point1d(x1), Points::Point1d(x2)) => x1 == x2,
            (Points::Point2d(x1, y1), Points::Point2d(x2, y2)) => x1 == x2 && y1 == y2,
            (Points::Point2d(x1, _), Points::Point1d(x2)) => x1 == x2,
            (Points::Point1d(x1), Points::Point2d(x2, _)) => x1 == x2,
        }
    }
}
impl Eq for Points {}

impl Points {
    pub fn equal_delta(&self, other: &Points, delta: f64) -> bool {
        use crate::utils::equal_delta as eq_d;

        match (*self, *other) {
            (Points::Point1d(x1), Points::Point1d(x2)) => eq_d(x1, x2, delta),
            (Points::Point2d(x1, y1), Points::Point2d(x2, y2)) => {
                eq_d(x1, x2, delta) && eq_d(y1, y2, delta)
            }
            (_, _) => false,
        }
    }

    pub fn x(&self) -> f64 {
        match *self {
            Points::Point1d(x) => x,
            Points::Point2d(x, _) => x,
        }
    }

    pub fn y(&self) -> f64 {
        match self {
            Points::Point1d(_) => 0.0,
            Points::Point2d(_, y) => *y,
        }
    }
}

impl From<(f64, f64)> for Points {
    fn from(value: (f64, f64)) -> Self {
        Points::Point2d(value.0, value.1)
    }
}

impl Add for Points {
    type Output = Points;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Points::Point1d(x1), Points::Point1d(x2)) => Points::Point1d(x1 + x2),
            (Points::Point2d(x1, y1), Points::Point2d(x2, y2)) => Points::Point2d(x1 + x2, y1 + y2),
            (Points::Point1d(x1), Points::Point2d(x2, y2)) => Points::Point2d(x1 + x2, y2),
            (Points::Point2d(x1, y1), Points::Point1d(x2)) => Points::Point2d(x1 + x2, y1),
        }
    }
}
impl Sub for Points {
    type Output = Points;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Points::Point1d(x1), Points::Point1d(x2)) => Points::Point1d(x1 - x2),
            (Points::Point2d(x1, y1), Points::Point2d(x2, y2)) => Points::Point2d(x1 - x2, y1 - y2),
            (Points::Point1d(x1), Points::Point2d(x2, y2)) => Points::Point2d(x1 - x2, y2),
            (Points::Point2d(x1, y1), Points::Point1d(x2)) => Points::Point2d(x1 - x2, y1),
        }
    }
}
impl Div for Points {
    type Output = Points;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Points::Point1d(x1), Points::Point1d(x2)) => Points::Point1d(x1 / x2),
            (Points::Point2d(x1, y1), Points::Point2d(x2, y2)) => Points::Point2d(x1 / x2, y1 / y2),
            (Points::Point1d(x1), Points::Point2d(x2, y2)) => Points::Point2d(x1 / x2, y2),
            (Points::Point2d(x1, y1), Points::Point1d(x2)) => Points::Point2d(x1 / x2, y1),
        }
    }
}
impl Mul for Points {
    type Output = Points;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Points::Point1d(x1), Points::Point1d(x2)) => Points::Point1d(x1 * x2),
            (Points::Point2d(x1, y1), Points::Point2d(x2, y2)) => Points::Point2d(x1 * x2, y1 * y2),
            (Points::Point1d(x1), Points::Point2d(x2, y2)) => Points::Point2d(x1 * x2, y2),
            (Points::Point2d(x1, y1), Points::Point1d(x2)) => Points::Point2d(x1 * x2, y1),
        }
    }
}

#[cfg(test)]
mod tests {
    mod Points {
        use crate::animation::curves::Points::*;

        #[test]
        fn add() {
            assert_eq!(Point1d(100.0) + Point1d(150.0), Point1d(250.0));
            assert_eq!(Point1d(100.0) + Point1d(-150.0), Point1d(-50.0));

            assert_eq!(
                Point2d(100.0, 0.0) + Point2d(150.0, 50.0),
                Point2d(250.0, 50.0)
            );
            assert_eq!(
                Point2d(100.0, -50.0) + Point2d(-150.0, 50.0),
                Point2d(-50.0, 0.0)
            );
        }

        #[test]
        fn sub() {
            assert_eq!(Point1d(100.0) - Point1d(50.0), Point1d(50.0));

            assert_eq!(
                Point2d(100.0, 500.0) - Point2d(50.0, 600.0),
                Point2d(50.0, -100.0)
            );
        }

        #[test]
        fn div() {
            todo!()
        }

        #[test]
        fn mul() {
            todo!()
        }
    }
}
