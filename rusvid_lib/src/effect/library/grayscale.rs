use anyhow::Result;
use itertools::Itertools;
use rusvid_core::plane::{Pixel, Plane};

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
    fn apply(&self, original: Plane) -> Result<Plane> {
        let width = original.width();
        let height = original.height();
        let data = original
            .into_iter()
            .map(|original_color| {
                let grayscale_value = (original_color[0] as f32 * MULTIPLIER_RED
                    + original_color[1] as f32 * MULTIPLIER_GREEN
                    + original_color[2] as f32 * MULTIPLIER_BLUE)
                    as u8;

                [
                    grayscale_value,
                    grayscale_value,
                    grayscale_value,
                    original_color[3],
                ]
            })
            .collect();

        Ok(Plane::from_data_unchecked(width, height, data))
    }
}
