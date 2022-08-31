use anyhow::{bail, Result};
use debug_ignore::DebugIgnore;
use image::RgbaImage;
use std::ops::{Deref, DerefMut};
use tiny_skia::Pixmap;
use usvg::{Fill, Node, NodeExt, NodeKind, Paint, Tree};

use crate::animation::manager::AnimationManager;
use crate::animation::Animation;
use crate::composition::CompositionBuilder;
use crate::resolution::Resolution;

mod transcoder;

#[derive(Debug)]
pub struct Layer {
    name: String,

    rtree: DebugIgnore<Tree>,

    animations: AnimationManager,
}

impl Layer {
    #[inline(always)]
    pub fn new(resolution: Resolution) -> Self {
        Layer {
            name: "layer_0".to_string(),
            rtree: DebugIgnore(CompositionBuilder::create_tree_from_resolution(resolution)),
            animations: AnimationManager::new(),
        }
    }

    #[inline(always)]
    pub fn rtree(&self) -> &Tree {
        self.rtree.deref()
    }

    #[inline(always)]
    pub fn rtree_mut(&mut self) -> &mut Tree {
        self.rtree.deref_mut()
    }

    #[inline(always)]
    pub fn add_to_defs(&mut self, kind: NodeKind) -> Node {
        self.rtree_mut().append_to_defs(kind)
    }

    #[inline(always)]
    pub fn add_to_root(&mut self, kind: NodeKind) -> Node {
        if let NodeKind::Path(path) = &kind {
            self.animations
                .add_reference(path.id.clone(), path.data.clone());
        }
        self.rtree().root().append_kind(kind)
    }

    #[inline(always)]
    pub fn fill_with_link(&self, id: &str) -> Option<Fill> {
        // TODO add check if the paint is in the defs?

        Some(Fill {
            paint: Paint::Link(id.to_string()),
            ..Fill::default()
        })
    }

    #[inline(always)]
    pub fn add_animation<T: Animation + 'static>(&mut self, animation: T) {
        self.animations.add_animation(animation);
    }

    #[inline(always)]
    pub fn update(&mut self, frame_count: usize) -> Result<()> {
        self.animations.update(frame_count)
    }
}

pub trait LayerTranscoder {
    fn render_pixmap(&self, layer: &Layer) -> Result<Pixmap> {
        let pixmap_size = layer.rtree().svg_node().size.to_screen_size();

        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .expect("Error while creating pixmap");
        resvg::render(
            layer.rtree(),
            usvg::FitTo::Original,
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        )
        .expect("Error while rendering");

        Ok(pixmap)
    }

    fn combine_layers(&self, layers: &Vec<Layer>) -> anyhow::Result<RgbaImage> {
        if layers.len() == 0 {
            bail!("TODO: error");
        }

        let mut pixmaps = Vec::new();
        for layer in layers {
            let pixmap = self.render_pixmap(layer)?;
            pixmaps.push(pixmap);
        }

        let image = self.combine_renders(pixmaps);

        Ok(image)
    }

    fn combine_renders(&self, pixmaps: Vec<Pixmap>) -> RgbaImage;
}
