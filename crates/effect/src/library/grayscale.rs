use rayon::prelude::*;
use rusvid_core::plane::Plane;

use crate::error::EffectError;
use crate::functions::grayscale;
use crate::{EffectLogic, Element, ID};

#[derive(Debug, Default)]
/// Effect to apply a [grayscale](https://en.wikipedia.org/wiki/Grayscale) effect on a [`Plane`].
pub struct GrayscaleEffect {
    id: Option<String>,
}

impl GrayscaleEffect {
    pub fn new() -> Self {
        GrayscaleEffect::default()
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

    fn name(&self) -> &str {
        "grayscale"
    }
}

impl EffectLogic for GrayscaleEffect {
    fn apply(&self, original: Plane) -> Result<Plane, EffectError> {
        let width = original.width();
        let height = original.height();

        let data = original
            .as_data()
            .clone()
            .par_iter()
            .map(grayscale::transform)
            .collect();

        Ok(Plane::from_data_unchecked(width, height, data))
    }
}
