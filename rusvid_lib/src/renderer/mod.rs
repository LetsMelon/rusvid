use anyhow::Result;
use std::path::PathBuf;
use std::rc::Rc;
use usvg::PathData;

use crate::composition::Composition;

pub mod ffmpeg;

pub trait Renderer {
    fn render<P: Into<PathBuf>>(
        &self,
        composition: Composition,
        out_path: P,
        tmp_path: P,
        position: Rc<PathData>, // TODO remove this and add a `animation` trait/struct/... in Composition
    ) -> Result<()>;
}
