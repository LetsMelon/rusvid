use anyhow::{Context, Result};
use resvg::usvg::{Fill, Node, NodeKind, Tree};

use crate::animation::Animation;
use crate::composition::CompositionBuilder;
use crate::effect::EffectLogic;
use crate::layer::{Layer, LayerLogic};
use crate::metrics::{MetricsSize, MetricsVideo};
use crate::resolution::Resolution;
use crate::types::FPS;

#[derive(Debug)]
pub struct Composition {
    /// The resolution of the composition
    pub(crate) resolution: Resolution,

    /// The fixed framerate of the composition in `frames per seconds`
    pub framerate: FPS,

    /// The duration of the composition in seconds
    pub duration: u16,

    pub name: String,

    pub(crate) layers: Vec<Layer>,

    pub(crate) effects: Vec<Box<dyn EffectLogic>>,
}

impl Composition {
    pub fn builder() -> CompositionBuilder {
        CompositionBuilder::default()
    }

    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    fn check_or_create_layer(&mut self) -> Result<()> {
        if self.layers.is_empty() {
            self.create_layer().context("Couldn't create a layer")?;
        };
        Ok(())
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn get_layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    pub fn update(&mut self, frame_count: usize) -> Result<()> {
        for layer in &mut self.layers {
            layer.update(frame_count)?;
        }
        Ok(())
    }

    pub fn create_layer(&mut self) -> Option<&mut Layer> {
        let layer = Layer::new(self.resolution());
        self.layers.push(layer);

        self.layers.last_mut()
    }
}

impl Default for Composition {
    fn default() -> Self {
        Composition::builder().build()
    }
}

impl MetricsVideo for Composition {
    fn frames(&self) -> usize {
        (self.framerate as u16 * self.duration) as usize
    }

    fn pixels(&self) -> usize {
        self.frames() * self.resolution.pixels()
    }
}

impl MetricsSize for Composition {
    fn bytes(&self) -> usize {
        let frames = self.frames();
        let per_frame_bytes = self.resolution.bytes();
        let layers = self.layers.len();

        frames * per_frame_bytes * layers
    }
}

impl LayerLogic for Composition {
    fn rtree(&self) -> Option<&Tree> {
        if self.layers.is_empty() {
            None
        } else {
            Some(self.layers[0].rtree()?)
        }
    }

    fn rtree_mut(&mut self) -> Option<&mut Tree> {
        if self.layers.is_empty() {
            None
        } else {
            Some(self.layers[0].rtree_mut()?)
        }
    }

    fn add_to_defs(&mut self, kind: NodeKind) -> Result<Node> {
        self.check_or_create_layer()?;
        self.layers[0].add_to_defs(kind)
    }

    fn add_to_root(&mut self, kind: NodeKind) -> Result<Node> {
        self.check_or_create_layer()?;
        self.layers[0].add_to_root(kind)
    }

    fn fill_with_link(&self, id: &str) -> Option<Fill> {
        if self.layers.is_empty() {
            None
        } else {
            self.layers[0].fill_with_link(id)
        }
    }

    fn add_animation<T: Animation + 'static>(&mut self, animation: T) {
        self.check_or_create_layer().unwrap();
        self.layers[0].add_animation(animation);
    }

    fn add_effect<T: EffectLogic + 'static>(&mut self, effect: T) {
        self.effects.push(Box::new(effect))
    }
}
