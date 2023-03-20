use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use glam::DVec2;

/// A 2-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point(DVec2);

impl Point {
    /// All zeroes.
    pub const ZERO: Self = Self::new_symmetric(0.0);

    /// All ones.
    pub const ONE: Self = Self::new_symmetric(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::new_symmetric(-1.0);

    pub const fn new(x: f64, y: f64) -> Self {
        Self::from_dvec(DVec2::new(x, y))
    }

    pub const fn new_symmetric(v: f64) -> Self {
        Self::new(v, v)
    }

    const fn from_dvec(dvec: DVec2) -> Self {
        Point(dvec)
    }

    pub fn x(&self) -> f64 {
        self.0.x
    }

    pub fn y(&self) -> f64 {
        self.0.y
    }

    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.0.x
    }

    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.0.y
    }
}

impl From<(f64, f64)> for Point {
    fn from(raw: (f64, f64)) -> Self {
        Point::new(raw.0, raw.1)
    }
}

impl AsRef<[f64; 2]> for Point {
    fn as_ref(&self) -> &[f64; 2] {
        self.0.as_ref()
    }
}

impl AbsDiffEq for Point {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;

    #[cfg_attr(coverage_nightly, no_coverage)]
    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.as_ref().abs_diff_eq(other.as_ref(), epsilon)
    }
}

impl RelativeEq for Point {
    #[cfg_attr(coverage_nightly, no_coverage)]
    fn default_max_relative() -> Self::Epsilon {
        f64::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.as_ref()
            .relative_eq(other.as_ref(), epsilon, max_relative)
    }
}

impl UlpsEq for Point {
    #[cfg_attr(coverage_nightly, no_coverage)]
    fn default_max_ulps() -> u32 {
        f64::default_max_ulps()
    }

    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.as_ref().ulps_eq(other.as_ref(), epsilon, max_ulps)
    }
}

macro_rules! implement_math_operator {
    ($operant:ident, $fct:ident, Point for Point) => {
        impl std::ops::$operant<Point> for Point {
            type Output = Point;
            fn $fct(self, rhs: Self) -> Self::Output {
                Point(self.0.$fct(rhs.0))
            }
        }
    };
    ($operant:ident, $fct:ident, f32 for Point) => {
        impl std::ops::$operant<f32> for Point {
            type Output = Point;
            fn $fct(self, rhs: f32) -> Self::Output {
                Point(self.0.$fct(rhs))
            }
        }
    };
    ($operant:ident, $fct:ident, f64 for Point) => {
        impl std::ops::$operant<f64> for Point {
            type Output = Point;
            fn $fct(self, rhs: f64) -> Self::Output {
                Point(self.0.$fct(rhs))
            }
        }
    };
    ($operant:ident, $fct:ident, Point for f32) => {
        impl std::ops::$operant<Point> for f32 {
            type Output = Point;
            fn $fct(self, rhs: Self::Output) -> Self::Output {
                Point(DVec2 {
                    x: self.$fct(rhs.x()),
                    y: self.$fct(rhs.y()),
                })
            }
        }
    };
    ($operant:ident, $fct:ident, Point for f64) => {
        impl std::ops::$operant<Point> for f64 {
            type Output = Point;
            fn $fct(self, rhs: Self::Output) -> Self::Output {
                Point(DVec2 {
                    x: self.$fct(rhs.x()),
                    y: self.$fct(rhs.y()),
                })
            }
        }
    };
    ($operant:ident, $fct:ident, Point for assign $other_type:ident) => {
        concat_idents::concat_idents!(method_name = $fct, _assign {
            impl std::ops::$operant<$other_type> for Point {
                fn method_name(&mut self, other: $other_type) {
                    use std::ops::*;

                    *self = self.$fct(other);
                }
            }
        });
    };
}

implement_math_operator!(Add, add, f64 for Point);
implement_math_operator!(Add, add, Point for f64);
implement_math_operator!(Add, add, Point for Point);
implement_math_operator!(AddAssign, add, Point for assign f64);
implement_math_operator!(AddAssign, add, Point for assign Point);

implement_math_operator!(Div, div, f64 for Point);
implement_math_operator!(Div, div, Point for f64);
implement_math_operator!(Div, div, Point for Point);
implement_math_operator!(DivAssign, div, Point for assign f64);
implement_math_operator!(DivAssign, div, Point for assign Point);

implement_math_operator!(Mul, mul, f64 for Point);
implement_math_operator!(Mul, mul, Point for f64);
implement_math_operator!(Mul, mul, Point for Point);
implement_math_operator!(MulAssign, mul, Point for assign f64);
implement_math_operator!(MulAssign, mul, Point for assign Point);

implement_math_operator!(Rem, rem, f64 for Point);
implement_math_operator!(Rem, rem, Point for f64);
implement_math_operator!(Rem, rem, Point for Point);
implement_math_operator!(RemAssign, rem, Point for assign f64);
implement_math_operator!(RemAssign, rem, Point for assign Point);

implement_math_operator!(Sub, sub, f64 for Point);
implement_math_operator!(Sub, sub, Point for f64);
implement_math_operator!(Sub, sub, Point for Point);
implement_math_operator!(SubAssign, sub, Point for assign f64);
implement_math_operator!(SubAssign, sub, Point for assign Point);

