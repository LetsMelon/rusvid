use crate::plane::Plane;

pub type HistogramChannel = [u64; 256];

pub struct RgbResult {
    pub r: u64,
    pub g: u64,
    pub b: u64,
}

impl RgbResult {
    fn from_rgb(r: u64, g: u64, b: u64) -> Self {
        RgbResult { r, g, b }
    }
}

pub struct Histogram {
    channel_r: HistogramChannel,
    channel_g: HistogramChannel,
    channel_b: HistogramChannel,
}

impl Histogram {
    pub fn new_from_plane(plane: &Plane) -> Self {
        let mut channel_r = [0; 256];
        let mut channel_g = [0; 256];
        let mut channel_b = [0; 256];

        plane.as_data().iter().for_each(|p| {
            let [r, g, b, a] = p.to_raw().map(usize::from);

            channel_r[r] += 1;
            channel_g[g] += 1;
            channel_b[b] += 1;
        });

        Histogram {
            channel_r,
            channel_g,
            channel_b,
        }
    }

    pub fn get_count_by_value(&self, value: u8) -> RgbResult {
        let value = value as usize;

        RgbResult::from_rgb(
            self.channel_r[value],
            self.channel_g[value],
            self.channel_b[value],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Histogram;
    use crate::pixel::Pixel;
    use crate::plane::Plane;

    #[test]
    fn new_from_plane() {
        let plane = Plane::new_with_fill(2, 2, Pixel::new(255, 0, 100, 0)).unwrap();

        let histogram = Histogram::new_from_plane(&plane);

        let values_count = histogram.get_count_by_value(255);
        assert_eq!(values_count.r, 4);
        assert_eq!(values_count.g, 0);
        assert_eq!(values_count.b, 0);

        let values_count = histogram.get_count_by_value(100);
        assert_eq!(values_count.r, 0);
        assert_eq!(values_count.g, 0);
        assert_eq!(values_count.b, 4);
    }
}
