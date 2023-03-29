use serde::{Deserialize, Serialize};

use super::layer::Layer;
use super::resolution::Resolution;
use super::TranslateIntoRusvidGeneric;

#[derive(Deserialize, Serialize, Debug)]
pub struct Composition {
    pub resolution: Resolution,
    pub framerate: u32,
    pub layers: Vec<Layer>,
}

impl TranslateIntoRusvidGeneric for Composition {
    type OUTPUT = rusvid_lib::composition::Composition;

    fn translate(&self) -> Self::OUTPUT {
        let mut composition_builder = rusvid_lib::composition::Composition::builder()
            .resolution(self.resolution.translate())
            .duration(1)
            .framerate(self.framerate as u8);

        for layer in &self.layers {
            let translated_layer = layer.translate();
            // TODO set custom resolution

            composition_builder = composition_builder.add_layer(translated_layer);
        }

        composition_builder.build()
    }
}
