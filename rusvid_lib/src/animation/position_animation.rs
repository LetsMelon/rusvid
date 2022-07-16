use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Pointer};
use std::rc::Rc;
use usvg::PathData;

use crate::animation::curves::Function;
use crate::animation::Animation;
use crate::types::Point2d;

pub struct PositionAnimation<T>
where
    T: Debug,
{
    position: Rc<PathData>,
    meta: Box<dyn Function<Value = T>>,
}

impl<T: std::fmt::Debug> Debug for PositionAnimation<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Position Animation { ")?;
        Debug::fmt(&self.position, f)?;
        f.write_str(", ")?;
        self.meta.internal_debug(f)?;
        f.write_str(" }")
    }
}

impl<T: std::fmt::Debug> PositionAnimation<T> {
    pub fn new(position: Rc<PathData>, meta: Box<dyn Function<Value = T>>) -> Self {
        PositionAnimation { position, meta }
    }
}

impl<T: std::fmt::Debug> Animation for PositionAnimation<T> {
    unsafe fn update(&mut self, frame_count: usize) -> anyhow::Result<()> {
        if frame_count > self.meta.start_frame() && frame_count < self.meta.end_frame() {
            let pd = Rc::get_mut_unchecked(&mut self.position);
            pd.transform(usvg::Transform::new_translate(5.0, 4.0));

            println!("{:?}", self.meta.calc(frame_count));

            // pd.transform(usvg::Transform::new_rotate(65.0 / (frame_count as f64)));
        }
        Ok(())
    }
}