/// Trait to transform struct into a [crate::types::Point]
pub trait AsPoint {
    /// Returns values of the struct as [crate::types::Point].
    ///
    /// Used to calculate with the values more easily
    /// ```rust
    /// use rusvid_core::point::{AsPoint, Point};
    ///
    /// struct Resolution((f64, f64));
    ///
    /// impl AsPoint for Resolution {
    ///     fn as_point(&self) -> Point {
    ///         Point::new(self.0.0, self.0.1)
    ///     }
    /// }
    ///
    /// let res = Resolution((100.0, 100.0));
    /// assert_eq!(res.as_point(), Point::new(100.0, 100.0));
    /// assert_eq!(res.as_point() * Point::NEG_ONE, Point::new(-100.0, -100.0));
    /// ```
    fn as_point(&self) -> Point;
}

#[cfg(test)]
mod tests {
    use super::Point;

    #[test]
    fn coordinate_mut() {
        let mut p = Point::new(25.0, 50.0);

        assert_eq!(p.x(), 25.0);
        *p.x_mut() = 75.0;
        assert_eq!(p.x(), 75.0);

        assert_eq!(p.y(), 50.0);
        *p.y_mut() = 25.0;
        assert_eq!(p.y(), 25.0);
    }

    #[test]
    fn from_tuple() {
        let tuple = (50.0, -10.0);
        let p = Point::from(tuple);

        assert_eq!(tuple.0, p.x());
        assert_eq!(tuple.1, p.y());
    }

    #[test]
    fn as_ref() {
        let p = Point::new(250.0, -300.0);
        let slice = p.as_ref();

        assert_eq!(p.x(), slice[0]);
        assert_eq!(p.y(), slice[1]);
    }

    mod approx {
        use approx::*;

        use crate::point::Point;

        #[test]
        fn abs_diff_eq() {
            let p1 = Point::new(100.0, 200.0);
            let p2 = Point::new(100.0001, 199.9999);
            let p3 = Point::new(75.0, 220.0);

            assert_eq!(p1.abs_diff_eq(&p2, 0.0), false);
            assert_eq!(p1.abs_diff_eq(&p3, 0.0), false);

            assert_eq!(p1.abs_diff_eq(&p2, 0.001), true);
            assert_eq!(p1.abs_diff_eq(&p3, 0.001), false);

            assert_eq!(p1.abs_diff_eq(&p2, 30.0), true);
            assert_eq!(p1.abs_diff_eq(&p3, 30.0), true);
        }

        #[test]
        fn relative_eq() {
            let p1 = Point::new(100.0, 200.0);
            let p2 = Point::new(100.0001, 199.9999);
            let p3 = Point::new(75.0, 220.0);

            assert_eq!(p1.relative_eq(&p2, 0.0, 0.0), false);
            assert_eq!(p1.relative_eq(&p3, 0.0, 0.0), false);

            assert_eq!(p1.relative_eq(&p2, 0.001, 0.0), true);
            assert_eq!(p1.relative_eq(&p3, 0.001, 0.0), false);

            assert_eq!(p1.relative_eq(&p3, 30.0, 0.0), true);
            assert_eq!(p1.relative_eq(&p2, 30.0, 0.0), true);
        }
    }

    mod math {
        use crate::point::Point;

        #[test]
        fn add() {
            let p1 = Point::new(25.0, 50.0);
            let p2 = Point::new_symmetric(10.0);

            assert_eq!(10.0 + p1, Point::new(35.0, 60.0));
            assert_eq!(p1 + 10.0, Point::new(35.0, 60.0));
            assert_eq!(p1 + p2, Point::new(35.0, 60.0));

            let mut p3 = Point::new(35.0, -10.0);
            p3 += 10.0;
            assert_eq!(p3, Point::new(45.0, 0.0));
            p3 += Point::new(-35.0, 100.0);
            assert_eq!(p3, Point::new(10.0, 100.0));
        }

        #[test]
        fn sub() {
            let p1 = Point::new(25.0, 50.0);
            let p2 = Point::new_symmetric(10.0);

            assert_eq!(10.0 - p1, Point::new(-15.0, -40.0));
            assert_eq!(p1 - 10.0, Point::new(15.0, 40.0));
            assert_eq!(p1 - p2, Point::new(15.0, 40.0));

            let mut p3 = Point::new(35.0, -10.0);
            p3 -= 10.0;
            assert_eq!(p3, Point::new(25.0, -20.0));
            p3 -= Point::new(-35.0, 100.0);
            assert_eq!(p3, Point::new(60.0, -120.0));
        }

        #[test]
        fn mul() {
            let p1 = Point::new(25.0, 50.0);
            let p2 = Point::new_symmetric(10.0);

            assert_eq!(10.0 * p1, Point::new(250.0, 500.0));
            assert_eq!(p1 * 10.0, Point::new(250.0, 500.0));
            assert_eq!(p1 * p2, Point::new(250.0, 500.0));

            let mut p3 = Point::new(35.0, -10.0);
            p3 *= 5.7;
            assert_eq!(p3, Point::new(199.5, -57.0));
            p3 *= Point::new(-35.0, 100.0);
            assert_eq!(p3, Point::new(-6982.5, -5700.0));
        }
    }
}
