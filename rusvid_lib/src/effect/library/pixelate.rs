use anyhow::Result;
use image::{Rgba, RgbaImage};
use itertools::Itertools;

use crate::effect::{EffectLogic, Element, ID};

#[derive(Debug)]
pub struct PixelateEffect {
    pixel_width: u32,
    pixel_height: u32,

    id: Option<String>,
}

impl PixelateEffect {
    pub fn new(pixel_width: u32, pixel_height: u32) -> Self {
        PixelateEffect {
            pixel_width,
            pixel_height,
            id: None,
        }
    }

    pub fn new_with_id(pixel_width: u32, pixel_height: u32, id: impl Into<String>) -> Self {
        let mut effect = Self::new(pixel_width, pixel_height);
        effect.id = Some(id.into());

        effect
    }
}

impl Element for PixelateEffect {
    fn id(&self) -> Option<&ID> {
        self.id.as_ref()
    }
}

impl EffectLogic for PixelateEffect {
    fn apply(&self, original: RgbaImage) -> Result<RgbaImage> {
        // TODO create extra config if last pixel in a row should be not fixed size or if the extra margin should be applied to the last pixel, (width & height)
        // eg.: pixel_width = 19px; width = 1920px; pixel_width * width = 1919px, last pixel either 1px wide or last one is 20px wide
        let pixels_count_width = original.width().div_ceil(self.pixel_width);
        let pixels_count_height = original.height().div_ceil(self.pixel_height);

        let mut result = RgbaImage::new(original.width(), original.height());

        for x in 0..pixels_count_width {
            for y in 0..pixels_count_height {
                let from_pixels_width = x * self.pixel_width;
                let to_pixels_width = ((x + 1) * self.pixel_width).min(result.width());

                let from_pixels_height = y * self.pixel_height;
                let to_pixels_height = ((y + 1) * self.pixel_height).min(result.height());

                let sum = (from_pixels_width..to_pixels_width)
                    .cartesian_product(from_pixels_height..to_pixels_height)
                    .map(|(i_x, i_y)| original.get_pixel(i_x, i_y).0)
                    .fold([0_u64; 4], |acc, val| {
                        let mut back_value = acc;

                        back_value[0] += val[0] as u64;
                        back_value[1] += val[1] as u64;
                        back_value[2] += val[2] as u64;
                        back_value[3] += val[3] as u64;

                        back_value
                    });

                let summed_pixels = (((to_pixels_width + 1) - from_pixels_width)
                    * ((to_pixels_height + 1) - from_pixels_height))
                    as u64;

                let new_color = Rgba([
                    (sum[0] / summed_pixels) as u8,
                    (sum[1] / summed_pixels) as u8,
                    (sum[2] / summed_pixels) as u8,
                    (sum[3] / summed_pixels) as u8,
                ]);

                for i_x in from_pixels_width..to_pixels_width {
                    for i_y in from_pixels_height..to_pixels_height {
                        result.put_pixel(i_x, i_y, new_color);
                    }
                }
            }
        }

        Ok(result)
    }
}
