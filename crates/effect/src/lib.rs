#![feature(int_roundings)]
#![cfg_attr(coverage_nightly, feature(no_coverage))]

use anyhow::Result;
use rusvid_core::plane::Plane;

pub mod error;
mod functions;
pub mod library;

pub type ID = String;

// TODO move into separate file and use in the whole project for objects which can hold an id
pub trait Element {
    fn id(&self) -> Option<&ID>;

    fn name(&self) -> &str;
}

pub trait EffectLogic: std::fmt::Debug + Element {
    fn apply(&self, original: Plane) -> Result<Plane>;

    fn depends_on_other_effects_ids(&self) -> Vec<ID> {
        Vec::new()
    }

    /// Returns `true` if the effect depends on one (or more) other effects, otherwise the function returns `false`
    fn depends_on_other_effects(&self) -> bool {
        !self.depends_on_other_effects_ids().is_empty()
    }

    #[allow(unused_variables)]
    fn add_depended_on_other_effect(&mut self, effect_id: &str) {}
}
