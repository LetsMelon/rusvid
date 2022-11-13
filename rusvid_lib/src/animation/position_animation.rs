use std::fmt::Debug;
use std::rc::Rc;

use log::debug;
use usvg::PathData;

use crate::animation::curves::Function;
use crate::animation::Animation;

#[derive(Debug)]
pub struct PositionAnimation {
    curve: Box<dyn Function>,
    object_id: String,
}

impl PositionAnimation {
    pub fn new<T: Function + 'static>(id: impl Into<String>, curve: T) -> Self {
        PositionAnimation {
            curve: Box::new(curve),
            object_id: id.into(),
        }
    }
}

impl Animation for PositionAnimation {
    unsafe fn update(&mut self, mut path: Rc<PathData>, frame_count: &usize) -> anyhow::Result<()> {
        if *frame_count >= self.curve.start_frame() && *frame_count <= self.curve.end_frame() {
            let pd = Rc::get_mut_unchecked(&mut path);

            let delta = self.curve.delta(*frame_count);
            debug!("Update {}: {:?}", self.object_id(), delta);
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
