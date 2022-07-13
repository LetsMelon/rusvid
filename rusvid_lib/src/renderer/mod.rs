use crate::composition::Composition;
use std::ffi::OsString;
use std::process::Command;
use tiny_skia::Pixmap;

pub mod ffmpeg;
pub mod png;

pub trait Renderer {
    fn render<P: Into<std::path::PathBuf>>(
        &self,
        composition: Composition,
        out_path: P,
        tmp_path: P,
        position: std::rc::Rc<usvg::PathData>, // TODO remove this and add a `animation` trait/struct/... in Composition
    ) -> anyhow::Result<()>;
}

pub trait ImageRender {
    fn generate_filepath(&self, frame_count: usize) -> std::path::PathBuf;

    fn render_pixmap(&self, composition: &Composition) -> anyhow::Result<Pixmap> {
        let pixmap_size = composition.rtree().svg_node().size.to_screen_size();

        let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .expect("Error while creating pixmap");
        resvg::render(
            composition.rtree(),
            usvg::FitTo::Original,
            tiny_skia::Transform::default(),
            pixmap.as_mut(),
        )
        .expect("Error while rendering");

        Ok(pixmap)
    }

    fn render(&self, composition: &Composition, frame_number: usize) -> anyhow::Result<()>;
}

pub trait CliArgument {
    fn build_cli_argument(&self) -> Vec<OsString>;
}

pub trait CliCommand {
    fn build_command(&self, out_path: &std::path::Path, tmp_path: &std::path::Path) -> Command;
}
