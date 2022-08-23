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

        // We use RGBA
        pixels * 4
    }
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution::FHD
    }
}

impl<T: Into<usize>> From<(T, T)> for Resolution {
    fn from(raw: (T, T)) -> Self {
        let width: usize = raw.0.into();
        let height: usize = raw.1.into();

        Resolution::Custom(width, height)
    }
}

#[cfg(test)]
mod tests {
    mod from {
        macro_rules! primitive_test {
            ($type:ty) => {
                paste::item! {
                        #[test]
                        fn [< from_primitive_ $type >] () {
                            use crate::resolution::Resolution;

                            let w: $type = $type::default();
                            let h: $type = $type::default();
                            assert_eq!(
                                Resolution::from((w, h)),
                                Resolution::Custom(usize::try_from(w).unwrap(), usize::try_from(h).unwrap())
                            )
                        }
                }
            };
        }

        primitive_test!(usize);
        primitive_test!(u8);
        primitive_test!(u16);

        #[test]
        fn sth() {
            assert!(true);
        }
    }
}
