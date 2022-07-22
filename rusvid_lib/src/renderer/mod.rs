use std::ffi::OsString;
use std::path::Path;
use std::process::Command;
use tiny_skia::Pixmap;

use crate::composition::Composition;

pub mod ffmpeg;
pub mod png;
pub mod raw;

pub trait Renderer {
    fn render(&mut self, composition: Composition) -> anyhow::Result<()>;

    fn out_path(&self) -> &Path;
    fn tmp_dir_path(&self) -> &Path;
}

pub trait ImageRender {
    fn generate_filepath(&self, tmp_dir_path: &Path, frame_count: usize) -> std::path::PathBuf;
    fn file_extension(&self) -> String;
    fn render(
        &self,
        composition: &Composition,
        tmp_dir_path: &Path,
        frame_number: usize,
    ) -> anyhow::Result<()>;

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
}

pub trait CliArgument {
    fn build_cli_argument(&self) -> Vec<OsString>;
}

pub trait CliCommand {
    fn build_command(&self, out_path: &std::path::Path, tmp_path: &std::path::Path) -> Command;
}
