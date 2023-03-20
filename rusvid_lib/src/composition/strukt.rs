use anyhow::Result;
use log::debug;

use crate::composition::CompositionBuilder;
use crate::effect::EffectLogic;
use crate::layer::{Layer, LayerType};
use crate::metrics::{MetricsSize, MetricsVideo};
use crate::resolution::Resolution;
use crate::types::FPS;

// TODO remove pub's
#[derive(Debug)]
pub struct Composition {
    /// The resolution of the composition
    pub(crate) resolution: Resolution,

    /// The fixed framerate of the composition in `frames per seconds`
    pub framerate: FPS,

    /// The duration of the composition in seconds
    pub duration: u16,

    pub name: String,

    pub layers: Vec<Layer>,

    pub(crate) effects: Vec<Box<dyn EffectLogic>>,
}

impl Composition {
    pub fn builder() -> CompositionBuilder {
        CompositionBuilder::default()
    }

    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    pub fn update(&mut self, frame_count: usize) -> Result<()> {
        debug!("Update composition at frame: {frame_count}");

        for layer in self.get_layers_mut() {
            layer.update(frame_count)?;
        }

        Ok(())
    }

    // TODO add layer
    // TODO get layer by id (only if they have an id)

    pub fn get_layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    pub fn get_layers_mut(&mut self) -> &mut Vec<Layer> {
        &mut self.layers
    }

    // TODO rename to `create_layer`
    pub fn create_new_layer(&mut self, layer_type: LayerType) -> Option<&mut Layer> {
        let layer = Layer::new(layer_type, self.resolution());
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
