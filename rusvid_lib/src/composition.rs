use debug_ignore::DebugIgnore;
use std::ops::{Deref, DerefMut};
use usvg::{AspectRatio, Fill, Node, NodeExt, NodeKind, Paint, Size, Svg, Tree, ViewBox};

use crate::metrics::{MetricsSize, MetricsVideo};
use crate::resolution::Resolution;
use crate::types::FPS;

#[derive(Debug)]
pub struct Composition {
    /// The resolution of the composition
    resolution: Resolution,

    /// The fixed framerate of the composition in `frames per seconds`
    pub framerate: FPS,

    /// The duration of the composition in seconds
    pub duration: u16,

    pub name: String,

    rtree: DebugIgnore<Tree>,
}

#[derive(Debug)]
pub struct CompositionBuilder {
    resolution: Resolution,
    framerate: FPS,
    duration: u16,
    name: String,
}

impl Default for CompositionBuilder {
    fn default() -> Self {
        let res = Resolution::default();

        CompositionBuilder {
            resolution: res,
            framerate: 30,
            duration: 10,
            name: "UNKNOWN".to_string(),
        }
    }
}

impl CompositionBuilder {
    fn create_tree_from_resolution(resolution: Resolution) -> Tree {
        let size = Size::new(resolution.width() as f64, resolution.height() as f64).unwrap();

        Tree::create(Svg {
            size,
            view_box: ViewBox {
                rect: size.to_rect(0.0, 0.0),
                aspect: AspectRatio::default(),
            },
        })
    }

    pub fn build(self) -> Composition {
        Composition {
            resolution: self.resolution,
            framerate: self.framerate,
            duration: self.duration,
            name: self.name,
            rtree: DebugIgnore(CompositionBuilder::create_tree_from_resolution(
                self.resolution,
            )),
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
    pub fn add_to_root(&self, kind: NodeKind) -> Node {
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

#[cfg(test)]
mod tests {
    use crate::composition::Composition;
    use crate::resolution::Resolution;

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
