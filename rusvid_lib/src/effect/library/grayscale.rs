use anyhow::Result;
use image::{Rgba, RgbaImage};

use crate::effect::{EffectLogic, Element, ID};

const MULTIPLIER_RED: f32 = 0.299;
const MULTIPLIER_GREEN: f32 = 0.587;
const MULTIPLIER_BLUE: f32 = 0.114;

#[derive(Debug)]
pub struct GrayscaleEffect {
    id: Option<String>,
}

impl GrayscaleEffect {
    pub fn new() -> Self {
        GrayscaleEffect { id: None }
    }

    pub fn new_with_id(id: impl Into<String>) -> Self {
        let mut effect = Self::new();
        effect.id = Some(id.into());

        effect
    }
}

impl Element for GrayscaleEffect {
    fn id(&self) -> Option<&ID> {
        self.id.as_ref()
    }
}

impl EffectLogic for GrayscaleEffect {
    fn apply(&self, original: &RgbaImage) -> Result<RgbaImage> {
        let mut result = original.clone();

        for x in 0..result.width() {
            for y in 0..result.height() {
                let original_color = original.get_pixel(x, y);

                let grayscale_value = (original_color[0] as f32 * MULTIPLIER_RED
                    + original_color[1] as f32 * MULTIPLIER_GREEN
                    + original_color[2] as f32 * MULTIPLIER_BLUE)
                    as u8;

                *result.get_pixel_mut(x, y) = Rgba([
                    grayscale_value,
                    grayscale_value,
                    grayscale_value,
                    original_color[3],
                ]);
            }
        }

        Ok(result)
    }
}
