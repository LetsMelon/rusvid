use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

use anyhow::Result;
use resvg::usvg::PathData;

use crate::animation::Animation;

#[derive(Debug, Default)]
pub struct AnimationManager {
    reference: HashMap<String, Rc<PathData>>,
    animations: Vec<Box<dyn Animation>>,
}

impl AnimationManager {
    pub fn new() -> AnimationManager {
        AnimationManager::default()
    }

    #[inline]
    pub fn add_reference(&mut self, id: String, path: Rc<PathData>) {
        self.reference.insert(id, path);
    }

    #[inline]
    pub fn add_animation<T: Animation + 'static>(&mut self, animation: T) {
        self.animations.push(Box::new(animation))
    }

    #[inline]
    pub fn update(&mut self, frame_count: usize) -> Result<()> {
        for animation in &mut self.animations {
            let id = animation.object_id();
            let rc = self.reference.get(id);

            if let Some(pd) = rc {
                unsafe {
                    animation.update(pd.clone(), &frame_count)?;
                }
            }
        }
        Ok(())
    }
}
