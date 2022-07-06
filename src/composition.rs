use crate::object::Object;
use crate::resolution::Resolution;

#[derive(Debug)]
pub struct Composition {
    /// The resolution of the composition
    pub resolution: Resolution,

    /// The fixed framerate of the composition in `frames per seconds`
    pub framerate: f32,

    /// The duration of the composition in seconds
    pub duration: u16,

    pub name: String,

    pub objects: Vec<Object>,
}

impl Default for Composition {
    fn default() -> Self {
        Composition {
            resolution: Default::default(),
            framerate: 24.0,
            duration: 10,
            name: "UNKNOWN".to_string(),
            objects: Vec::new(),
        }
    }
}

impl Composition {
    pub fn calculate_frames(&self) -> usize {
        (self.framerate * self.duration as f32) as usize
    }

    pub fn calculate_bytes(&self) -> usize {
        let frames = self.calculate_frames();
        let per_frame_bytes = self.resolution.calculate_bytes(3);

        frames * per_frame_bytes
    }
}
