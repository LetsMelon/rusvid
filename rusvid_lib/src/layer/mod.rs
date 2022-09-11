use anyhow::{bail, Result};
use debug_ignore::DebugIgnore;
use std::fs::{canonicalize, read};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use tiny_skia::Pixmap;
use usvg::{Fill, Node, NodeExt, NodeKind, Options, Paint, Tree};

use crate::animation::manager::AnimationManager;
use crate::animation::Animation;
use crate::composition::CompositionBuilder;
use crate::resolution::Resolution;

pub trait LayerLogic {
    fn rtree(&self) -> Option<&Tree>;
    fn rtree_mut(&mut self) -> Option<&mut Tree>;
    fn add_to_defs(&mut self, kind: NodeKind) -> Result<Node>;
    fn add_to_root(&mut self, kind: NodeKind) -> Result<Node>;
    fn fill_with_link(&self, id: &str) -> Option<Fill>;
    fn add_animation<T: Animation + 'static>(&mut self, animation: T);
}

pub trait CacheLogic {
    /// returns `false` if no animation happens between `frame_count - 1` and `frame_count`, otherwise returns `true`
    fn has_update(&self, frame_count: &usize) -> bool;
}

#[derive(Debug)]
pub struct Layer {
    name: String,

    rtree: DebugIgnore<Tree>,

    animations: AnimationManager,

    pub(crate) cache: Option<Pixmap>,
}

impl Layer {
    #[inline(always)]
    pub fn new(resolution: Resolution) -> Self {
        Layer {
            name: "layer_0".to_string(),
            rtree: DebugIgnore(CompositionBuilder::create_tree_from_resolution(resolution)),
            animations: AnimationManager::new(),
            cache: None,
        }
    }

    pub fn from_file(resolution: Resolution, path: &Path) -> Result<Self> {
        if path.is_relative() {
            bail!("Path must be absolute")
        }

        let mut layer_item = Layer::new(resolution);

        let mut options = Options::default();
        options.resources_dir = canonicalize(path)
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()));
        options.keep_named_groups = true;

        let svg_data = read(path)?;
        let rtree = Tree::from_data(&svg_data, &options.to_ref())?;

        for node in rtree.root().descendants() {
            let node = &*node.borrow();
            match node {
                NodeKind::LinearGradient(ref gradient) => {
                    layer_item.add_to_defs(NodeKind::LinearGradient(gradient.clone()))?;
                }
                NodeKind::RadialGradient(ref gradient) => {
                    layer_item.add_to_defs(NodeKind::RadialGradient(gradient.clone()))?;
                }
                NodeKind::Svg(ref svg) => {
                    layer_item.add_to_root(NodeKind::Svg(*svg))?;
                }
                NodeKind::Group(ref group) => {
                    layer_item.add_to_root(NodeKind::Group(group.clone()))?;
                }
                NodeKind::Path(ref path) => {
                    layer_item.add_to_root(NodeKind::Path(path.clone()))?;
                }
                _ => (),
            }
        }

        Ok(layer_item)
    }

    #[inline(always)]
    pub fn update(&mut self, frame_count: usize) -> Result<()> {
        self.animations.update(frame_count)
    }
}

impl LayerLogic for Layer {
    #[inline(always)]
    fn rtree(&self) -> Option<&Tree> {
        Some(self.rtree.deref())
    }

    #[inline(always)]
    fn rtree_mut(&mut self) -> Option<&mut Tree> {
        Some(self.rtree.deref_mut())
    }

    #[inline(always)]
    fn add_to_defs(&mut self, kind: NodeKind) -> Result<Node> {
        Ok(self.rtree_mut().unwrap().append_to_defs(kind))
    }

    #[inline(always)]
    fn add_to_root(&mut self, kind: NodeKind) -> Result<Node> {
        if let NodeKind::Path(path) = &kind {
            self.animations
                .add_reference(path.id.clone(), path.data.clone());
        }
        Ok(self.rtree().unwrap().root().append_kind(kind))
    }

    #[inline(always)]
    fn fill_with_link(&self, id: &str) -> Option<Fill> {
        // TODO add check if the paint is in the defs?

        Some(Fill {
            paint: Paint::Link(id.to_string()),
            ..Fill::default()
        })
    }

    #[inline(always)]
    fn add_animation<T: Animation + 'static>(&mut self, animation: T) {
        self.animations.add_animation(animation);
    }
}

impl CacheLogic for Layer {
    fn has_update(&self, frame_count: &usize) -> bool {
        self.animations.has_update(frame_count)
    }
}
