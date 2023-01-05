mod math;
mod strukt;

pub use math::*;
pub use strukt::Point;

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
