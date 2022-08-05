use std::ops::{Deref, DerefMut};
use debug_ignore::DebugIgnore;
use usvg::{Fill, Node, NodeExt, NodeKind, Paint, Tree};
use crate::animation::Animation;

use crate::animation::manager::AnimationManager;
use crate::composition::CompositionBuilder;
use crate::resolution::Resolution;

#[derive(Debug)]
pub struct Layer {
    name: String,

    rtree: DebugIgnore<Tree>,

    animations: AnimationManager,
}

impl Layer {
    pub fn new(resolution: Resolution) -> Self {
        Layer {
            name: "layer_1".to_string(),
            rtree: DebugIgnore(CompositionBuilder::create_tree_from_resolution(
                resolution,
            )),
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

    #[inline]
    pub fn fill_with_link(&self, id: &str) -> Option<Fill> {
        // TODO add check if the paint is in the defs?

        Some(Fill {
            paint: Paint::Link(id.to_string()),
            ..Fill::default()
        })
    }

    #[inline]
    pub fn add_animation<T: Animation + 'static>(&mut self, animation: T) {
        self.animations.add_animation(animation);
    }
}
