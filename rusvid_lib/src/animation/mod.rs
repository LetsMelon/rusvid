use anyhow::Result;
use std::rc::Rc;
use usvg::PathData;

use crate::layer::CacheLogic;

pub mod curves;
pub mod manager;
pub mod position_animation;

pub trait Animation: std::fmt::Debug + CacheLogic {
    // TODO maybe add internal frame_count state in the animation to track the frame number
    /// Called once every frame
    unsafe fn update(&mut self, path: Rc<PathData>, frame_count: &usize) -> Result<()>;

    fn object_id(&self) -> &str;
}
