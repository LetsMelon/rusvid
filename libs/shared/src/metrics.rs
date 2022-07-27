pub trait MetricsSize {
    fn bytes(&self) -> usize;

    fn kilo_bytes(&self) -> usize {
        self.bytes() / 1024
    }

    fn mega_bytes(&self) -> f32 {
        self.kilo_bytes() as f32 / 1024.0
    }

    fn giga_bytes(&self) -> f32 {
        self.mega_bytes() / 1024.0
    }
}

pub trait MetricsVideo {
    fn frames(&self) -> usize;
    fn pixels(&self) -> usize;
}

#[cfg(test)]
mod tests {
    mod metrics_size {
        use crate::metrics::MetricsSize;

        struct StructForTesting {
            size: usize,
        }

        impl MetricsSize for StructForTesting {
            fn bytes(&self) -> usize {
                self.size
            }
        }

        #[test]
        fn convert() {
            let s = StructForTesting {
                size: 1_425_759_284,
            };

            assert_eq!(s.bytes(), 1_425_759_284);
            assert_eq!(s.kilo_bytes(), 1_392_343);
            assert_eq!(s.mega_bytes(), 1359.71);
            assert_eq!(s.giga_bytes(), 1.3278418);
        }
    }
}
