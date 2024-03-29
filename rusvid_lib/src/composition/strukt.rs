use anyhow::Result;
use tracing::debug;

use crate::composition::CompositionBuilder;
use crate::effect::EffectLogic;
use crate::layer::{Layer, LayerType};
use crate::metrics::{MetricsSize, MetricsVideo};
use crate::resolution::Resolution;
use crate::types::FPS;

// TODO remove pub's
#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Composition {
    /// The resolution of the composition
    pub(crate) resolution: Resolution,

    /// The fixed framerate of the composition in `frames per seconds`
    pub framerate: FPS,

    /// The duration of the composition in seconds
    pub duration: u16,

    pub name: String,

    pub layers: Vec<Layer>,

    // TODO remove serde skip
    #[cfg_attr(any(feature = "serialize", feature = "deserialize"), serde(skip))]
    pub(crate) effects: Vec<Box<dyn EffectLogic>>,
}

unsafe impl Send for Composition {}
unsafe impl Sync for Composition {}

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

    pub fn create_layer(&mut self, layer_type: LayerType) -> Option<&mut Layer> {
        let layer = Layer::new(layer_type, self.resolution());
        self.layers.push(layer);

        self.layers.last_mut()
    }

    #[cfg(feature = "save_load")]
    pub fn save_as_file(&self, path: impl Into<std::path::PathBuf>) -> Result<()> {
        use std::fs::File;
        use std::io::Write;

        use bincode::serialize;
        use miniz_oxide::deflate::compress_to_vec;

        let encoded = serialize(&self)?;
        let compressed = compress_to_vec(&encoded, 6);

        let mut file = File::create(path.into())?;
        file.write_all(&compressed)?;

        Ok(())
    }

    #[cfg(feature = "save_load")]
    pub fn load_from_file(path: impl Into<std::path::PathBuf>) -> Result<Self> {
        use bincode::deserialize;
        use miniz_oxide::inflate::decompress_to_vec;

        let buffer = std::fs::read(path.into())?;

        let decompressed = decompress_to_vec(&buffer)?;
        let item = deserialize(&decompressed)?;

        Ok(item)
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
