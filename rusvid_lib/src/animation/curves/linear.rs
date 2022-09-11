use anyhow::{bail, Result};
use std::fmt::{Debug, Formatter};

use crate::animation::curves::Points::*;
use crate::animation::curves::{Function, Points};

use super::has_update_function;

#[derive(Debug, Copy, Clone)]
pub struct Linear {
    start_frame: usize,
    end_frame: usize,
    start: Points,
    end: Points,

    // f(x) = kx+d
    k: Points,
    d: Points,
}

impl Function for Linear {
    fn new(start_frame: usize, end_frame: usize, start: Points, end: Points) -> Result<Self>
    where
        Self: Sized,
    {
        if start_frame > end_frame {
            bail!("`start_frame` has to be smaller than `end_frame`");
        }

        let start_frame_point = Point2d(start_frame as f64, start_frame as f64);
        let end_frame_point = Point2d(end_frame as f64, end_frame as f64);

        let (k, d) = if start == end && start == Points::zero_2d() {
            (Points::zero_2d(), Points::zero_2d())
        } else {
            let k = (end - start) / (end_frame_point - start_frame_point);
            let d = end - k * end_frame_point;

            (k, d)
        };

        Ok(Linear {
            start_frame,
            end_frame,
            start,
            end,
            d,
            k,
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

    fn has_update(&self, frame_number: &usize) -> bool {
        let linear_change = self.k == self.d && self.k == Points::zero_2d();
        if linear_change {
            return false;
        }

        has_update_function(self.start_frame(), self.end_frame(), frame_number)
    }

    #[inline]
    fn calc_raw(&self, frame_number: usize) -> Points {
        let frame_number = frame_number as f64;
        let frame_number_point = Point2d(frame_number, frame_number);
        self.k * frame_number_point + self.d
    }

    fn delta_raw(&self, _frame_number: usize) -> Points {
        self.k
    }

    fn internal_debug(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::Linear;
    use crate::animation::curves::Function;
    use crate::animation::curves::Points::*;

    const DELTA: f64 = 0.1;

    #[test]
    fn new() {
        let linear = Linear::new(30, 90, Point1d(100.0), Point1d(300.0));

        match linear {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn calc() {
        let linear = Linear::new(30, 90, Point2d(100.0, 100.0), Point2d(300.0, 300.0)).unwrap();

        assert_eq!(linear.calc(30), Point2d(100.0, 100.0));
        assert_eq!(linear.calc_raw(30), Point2d(100.0, 100.0));
        assert_eq!(linear.calc(60), Point2d(200.0, 200.0));
        assert_eq!(linear.calc_raw(60), Point2d(200.0, 200.0));
        assert_eq!(linear.calc(90), Point2d(300.0, 300.0));
        assert_eq!(linear.calc_raw(90), Point2d(300.0, 300.0));

        assert_eq!(linear.calc(20), Point2d(100.0, 100.0));
        assert!(linear
            .calc_raw(20)
            .equal_delta(&Point2d(66.66, 66.66), DELTA));
        assert_eq!(linear.calc(100), Point2d(300.0, 300.0));
        assert!(linear
            .calc_raw(100)
            .equal_delta(&Point2d(333.33, 333.33), DELTA));
    }

    #[test]
    fn delta() {
        let linear = Linear::new(30, 90, Point1d(100.0), Point1d(100.0)).unwrap();

        assert_eq!(linear.delta(30), Point1d(0.0));
        assert_eq!(linear.delta_raw(30), Point1d(0.0));
        assert_eq!(linear.delta(80), Point1d(0.0));
        assert_eq!(linear.delta_raw(80), Point1d(0.0));

        assert_eq!(linear.delta(20), Point1d(0.0));
        assert_eq!(linear.delta_raw(20), Point1d(0.0));
        assert_eq!(linear.delta(100), Point1d(0.0));
        assert_eq!(linear.delta_raw(100), Point1d(0.0));

        let linear = Linear::new(30, 90, Point2d(100.0, 100.0), Point2d(300.0, 500.0)).unwrap();

        assert!(linear.delta(30).equal_delta(&Point2d(0.0, 0.0), DELTA));
        assert!(linear
            .delta_raw(30)
            .equal_delta(&Point2d(3.33, 6.66), DELTA));
        assert!(linear.delta(80).equal_delta(&Point2d(3.33, 6.66), DELTA));
        assert!(linear
            .delta_raw(80)
            .equal_delta(&Point2d(3.33, 6.66), DELTA));

        assert_eq!(linear.delta(20), Point2d(0.0, 0.0));
        assert!(linear
            .delta_raw(20)
            .equal_delta(&Point2d(3.33, 6.66), DELTA));
        assert_eq!(linear.delta(100), Point2d(0.0, 0.0));
        assert!(linear
            .delta_raw(100)
            .equal_delta(&Point2d(3.33, 6.66), DELTA));
    }

    #[test]
    fn zero_error() {
        let linear = Linear::new(0, 10, Point1d(10.0), Point1d(20.0)).unwrap();

        assert_eq!(linear.calc(0), Point1d(10.0));
        assert_eq!(linear.calc(5), Point1d(15.0));
        assert_eq!(linear.calc(10), Point1d(20.0));
    }

    #[test]
    fn has_update() {
        let linear = Linear::new(30, 90, Point1d(0.0), Point1d(100.0)).unwrap();

        assert!(!linear.has_update(&0));
        assert!(linear.has_update(&30));
        assert!(linear.has_update(&90));
        assert!(!linear.has_update(&100));

        let linear = Linear::new(30, 90, Point1d(0.0), Point1d(0.0)).unwrap();

        assert!(!linear.has_update(&0));
        assert!(!linear.has_update(&30));
        assert!(!linear.has_update(&90));
        assert!(!linear.has_update(&100));
    }
}
