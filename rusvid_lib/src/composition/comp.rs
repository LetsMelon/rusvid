use debug_ignore::DebugIgnore;
use std::ops::{Deref, DerefMut};
use usvg::{Fill, Node, NodeExt, NodeKind, Paint, Tree};

use crate::animation::manager::AnimationManager;
use crate::composition::CompositionBuilder;
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

    pub(crate) rtree: DebugIgnore<Tree>,

    pub animations: AnimationManager,
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

    #[inline(always)]
    pub fn rtree(&self) -> &Tree {
        self.rtree.deref()
    }

    #[inline(always)]
    pub fn rtree_mut(&mut self) -> &mut Tree {
        self.rtree.deref_mut()
    }

    #[inline]
    pub fn add_to_defs(&mut self, kind: NodeKind) -> Node {
        self.rtree_mut().append_to_defs(kind)
    }

    #[inline]
    pub fn add_to_root(&mut self, kind: NodeKind) -> Node {
        if let NodeKind::Path(path) = &kind {
            self.animations
                .add_reference(path.id.clone(), path.data.clone());
        }
        self.rtree().root().append_kind(kind)
    }

    #[inline]
    pub fn fill_with_link(&self, id: &str) -> Option<Fill> {
        // TODO add check if the paint is in the defs?

        Some(Fill {
            paint: Paint::Link(id.to_string()),
            ..Fill::default()
        })
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

        frames * per_frame_bytes
    }
}
