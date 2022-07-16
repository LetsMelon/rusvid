use anyhow::Result;

pub mod curves;
pub mod position_animation;

pub trait Animation {
    // TODO maybe add internal frame_count state in the animation to track the frame number
    /// Called once every frame
    unsafe fn update(&mut self, frame_count: usize) -> Result<()>;
}
