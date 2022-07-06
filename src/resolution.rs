pub type ResolutionType = (usize, usize);

#[derive(Debug)]
pub enum Resolution {
    HD,
    FHD,
    UHD,
    FourK,
    Custom(ResolutionType),
}

impl Resolution {
    pub fn value(&self) -> ResolutionType {
        match self {
            Resolution::HD => (1280, 720),
            Resolution::FHD => (1920, 1080),
            Resolution::UHD => (3840, 2160),
            Resolution::FourK => (4096, 2160),
            Resolution::Custom(res) => *res,
        }
    }

    pub fn calculate_pixels(&self) -> usize {
        let res = self.value();

        res.0 * res.1
    }

    pub fn calculate_bytes(&self, pixel_depth: usize) -> usize {
        let pixels = self.calculate_pixels();

        pixels * pixel_depth
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution::FHD
    }
}
