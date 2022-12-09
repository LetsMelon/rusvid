use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::{canonicalize, read};
use std::path::Path;
use std::rc::Rc;

use anyhow::{bail, Context, Result};
use resvg::usvg::{Fill, LinearGradient, Node, NodeExt, NodeKind, Options, Paint, Tree};

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

// TODO move Layer into separate file
pub struct Layer {
    name: String,

    rtree: Tree,

    animations: AnimationManager,

    // Wrapper enum for Colors
    gradients: HashMap<String, Rc<LinearGradient>>,

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
            gradients: HashMap::new(),
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

        for node in rtree.root.descendants() {
            let node = &*node.borrow();
            match node {
                // TODO hmmmmm???
                // NodeKind::LinearGradient(ref gradient) => {
                //     layer_item.add_to_defs(NodeKind::LinearGradient(gradient.clone()))?;
                // }
                // NodeKind::RadialGradient(ref gradient) => {
                //     layer_item.add_to_defs(NodeKind::RadialGradient(gradient.clone()))?;
                // }
                // NodeKind::Svg(ref svg) => {
                //     layer_item.add_to_root(NodeKind::Svg(*svg))?;
                // }
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

    pub fn add_linear_gradient(&mut self, gradient: LinearGradient) {
        let id = gradient.id.clone();
        self.gradients.insert(id, Rc::new(gradient));
    }

    pub fn get_linear_gradient(&self, id: &str) -> Option<Paint> {
        let gradient = self.gradients.get(id)?;
        Some(Paint::LinearGradient(gradient.clone()))
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
        let tree = self
            .rtree_mut()
            .context("Error in getting a mutable reference to rtree")?;
        Ok(tree.root.append_kind(kind))
    }

    #[inline(always)]
    fn add_to_root(&mut self, kind: NodeKind) -> Result<Node> {
        if let NodeKind::Path(path) = &kind {
            self.animations
                .add_reference(path.id.clone(), path.data.clone());
        }
        Ok(self
            .rtree_mut()
            .context("Error in getting reference to rtree")?
            .root
            .append_kind(kind))
    }

    #[inline(always)]
    // TODO only linear gradient
    fn fill_with_link(&self, id: &str) -> Option<Fill> {
        Some(Fill {
            paint: self.get_linear_gradient(id)?,
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
