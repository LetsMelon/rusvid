use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use usvg::PathData;

use crate::animation::curves::Function;
use crate::animation::Animation;

pub struct PositionAnimation {
    meta: Box<dyn Function>,
    object_id: String,
}

impl Debug for PositionAnimation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl PositionAnimation {
    pub fn new(id: String, meta: Box<dyn Function>) -> Self {
        PositionAnimation {
            meta,
            object_id: id,
        }
    }
}

impl Animation for PositionAnimation {
    unsafe fn update(&mut self, mut path: Rc<PathData>, frame_count: &usize) -> anyhow::Result<()> {
        if *frame_count >= self.meta.start_frame() && *frame_count < self.meta.end_frame() {
            let pd = Rc::get_mut_unchecked(&mut path);

            let delta = self.meta.delta(*frame_count);
            println!("{} -> {:?}", frame_count, delta);
            pd.transform(usvg::Transform::new_translate(delta.x(), delta.y()));
            // let pos = self.meta.calc(frame_count);
            // println!("{} -> {:?}", frame_count, pos);
            // set_path(&mut pd, pos);
        }
        Ok(())
    }

    fn object_id(&self) -> &str {
        &self.object_id
    }
}
