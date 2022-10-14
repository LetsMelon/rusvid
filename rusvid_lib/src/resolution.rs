use crate::{
    metrics::{MetricsSize, MetricsVideo},
    types::Point,
};

pub type ResolutionType = (usize, usize);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Resolution {
    HD,
    FHD,
    UHD,
    FourK,
    /// width, height
    Custom(usize, usize),
}

impl Resolution {
    #[inline]
    pub fn value(&self) -> ResolutionType {
        match self {
            Resolution::HD => (1280, 720),
            Resolution::FHD => (1920, 1080),
            Resolution::UHD => (3840, 2160),
            Resolution::FourK => (4096, 2160),
            Resolution::Custom(w, h) => (*w, *h),
        }
    }

    #[inline]
    pub fn as_point(&self) -> Point {
        let (width, height) = self.value();
        Point::new(width as f64, height as f64)
    }

    #[inline]
    pub fn width(&self) -> usize {
        self.value().0
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.value().1
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.width() as f64
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.height() as f64
    }
}

impl MetricsVideo for Resolution {
    fn frames(&self) -> usize {
        1
    }

    fn pixels(&self) -> usize {
        let res = self.value();

        res.0 * res.1
    }
}

impl MetricsSize for Resolution {
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
