use anyhow::Result;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use usvg::PathData;

pub mod curves;
pub mod manager;
pub mod position_animation;

pub trait Animation: std::fmt::Debug {
    // TODO maybe add internal frame_count state in the animation to track the frame number
    /// Called once every frame
    unsafe fn update(&mut self, path: Rc<PathData>, frame_count: &usize) -> Result<()>;

    fn object_id(&self) -> &str;
}
