use anyhow::{bail, Result};
use image::{Rgba, RgbaImage};
use rusvid_plane::Pixel;

use crate::effect::{EffectLogic, Element, ID};

#[derive(Debug)]
pub struct ColorPaletteEffect {
    color_palette: Vec<Pixel>,

    id: Option<String>,
}

impl ColorPaletteEffect {
    pub fn new(color_palette: Vec<Pixel>) -> Self {
        ColorPaletteEffect {
            color_palette,
            id: None,
        }
    }

    pub fn new_with_id(color_palette: Vec<Pixel>, id: impl Into<ID>) -> Self {
        let mut cpe = Self::new(color_palette);
        cpe.id = Some(id.into());

        cpe
    }
}

impl Element for ColorPaletteEffect {
    fn id(&self) -> Option<&ID> {
        self.id.as_ref()
    }
}

impl EffectLogic for ColorPaletteEffect {
    fn apply(&self, original: RgbaImage) -> Result<RgbaImage> {
        if self.color_palette.len() == 0 {
            bail!("Must have at least one color in the color palette");
        }

        let mut result = RgbaImage::new(original.width(), original.height());

        for x in 0..result.width() {
            for y in 0..result.height() {
                let old_color = original.get_pixel(x, y);

                let mut best_palette_color = self.color_palette[0];
                let mut distance = u32::MAX;
                for i in 0..self.color_palette.len() {
                    let color_to_test = self.color_palette[i];
                    let test_distance = old_color[0].abs_diff(color_to_test[0]) as u32
                        + old_color[1].abs_diff(color_to_test[1]) as u32
                        + old_color[2].abs_diff(color_to_test[2]) as u32
                        + old_color[3].abs_diff(color_to_test[3]) as u32;

                    if test_distance < distance {
                        best_palette_color = color_to_test;
                        distance = test_distance;
                    }
                }

                let new_color = Rgba([
                    best_palette_color[0],
                    best_palette_color[1],
                    best_palette_color[2],
                    best_palette_color[3],
                ]);

                *result.get_pixel_mut(x, y) = new_color;
            }
        }

        Ok(result)
    }
}
