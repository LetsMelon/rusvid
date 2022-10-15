use glam::DVec2;

pub type FPS = u8;
pub type Point = DVec2;

pub trait AsPoint {
    fn as_point(&self) -> Point;
}
