use anyhow::{bail, Result};
use rayon::prelude::*;
use rusvid_core::plane::{Pixel, Plane};

use crate::{EffectLogic, Element, ID};

#[inline(always)]
fn calculate_color_diff(c1: &Pixel, c2: &Pixel) -> u32 {
    c1[0].abs_diff(c2[0]) as u32
        + c1[1].abs_diff(c2[1]) as u32
        + c1[2].abs_diff(c2[2]) as u32
        + c1[3].abs_diff(c2[3]) as u32
}

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

    pub fn palette_length(&self) -> usize {
        self.color_palette.len()
    }
}

impl Element for ColorPaletteEffect {
    fn id(&self) -> Option<&ID> {
        self.id.as_ref()
    }

    fn name(&self) -> &str {
        "color palette"
    }
}

impl EffectLogic for ColorPaletteEffect {
    fn apply(&self, original: Plane) -> Result<Plane> {
        if self.color_palette.is_empty() {
            bail!("Must have at least one color in the color palette");
        }

        let mut result = Plane::new(original.width(), original.height())?;

        result.as_data_mut().par_iter_mut().for_each(|old_color| {
            let mut best_palette_color = self.color_palette[0];
            let mut distance = u32::MAX;
            for i in 0..self.color_palette.len() {
                let color_to_test = self.color_palette[i];
                let test_distance = calculate_color_diff(old_color, &color_to_test);

                if test_distance < distance {
                    best_palette_color = color_to_test;
                    distance = test_distance;
                }
            }

            *old_color = best_palette_color.clone();
        });

        Ok(result)
    }
}
