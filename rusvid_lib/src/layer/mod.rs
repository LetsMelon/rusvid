use anyhow::Result;
use resvg::usvg::{Fill, Node, NodeKind, Tree};

use crate::animation::Animation;
use crate::prelude::EffectLogic;

mod strukt;
pub use strukt::Layer;

pub trait LayerLogic {
    fn rtree(&self) -> Option<&Tree>;
    fn rtree_mut(&mut self) -> Option<&mut Tree>;
    fn add_to_defs(&mut self, kind: NodeKind) -> Result<Node>;
    fn add_to_root(&mut self, kind: NodeKind) -> Result<Node>;
    fn fill_with_link(&self, id: &str) -> Option<Fill>;
    fn add_animation<T: Animation + 'static>(&mut self, animation: T);
    fn add_effect<T: EffectLogic + 'static>(&mut self, effect: T);
}
