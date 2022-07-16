use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use usvg::PathData;

use crate::animation::curves::Function;
use crate::animation::Animation;

pub struct PositionAnimation {
    position: Rc<PathData>,
    meta: Box<dyn Function>,
}

impl Debug for PositionAnimation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Position Animation { ")?;
        Debug::fmt(&self.position, f)?;
        f.write_str(", ")?;
        self.meta.internal_debug(f)?;
        f.write_str(" }")
    }
}

impl PositionAnimation {
    pub fn new(position: Rc<PathData>, meta: Box<dyn Function>) -> Self {
        PositionAnimation { position, meta }
    }
}

impl Animation for PositionAnimation {
    unsafe fn update(&mut self, frame_count: usize) -> anyhow::Result<()> {
        if frame_count > self.meta.start_frame() && frame_count < self.meta.end_frame() {
            let pd = Rc::get_mut_unchecked(&mut self.position);

            let delta = self.meta.delta(frame_count);
            pd.transform(usvg::Transform::new_translate(delta.x(), delta.y()));
        }
        Ok(())
    }
}
