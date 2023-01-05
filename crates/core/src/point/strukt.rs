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

    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self::from_dvec(DVec2::new(x, y))
    }

    #[inline]
    pub const fn new_symmetric(v: f64) -> Self {
        Self::new(v, v)
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

    #[inline]
    pub fn x_mut(&mut self) -> &mut f64 {
        &mut self.0.x
    }

    #[inline]
    pub fn y_mut(&mut self) -> &mut f64 {
        &mut self.0.y
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

    pub(crate) fn raw(&self) -> DVec2 {
        self.0
    }

    pub(crate) fn raw_mut(&mut self) -> &mut DVec2 {
        &mut self.0
    }

    pub(crate) fn set_raw(&mut self, value: DVec2) {
        self.0 = value
    }

    pub(crate) fn from_raw(value: DVec2) -> Point {
        Point(value)
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
