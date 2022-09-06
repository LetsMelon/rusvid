use anyhow::Result;
use debug_ignore::DebugIgnore;
use std::ops::{Deref, DerefMut};
use usvg::{Fill, Node, NodeExt, NodeKind, Paint, Tree};

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
