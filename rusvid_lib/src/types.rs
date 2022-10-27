use glam::DVec2;

/// Type for fps in frames per second
pub type FPS = u8;

/// A 2-dimensional vector.
pub type Point = DVec2;

pub type ResolutionType = (u32, u32);

/// Trait to transform struct into a [crate::types::Point]
pub trait AsPoint {
    /// Returns values of the struct as [crate::types::Point].
    ///
    /// Used to calculate with the values more easily
    /// ```rust
    /// use rusvid_lib::resolution::Resolution;
    /// use rusvid_lib::types::{AsPoint, Point};
    ///
    /// let res = Resolution::Custom(100, 100);
    /// assert_eq!(res.as_point(), Point::new(100.0, 100.0));
    /// assert_eq!(res.as_point() * Point::NEG_ONE, Point::new(-100.0, -100.0));
    /// ```
    fn as_point(&self) -> Point;
}
