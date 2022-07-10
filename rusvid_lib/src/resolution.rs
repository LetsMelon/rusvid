use crate::metrics::{MetricsSize, MetricsVideo};

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
    pub fn value(&self) -> ResolutionType {
        match self {
            Resolution::HD => (1280, 720),
            Resolution::FHD => (1920, 1080),
            Resolution::UHD => (3840, 2160),
            Resolution::FourK => (4096, 2160),
            Resolution::Custom(w, h) => (*w, *h),
        }
    }

    pub fn width(&self) -> usize {
        self.value().0
    }

    pub fn height(&self) -> usize {
        self.value().1
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

        pixels * 3
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution::FHD
    }
}
