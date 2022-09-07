use anyhow::Result;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub mod linear;
pub mod s;

#[inline(always)]
pub(crate) fn has_update_function(
    start_frame: usize,
    end_frame: usize,
    frame_number: &usize,
) -> bool {
    *frame_number >= start_frame && *frame_number <= end_frame
}

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
pub trait Function: std::fmt::Debug {
    fn new(start_frame: usize, end_frame: usize, start: Points, end: Points) -> Result<Self>
    where
        Self: Sized;

    fn start_frame(&self) -> usize;
    fn end_frame(&self) -> usize;
    fn start(&self) -> Points;
    fn end(&self) -> Points;

    fn has_update(&self, frame_number: &usize) -> bool {
        has_update_function(self.start_frame(), self.end_frame(), frame_number)
    }

    fn calc_raw(&self, frame_number: usize) -> Points;
    fn calc(&self, frame_number: usize) -> Points {
        if frame_number <= self.start_frame() {
            return self.start();
        } else if frame_number >= self.end_frame() {
            return self.end();
        }
        self.calc_raw(frame_number)
    }

    /// Raw instantaneous rate of change at the point `frame_number`
    fn delta_raw(&self, frame_number: usize) -> Points;
    fn delta(&self, frame_number: usize) -> Points {
        if frame_number <= self.start_frame() || frame_number > self.end_frame() {
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

    pub fn pow(self, exp: Points) -> Points {
        let x = if self.x() == 0.0 || exp.x() == 0.0 {
            0.0
        } else {
            self.x().powf(exp.x())
        };
        let y = if self.y() == 0.0 || exp.y() == 0.0 {
            0.0
        } else {
            self.y().powf(exp.y())
        };
        Points::Point2d(x, y)
    }

    pub fn zero_2d() -> Self {
        Points::Point2d(0.0, 0.0)
    }

    pub fn one_2d() -> Self {
        Points::Point2d(1.0, 1.0)
    }

    pub fn two_2d() -> Self {
        Points::Point2d(2.0, 2.0)
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
            (Points::Point1d(x1), Points::Point1d(x2)) => {
                if x1 == 0.0 || x2 == 0.0 {
                    Points::Point2d(0.0, 0.0)
                } else {
                    Points::Point2d(x1 / x2, 0.0)
                }
            }
            (Points::Point2d(x1, y1), Points::Point2d(x2, y2)) => {
                let mut x = 0.0;
                let mut y = 0.0;

                if x1 != 0.0 && x2 != 0.0 {
                    x = x1 / x2;
                }
                if y1 != 0.0 && y2 != 0.0 {
                    y = y1 / y2;
                }

                Points::Point2d(x, y)
            }
            (Points::Point1d(x1), Points::Point2d(x2, y2)) => {
                if x1 == 0.0 || x2 == 0.0 {
                    Points::Point2d(0.0, y2)
                } else {
                    Points::Point2d(x1 / x2, y2)
                }
            }
            (Points::Point2d(x1, y1), Points::Point1d(x2)) => {
                if x1 == 0.0 || x2 == 0.0 {
                    Points::Point2d(0.0, y1)
                } else {
                    Points::Point2d(x1 / x2, y1)
                }
            }
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
impl Neg for Points {
    type Output = Points;

    fn neg(self) -> Self::Output {
        self * Points::Point2d(-1.0, -1.0)
    }
}

#[cfg(test)]
mod tests {
    mod function {
        use anyhow::Result;

        use crate::animation::curves::{Function, Points};

        #[derive(Debug)]
        struct TestFunction {
            start_frame: usize,
            end_frame: usize,
        }

        impl Function for TestFunction {
            fn new(
                start_frame: usize,
                end_frame: usize,
                _start: Points,
                _end: Points,
            ) -> Result<Self>
            where
                Self: Sized,
            {
                Ok(TestFunction {
                    start_frame,
                    end_frame,
                })
            }

            fn start_frame(&self) -> usize {
                self.start_frame
            }

            fn end_frame(&self) -> usize {
                self.end_frame
            }

            fn start(&self) -> crate::prelude::animation::Points {
                todo!()
            }

            fn end(&self) -> crate::prelude::animation::Points {
                todo!()
            }

            fn calc_raw(&self, _frame_number: usize) -> crate::prelude::animation::Points {
                todo!()
            }

            fn delta_raw(&self, _frame_number: usize) -> crate::prelude::animation::Points {
                todo!()
            }

            fn internal_debug(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                todo!()
            }
        }

        mod has_update {
            use super::*;

            #[test]
            fn is_only_true_if_updated() {
                let item = TestFunction::new(10, 20, (0.0, 0.0).into(), (0.0, 0.0).into()).unwrap();

                assert!(!item.has_update(&0));
                assert!(!item.has_update(&50));

                assert!(item.has_update(&10));
                assert!(item.has_update(&15));
                assert!(item.has_update(&20));
            }
        }
    }

    mod points {
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
            assert_eq!(Point1d(100.0) / Point2d(50.0, 20.0), Point2d(2.0, 20.0));
        }

        #[test]
        fn mul() {
            assert_eq!(Point1d(100.0) * Point1d(0.0), Point1d(0.0));
            assert_eq!(Point1d(100.0) * Point1d(1.0), Point1d(100.0));
            assert_eq!(Point2d(200.0, 50.0) * Point1d(0.0), Point2d(0.0, 50.0));
        }
    }
}
