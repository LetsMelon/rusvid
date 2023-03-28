use serde::{Deserialize, Serialize};

use super::{Object, TranslateIntoRusvidGeneric};

#[derive(Deserialize, Serialize, Debug)]
pub struct Layer {
    pub objects: Vec<Object>,
}

impl TranslateIntoRusvidGeneric for Layer {
    type OUTPUT = rusvid_lib::layer::Layer;

    fn translate(&self) -> Self::OUTPUT {
        let mut layer = rusvid_lib::layer::Layer::new(
            rusvid_lib::layer::LayerType::Svg,
            rusvid_lib::resolution::Resolution::Custom(100, 100),
        );

        for item in &self.objects {
            layer.add_svg_item(item.translate()).unwrap();
        }

        layer
    }
}
