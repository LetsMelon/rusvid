use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use glam::DVec2;

/// A 2-dimensional vector.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point(DVec2);

impl Point {
    /// All zeroes.
    pub const ZERO: Self = Self::from_dvec(DVec2::splat(0.0));

    /// All ones.
    pub const ONE: Self = Self::from_dvec(DVec2::splat(1.0));

    /// All negative ones.
    pub const NEG_ONE: Self = Self::from_dvec(DVec2::splat(-1.0));

    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self::from_dvec(DVec2::new(x, y))
    }

    #[inline(always)]
    const fn from_dvec(dvec: DVec2) -> Self {
        Point(dvec)
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0.x
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.0.y
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two vectors contain similar elements. It works best when
    /// comparing with a known value. The `max_abs_diff` that should be used used depends on
    /// the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f64) -> bool {
        self.0.abs_diff_eq(rhs.0, max_abs_diff)
    }
}

impl From<(f64, f64)> for Point {
    #[inline]
    fn from(raw: (f64, f64)) -> Self {
        Point::new(raw.0, raw.1)
    }
}

impl AsRef<[f64; 2]> for Point {
    #[inline]
    fn as_ref(&self) -> &[f64; 2] {
        self.0.as_ref()
    }
}

impl AbsDiffEq for Point {
    type Epsilon = <f64 as AbsDiffEq>::Epsilon;
    fn default_epsilon() -> Self::Epsilon {
        f64::default_epsilon()
    }
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.as_ref().abs_diff_eq(other.as_ref(), epsilon)
    }
}

impl RelativeEq for Point {
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
    fn default_max_ulps() -> u32 {
        f64::default_max_ulps()
    }
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.as_ref().ulps_eq(other.as_ref(), epsilon, max_ulps)
    }
}

macro_rules! implement_math_operator {
    ($operant:ident, $fct:ident) => {
        impl std::ops::$operant<Point> for Point {
            type Output = Point;
            #[inline]
            fn $fct(self, rhs: Self) -> Self::Output {
                Point(self.0.$fct(rhs.0))
            }
        }
    };
}

implement_math_operator!(Div, div);
implement_math_operator!(Mul, mul);
implement_math_operator!(Add, add);
implement_math_operator!(Sub, sub);
implement_math_operator!(Rem, rem);

/*
impl DivAssign<DVec2> for DVec2 {...}
impl Div<f64> for DVec2 {...}
impl DivAssign<f64> for DVec2 {...}
impl Div<DVec2> for f64 {...}

impl MulAssign<DVec2> for DVec2 {...}
impl Mul<f64> for DVec2 {...}
impl MulAssign<f64> for DVec2 {...}
impl Mul<DVec2> for f64 {...}

impl AddAssign<DVec2> for DVec2 {...}
impl Add<f64> for DVec2 {...}
impl AddAssign<f64> for DVec2 {...}
impl Add<DVec2> for f64 {...}

impl SubAssign<DVec2> for DVec2 {...}
impl Sub<f64> for DVec2 {...}
impl SubAssign<f64> for DVec2 {...}
impl Sub<DVec2> for f64 {...}

impl RemAssign<DVec2> for DVec2 {...}
impl Rem<f64> for DVec2 {...}
impl RemAssign<f64> for DVec2 {...}
impl Rem<DVec2> for f64 {...}
 */

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
