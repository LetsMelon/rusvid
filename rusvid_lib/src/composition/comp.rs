use crate::animation::Animation;
use anyhow::Result;
use usvg::{Fill, Node, NodeKind};

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
    pub fn get_layers(&self) -> &Vec<Layer> {
        &self.layers
    }

    #[inline]
    pub fn update(&mut self, frame_count: usize) -> Result<()> {
        for layer in &mut self.layers {
            let _ = layer.update(frame_count)?;
        }
        Ok(())
    }

    #[inline]
    fn check_or_create_layer(&mut self) {
        if self.layers.len() == 0 {
            self.layers.push(Layer::new(self.resolution()));
        }
    }

    #[inline]
    pub fn add_to_defs(&mut self, kind: NodeKind) -> Node {
        self.check_or_create_layer();
        self.layers[0].add_to_defs(kind)
    }

    // TODO create trait to keep in-sync with the functions of Layer-struct
    #[inline]
    pub fn add_to_root(&mut self, kind: NodeKind) -> Node {
        self.check_or_create_layer();
        self.layers[0].add_to_root(kind)
    }

    #[inline]
    pub fn fill_with_link(&self, id: &str) -> Option<Fill> {
        if self.layers.len() == 0 {
            None
        } else {
            self.layers[0].fill_with_link(id)
        }
    }

    #[inline]
    pub fn add_animation<T: Animation + 'static>(&mut self, animation: T) {
        self.check_or_create_layer();
        self.layers[0].add_animation(animation);
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
