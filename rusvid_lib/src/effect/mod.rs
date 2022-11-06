use anyhow::Result;
use image::RgbaImage;
use rusvid_core::plane::Plane;

pub mod library;

pub type ID = String;

// TODO move into separate file and use in the whole project for objects which can hold an id
pub trait Element {
    fn id(&self) -> Option<&ID>;
}

pub trait EffectLogic: std::fmt::Debug + Element {
    fn apply(&self, original: Plane) -> Result<Plane>;

    // TODO remove after rusvid_lib uses `rusvid_core::plane::Plane` internal everywhere
    fn internal_apply(&self, original: RgbaImage) -> Result<RgbaImage> {
        Plane::from_rgba_image(original)
            .and_then(|plane| self.apply(plane))
            .and_then(|plane| plane.as_rgba_image())
    }

    fn depends_on_other_effects_ids(&self) -> Vec<ID> {
        Vec::new()
    }

    /// Returns `true` if the effect depends on one (or more) other effects, otherwise the function returns `false`
    ///
    /// Example:
    /// ```rust
    /// use rusvid_lib::prelude::*;
    ///
    /// let effect = ColorPaletteEffect::new(vec![]);
    /// assert!(!effect.depends_on_other_effects());
    /// ```
    fn depends_on_other_effects(&self) -> bool {
        self.depends_on_other_effects_ids().len() != 0
    }

    #[allow(unused_variables)]
    fn add_depended_on_other_effect(&mut self, effect_id: &str) {}
}
