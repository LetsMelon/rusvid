use anyhow::bail;

use std::fmt::{Debug, Formatter};

use crate::animation::curves::Points::Point2d;
use crate::animation::curves::{Function, Points};
use crate::utils::map;

#[derive(Debug, Copy, Clone)]
/// Formula:
/// `y = 1 / (1 + (x / (1 - x))**(-n))`
// Source: https://stats.stackexchange.com/a/289477
pub struct S {
    start_frame: usize,
    end_frame: usize,
    start: Points,
    end: Points,

    // Always Point2d
    b: Points,
}

impl S {
    #[inline]
    fn derivative(x: f64, b: f64) -> f64 {
        if x <= 0.0 || x >= 1.0 {
            return 0.0;
        }

        let inner = (-(x / (x - 1.0))).powf(b);

        let upper = b * inner;
        let lower = x * (x - 1.0) * (inner + 1.0).powf(2.0);

        -(upper / lower)
    }
}

impl Function for S {
    fn new(start_frame: usize, end_frame: usize, start: Points, end: Points) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        if start_frame > end_frame {
            bail!("`start_frame` has to be smaller than `end_frame`");
        }

        Ok(S {
            start_frame,
            end_frame,
            start,
            end,
            b: Point2d(2.0, 2.0),
        })
    }

    #[inline]
    fn start_frame(&self) -> usize {
        self.start_frame
    }

    #[inline]
    fn end_frame(&self) -> usize {
        self.end_frame
    }

    #[inline]
    fn start(&self) -> Points {
        self.start
    }

    #[inline]
    fn end(&self) -> Points {
        self.end
    }

    fn calc_raw(&self, frame_number: usize) -> Points {
        let frame_number = map(
            frame_number as f64,
            self.start_frame as f64,
            self.end_frame as f64,
            0.0,
            1.0,
        );
        let frame_number_point = Point2d(frame_number, frame_number);

        let diff = self.end - self.start;

        let upper = Point2d(1.0, 1.0);
        let lower = Point2d(1.0, 1.0)
            + (frame_number_point / (Point2d(1.0, 1.0) - frame_number_point)).pow(-self.b);
        let percentage = upper / lower;

        self.start + diff * percentage
    }

    fn delta_raw(&self, frame_number: usize) -> Points {
        /*
        let frame_number = map(
            frame_number as f64,
            self.start_frame as f64,
            self.end_frame as f64,
            0.0,
            1.0,
        );

        let x = Point2d(frame_number, frame_number);

        let term = (x / (Points::one_2d() - x)).pow(self.b);

        let upper = self.b * term;
        let lower = (x - Points::one_2d()) * x * (term + Points::one_2d()).pow(Points::two_2d());

        let diff = self.end - self.start;

        (-(upper / lower)) * diff
         */
        let x = map(
            frame_number as f64,
            self.start_frame as f64,
            self.end_frame as f64,
            0.0,
            1.0,
        );

        let f1 = Self::derivative(x, self.b.x());
        let f2 = Self::derivative(x, self.b.y());

        let diff = self.end - self.start;
        let out = Points::Point2d(f1, f2) * diff;

        let mut sum = Points::zero_2d();
        for item in (self.start_frame + 1)..self.end_frame {
            let x = map(
                item as f64,
                self.start_frame as f64,
                self.end_frame as f64,
                0.0,
                1.0,
            );

            let ff1 = Self::derivative(x, self.b.x());
            let ff2 = Self::derivative(x, self.b.y());

            sum = sum + (Points::Point2d(ff1, ff2) * diff);
        }

        Points::Point2d(
            map(out.x(), 0.0, sum.x(), 0.0, diff.x()),
            map(out.y(), 0.0, sum.y(), 0.0, diff.y()),
        )
    }

    fn internal_debug(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::S;

    mod calc {
        use super::S;
        use crate::animation::curves::Function;
        use crate::animation::curves::Points::*;

        #[test]
        fn has_boundaries() {
            let s = S::new(30, 90, Point2d(0.0, 0.0), Point2d(10.0, 50.0)).unwrap();

            assert_eq!(s.calc(30), Point2d(0.0, 0.0));
            assert_eq!(s.calc(60), Point2d(5.0, 25.0));
            assert_eq!(s.calc(90), Point2d(10.0, 50.0));

            let s = S::new(0, 90, Point2d(800.0, 2000.0), Point2d(0.0, 0.0)).unwrap();

            assert_eq!(s.calc(0), Point2d(800.0, 2000.0));
            assert_eq!(s.calc(45), Point2d(400.0, 1000.0));
            assert_eq!(s.calc(90), Point2d(0.0, 0.0));
        }
    }

    mod derivative {
        use crate::animation::curves::s::S;

        #[test]
        fn is_b() {
            assert_eq!(S::derivative(0.5, 2.0), 2.0);
            assert_eq!(S::derivative(0.5, 5.0), 5.0);
            assert_eq!(S::derivative(0.5, 1.1), 1.1);
            assert_eq!(S::derivative(0.5, 0.75), 0.75);
        }

        #[test]
        fn has_boundaries() {
            assert_eq!(S::derivative(0.0, 2.0), 0.0);
            assert_eq!(S::derivative(1.0, 5.0), 0.0);
        }
    }

    /*
    mod delta_raw {
        use super::S;
        use crate::animation::curves::Function;
        use crate::animation::curves::Points::*;

        #[test]
        fn just_works() {
            let s = S::new(0, 100, Point2d(0.0, 0.0), Point2d(10.0, 50.0)).unwrap();

            assert_eq!(s.delta(0), Point2d(0.0, 0.0));
            assert_eq!(s.delta_raw(0), Point2d(-0.0, -0.0));
            assert!(s.delta(25).equal_delta(&Point2d(9.6, 48.0), 0.1));
            assert!(s.delta_raw(25).equal_delta(&Point2d(9.6, 48.0), 0.1));
            assert_eq!(s.delta(50), Point2d(20.0, 100.0));
            assert_eq!(s.delta_raw(50), Point2d(20.0, 100.0));
            assert_eq!(s.delta(75), Point2d(9.6, 48.0));
            assert_eq!(s.delta_raw(75), Point2d(9.6, 48.0));
            assert_eq!(s.delta(100), Point2d(0.0, 0.0));
            assert_eq!(s.delta_raw(100), Point2d(-0.0, -0.0));
        }
    }
     */
}
