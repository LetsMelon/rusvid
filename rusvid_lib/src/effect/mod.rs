use anyhow::Result;
use image::RgbaImage;

pub mod library;

pub trait EffectLogic: std::fmt::Debug {
    // TODO switch to `Plane`
    fn apply(self: &Self, original: &RgbaImage) -> Result<RgbaImage>;
}
