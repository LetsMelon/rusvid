use anyhow::Result;
use image::{Rgba, RgbaImage};

use super::EffectLogic;

#[derive(Debug, Default)]
pub struct Grayscale {}

impl EffectLogic for Grayscale {
    fn execute(&mut self, data: &mut RgbaImage) -> Result<()> {
        for (_, _, pixel) in data.enumerate_pixels_mut() {
            let value =
                (pixel[0] as f32 * 0.299 + pixel[1] as f32 * 0.587 + pixel[2] as f32 * 0.114) as u8;
            *pixel = Rgba([value, value, value, pixel[3]]);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    macro_rules! test_image_data {
        (rgba, ($width:expr, $height:expr)) => {
            image::RgbaImage::from_fn($width, $height, |_, y| {
                let p = y as f32 / $height as f32;

                if p < 0.33 {
                    image::Rgba([255, 0, 0, 255])
                } else if p >= 0.33 && p < 0.66 {
                    image::Rgba([0, 255, 0, 255])
                } else {
                    image::Rgba([0, 0, 255, 255])
                }
            })
        };
    }

    macro_rules! grayscale {
        ($lum:expr) => {
            grayscale!($lum, 255)
        };
        ($lum:expr, $alpha:expr) => {
            Rgba([$lum, $lum, $lum, $alpha])
        };
    }

    use anyhow::Result;
    use image::Rgba;

    use crate::effects::grayscale::Grayscale;
    use crate::effects::EffectLogic;

    #[test]
    fn transforms_from_rgba() -> Result<()> {
        let (width, height) = (100, 100);
        let mut data = test_image_data!(rgba, (width, height));

        let mut effect = Grayscale::default();
        effect.execute(&mut data)?;

        assert_eq!(data.get_pixel(0, 0), &grayscale!(76));
        assert_eq!(data.get_pixel(0, 50), &grayscale!(149));
        assert_eq!(data.get_pixel(0, 99), &grayscale!(29));

        Ok(())
    }
}
