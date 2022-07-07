use crate::object::Object;
use crate::resolution::Resolution;
use anyhow::Result;
use debug_ignore::DebugIgnore;
use resvg::render;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use tiny_skia::{Pixmap, Transform};
use usvg::{AspectRatio, FitTo, Size, Svg, Tree, ViewBox};

#[derive(Debug)]
pub struct Composition {
    /// The resolution of the composition
    resolution: Resolution,

    /// The fixed framerate of the composition in `frames per seconds`
    pub framerate: f32,

    /// The duration of the composition in seconds
    pub duration: u16,

    pub name: String,

    rtree: DebugIgnore<Tree>,
}

impl Composition {
    pub fn new(name: String, resolution: Resolution) -> Composition {
        Composition {
            name,
            rtree: DebugIgnore(Composition::create_tree_from_resolution(resolution)),
            resolution,
            ..Composition::default()
        }
    }

    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    pub fn rtree(&self) -> &Tree {
        self.rtree.deref()
    }

    pub fn rtree_mut(&mut self) -> &mut Tree {
        self.rtree.deref_mut()
    }

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

    pub fn save_single(&self, path: &Path) -> Result<()> {
        let pixmap_size = self.rtree().svg_node().size.to_screen_size();

        let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .expect("Error while creating pixmap");
        render(
            &self.rtree(),
            FitTo::Original,
            Transform::default(),
            pixmap.as_mut(),
        )
        .expect("Error while rendering");

        pixmap.save_png(path)?;

        Ok(())
    }
}

impl Default for Composition {
    fn default() -> Self {
        let res = Resolution::default();

        Composition {
            resolution: res,
            framerate: 24.0,
            duration: 10,
            name: "UNKNOWN".to_string(),
            rtree: DebugIgnore(Composition::create_tree_from_resolution(res.clone())),
        }
    }
}

// Metrics
impl Composition {
    pub fn calculate_frames(&self) -> usize {
        (self.framerate * self.duration as f32) as usize
    }

    pub fn calculate_bytes(&self) -> usize {
        let frames = self.calculate_frames();
        let per_frame_bytes = self.resolution.calculate_bytes(3);

        frames * per_frame_bytes
    }
}
