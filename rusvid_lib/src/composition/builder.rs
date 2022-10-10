use anyhow::{Context, Result};
use usvg::{AspectRatio, Size, Svg, Tree, ViewBox};

use crate::composition::Composition;

use crate::effect::EffectLogic;
use crate::layer::Layer;
use crate::resolution::Resolution;
use crate::types::FPS;

#[derive(Debug)]
pub struct CompositionBuilder {
    resolution: Resolution,
    framerate: FPS,
    duration: u16,
    name: String,
    layers: Vec<Layer>,
    effects: Vec<Box<dyn EffectLogic>>,
}

impl Default for CompositionBuilder {
    fn default() -> Self {
        let res = Resolution::default();

        CompositionBuilder {
            resolution: res,
            framerate: 30,
            duration: 10,
            name: "UNKNOWN".to_string(),
            layers: Vec::new(),
            effects: Vec::new(),
        }
    }
}

impl CompositionBuilder {
    pub(crate) fn create_tree_from_resolution(resolution: Resolution) -> Result<Tree> {
        let size = Size::new(resolution.x(), resolution.y())
            .context("Width oder height must be greater 0")?;

        Ok(Tree::create(Svg {
            size,
            view_box: ViewBox {
                rect: size.to_rect(0.0, 0.0),
                aspect: AspectRatio::default(),
            },
        }))
    }

    pub fn build(self) -> Composition {
        Composition {
            resolution: self.resolution,
            framerate: self.framerate,
            duration: self.duration,
            name: self.name,
            layers: self.layers,
            effects: self.effects,
        }
    }

    pub fn framerate(mut self, framerate: FPS) -> Self {
        self.framerate = framerate;
        self
    }

    pub fn resolution(mut self, resolution: Resolution) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn duration(mut self, duration: u16) -> Self {
        self.duration = duration;
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn add_layer(mut self, layer: Layer) -> Self {
        self.layers.push(layer);
        self
    }

    pub fn add_effect<T: EffectLogic + 'static>(mut self, effect: T) -> Self {
        self.effects.push(Box::new(effect));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Composition;
    use super::Resolution;

    #[test]
    fn takes_arguments_and_build_composition() {
        let comp = Composition::builder()
            .name("test")
            .resolution(Resolution::HD)
            .duration(15)
            .framerate(5)
            .build();

        assert_eq!(comp.resolution, Resolution::HD);
        assert_eq!(comp.framerate, 5);
        assert_eq!(comp.duration, 15);
        assert_eq!(comp.name, "test".to_string());
    }
}
