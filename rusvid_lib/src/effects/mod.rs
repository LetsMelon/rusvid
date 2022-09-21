use anyhow::Result;
use image::RgbaImage;
use std::fmt::Debug;

pub mod grayscale;

pub trait EffectLogic: Debug {
    fn execute(&mut self, data: &mut RgbaImage) -> Result<()>;
}
