use debug_ignore::DebugIgnore;
use std::ops::{Deref, DerefMut};
use usvg::{Fill, Node, NodeExt, NodeKind, Paint, Tree};

use crate::animation::manager::AnimationManager;
use crate::composition::CompositionBuilder;
use crate::layer::Layer;
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
}

impl Composition {
    #[inline(always)]
    pub fn builder() -> CompositionBuilder {
        CompositionBuilder::default()
    }

    #[inline(always)]
    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    #[inline]
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    #[inline]
    pub fn get_layers(&mut self) -> &Vec<Layer> {
        &self.layers
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
