use anyhow::Result;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use usvg::PathData;

use crate::animation::Animation;
use crate::layer::CacheLogic;

#[derive(Debug)]
pub struct AnimationManager {
    reference: HashMap<String, Rc<PathData>>,
    animations: Vec<Box<dyn Animation>>,
}

impl AnimationManager {
    pub fn new() -> AnimationManager {
        AnimationManager {
            reference: HashMap::new(),
            animations: Vec::new(),
        }
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

impl CacheLogic for AnimationManager {
    fn has_update(&self, frame_count: &usize) -> bool {
        if self.animations.len() == 0 {
            return false;
        }

        self.animations
            .iter()
            .map(|animation| animation.has_update(&frame_count))
            .reduce(|accum, item| accum | item)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    mod cache_logic {
        mod has_update {
            use crate::animation::curves::Function;
            use crate::animation::manager::{AnimationManager, CacheLogic};
            use crate::prelude::animation::functions::Linear;
            use crate::prelude::animation::PositionAnimation;

            #[test]
            fn return_false_if_no_animation() {
                let manager = AnimationManager::new();

                assert!(!manager.has_update(&0));
                assert!(!manager.has_update(&1_000));
            }

            #[test]
            fn works_with_multiple_animations() {
                let mut manager = AnimationManager::new();

                // dry run
                assert!(!manager.has_update(&0));
                assert!(!manager.has_update(&20));
                assert!(!manager.has_update(&25));
                assert!(!manager.has_update(&35));
                assert!(!manager.has_update(&50));
                assert!(!manager.has_update(&55));
                assert!(!manager.has_update(&60));
                assert!(!manager.has_update(&65));
                assert!(!manager.has_update(&75));

                manager.add_animation(PositionAnimation::new(
                    "test_1".to_string(),
                    Linear::new(25, 50, (0.0, 0.0).into(), (1.0, 0.0).into()).unwrap(),
                ));
                manager.add_animation(PositionAnimation::new(
                    "test_2".to_string(),
                    Linear::new(35, 55, (0.0, 0.0).into(), (1.0, 0.0).into()).unwrap(),
                ));
                manager.add_animation(PositionAnimation::new(
                    "test_3".to_string(),
                    Linear::new(65, 70, (0.0, 0.0).into(), (1.0, 0.0).into()).unwrap(),
                ));

                // check values
                assert!(!manager.has_update(&0));
                assert!(!manager.has_update(&20));
                assert!(manager.has_update(&25));
                assert!(manager.has_update(&35));
                assert!(manager.has_update(&50));
                assert!(manager.has_update(&55));
                assert!(!manager.has_update(&60));
                assert!(manager.has_update(&65));
                assert!(!manager.has_update(&75));
            }
        }
    }
}
