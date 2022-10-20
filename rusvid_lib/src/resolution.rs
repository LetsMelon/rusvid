use crate::{
    metrics::{MetricsSize, MetricsVideo},
    prelude::AsPoint,
    types::Point,
};

// TODO move into types.rs
pub type ResolutionType = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Enum for common resolutions and custom ones
///
/// Currently only has presets for resolutions with 16:9 format. For other formats use `Resolution::Custom(width, height)`.
pub enum Resolution {
    SD,
    HD,
    FHD,
    QHD,
    UHD,
    TwoK,
    FourK,
    /// width, height
    Custom(usize, usize),
}

impl Resolution {
    #[inline]
    /// Get the width and height of the resolution.
    /// ```rust
    /// use rusvid_lib::resolution::Resolution;
    ///
    /// let res = Resolution::HD;
    /// assert_eq!(res.value(), (1280, 720));
    ///
    /// let res = Resolution::Custom(100, 100);
    /// assert_eq!(res.value(), (100, 100));
    /// ```
    pub fn value(&self) -> ResolutionType {
        match self {
            Resolution::SD => (640, 480),
            Resolution::HD => (1280, 720),
            Resolution::FHD => (1920, 1080),
            Resolution::QHD => (2560, 1440),
            Resolution::UHD => (3840, 2160),
            Resolution::TwoK => (2048, 1080),
            Resolution::FourK => (4096, 2160),
            Resolution::Custom(w, h) => (*w, *h),
        }
    }

    #[inline]
    /// Get the width of the resolution.
    /// ```rust
    /// use rusvid_lib::resolution::Resolution;
    ///
    /// let res = Resolution::HD;
    /// assert_eq!(res.width(), 1280);
    /// ```
    pub fn width(&self) -> usize {
        self.value().0
    }

    #[inline]
    /// Get the height of the resolution.
    /// ```rust
    /// use rusvid_lib::resolution::Resolution;
    ///
    /// let res = Resolution::HD;
    /// assert_eq!(res.height(), 720);
    /// ```
    pub fn height(&self) -> usize {
        self.value().1
    }

    #[inline]
    /// Get the width of the resolution as `f64`. Used for math
    /// ```rust
    /// use rusvid_lib::resolution::Resolution;
    ///
    /// let res = Resolution::HD;
    /// assert_eq!(res.x(), 1280.0);
    /// ```
    pub fn x(&self) -> f64 {
        self.width() as f64
    }

    #[inline]
    /// Get the height of the resolution as `f64`. Used for math
    /// ```rust
    /// use rusvid_lib::resolution::Resolution;
    ///
    /// let res = Resolution::HD;
    /// assert_eq!(res.y(), 720.0);
    /// ```
    pub fn y(&self) -> f64 {
        self.height() as f64
    }
}

impl MetricsVideo for Resolution {
    /// Returns the number of frames.
    ///
    /// For `Resolution` constant 1.
    fn frames(&self) -> usize {
        1
    }

    /// Returns the number of pixels.
    /// ```rust
    /// use rusvid_lib::resolution::Resolution;
    /// use rusvid_lib::metrics::MetricsVideo;
    ///
    /// let res = Resolution::HD;
    /// assert_eq!(res.pixels(), 921_600);
    /// ```
    fn pixels(&self) -> usize {
        let res = self.value();

        res.0 * res.1
    }
}

impl MetricsSize for Resolution {
    /// Returns the number of bytes.
    /// ```rust
    /// use rusvid_lib::resolution::Resolution;
    /// use rusvid_lib::metrics::MetricsSize;
    ///
    /// let res = Resolution::HD;
    /// assert_eq!(res.bytes(), 3_686_400);
    /// ```
    fn bytes(&self) -> usize {
        let pixels = self.pixels();

        // We use RGBA
        pixels * 4
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution::FHD
    }
}

impl AsPoint for Resolution {
    /// Returns values of the struct as crate::types::Point.
    /// ```rust
    /// use rusvid_lib::resolution::Resolution;
    /// use rusvid_lib::types::{AsPoint, Point};
    ///
    /// let res = Resolution::Custom(100, 100);
    /// assert_eq!(res.as_point(), Point::new(100.0, 100.0));
    /// ```
    fn as_point(&self) -> Point {
        Point::new(self.x(), self.y())
    }
}
