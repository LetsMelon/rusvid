use std::fmt::Debug;
use std::fs::{canonicalize, read};
use std::path::Path;

use anyhow::{bail, Context, Result};
use usvg::{Fill, Node, NodeExt, NodeKind, Options, Paint, Tree};

use crate::animation::manager::AnimationManager;
use crate::animation::Animation;
use crate::composition::CompositionBuilder;
use crate::prelude::EffectLogic;
use crate::resolution::Resolution;

pub trait LayerLogic {
    fn rtree(&self) -> Option<&Tree>;
    fn rtree_mut(&mut self) -> Option<&mut Tree>;
    fn add_to_defs(&mut self, kind: NodeKind) -> Result<Node>;
    fn add_to_root(&mut self, kind: NodeKind) -> Result<Node>;
    fn fill_with_link(&self, id: &str) -> Option<Fill>;
    fn add_animation<T: Animation + 'static>(&mut self, animation: T);
    fn add_effect<T: EffectLogic + 'static>(&mut self, effect: T);
}

pub struct Layer {
    name: String,

    rtree: Tree,

    animations: AnimationManager,

    // TODO maybe use an EffectManger, here and in `rusvid_lib/src/composition/strukt.rs::Composition.effects`
    pub(crate) effects: Vec<Box<dyn EffectLogic>>,
}

impl Debug for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Layer")
            .field("name", &self.name)
            .field("rtree", &"NOT_PRINTABLE")
            .field("animations", &self.animations)
            .field("effects", &self.effects)
            .finish()
    }
}

impl Layer {
    #[inline(always)]
    pub fn new(resolution: Resolution) -> Self {
        Layer {
            name: "layer_0".to_string(),
            // TODO: remove unwrap
            rtree: CompositionBuilder::create_tree_from_resolution(resolution).unwrap(),
            animations: AnimationManager::new(),
            effects: Vec::new(),
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
        Some(&self.rtree)
    }

    #[inline(always)]
    fn rtree_mut(&mut self) -> Option<&mut Tree> {
        Some(&mut self.rtree)
    }

    #[inline(always)]
    fn add_to_defs(&mut self, kind: NodeKind) -> Result<Node> {
        Ok(self
            .rtree_mut()
            .context("Error in getting mutable reference to rtree")?
            .append_to_defs(kind))
    }

    #[inline(always)]
    fn add_to_root(&mut self, kind: NodeKind) -> Result<Node> {
        if let NodeKind::Path(path) = &kind {
            self.animations
                .add_reference(path.id.clone(), path.data.clone());
        }
        Ok(self
            .rtree()
            .context("Error in getting reference to rtree")?
            .root()
            .append_kind(kind))
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

    #[inline(always)]
    fn add_effect<T: EffectLogic + 'static>(&mut self, effect: T) {
        self.effects.push(Box::new(effect))
    }
}
