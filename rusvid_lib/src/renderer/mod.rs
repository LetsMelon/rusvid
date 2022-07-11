use crate::composition::Composition;

pub mod ffmpeg;

pub trait Renderer {
    fn render<P: Into<std::path::PathBuf>>(
        &self,
        composition: Composition,
        out_path: P,
        tmp_path: P,
        position: std::rc::Rc<usvg::PathData>, // TODO remove this and add a `animation` trait/struct/... in Composition
    ) -> anyhow::Result<()>;
}
