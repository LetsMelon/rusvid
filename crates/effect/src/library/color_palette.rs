use rayon::prelude::*;
use rusvid_core::pixel::Pixel;
use rusvid_core::plane::Plane;

use crate::error::EffectError;
use crate::functions::color_palette::transform;
use crate::{EffectLogic, Element, ID};

#[derive(Debug)]
/// Effect to apply a color palette effect on a [`Plane`].
///
/// See [color quantization](https://en.wikipedia.org/wiki/Color_quantization) or [palette computing](https://en.wikipedia.org/wiki/Palette_(computing))
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
    fn apply(&self, original: Plane) -> Result<Plane, EffectError> {
        if self.color_palette.is_empty() {
            return Err(EffectError::SizeError {
                message: "Must have at least one color in the color palette",
                value: 0,
            });
        }

        let mut result = Plane::new(original.width(), original.height())?;

        result
            .as_data_mut()
            .par_iter_mut()
            .for_each(|old| *old = transform(&old, &self.color_palette));

        Ok(result)
    }
}
