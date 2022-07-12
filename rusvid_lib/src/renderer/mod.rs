use crate::composition::Composition;
use std::ffi::OsString;
use std::process::Command;

pub mod ffmpeg;

pub trait Renderer {
    fn render<P: Into<std::path::PathBuf>>(
        &self,
        composition: Composition,
        out_path: P,
        tmp_path: P,
        position: std::rc::Rc<usvg::PathData>, // TODO remove this and add a `animation` trait/struct/... in Composition
    ) -> anyhow::Result<()>;

    fn generate_filepath<P: Into<std::path::PathBuf>>(
        tmp_dir_path: P,
        frame_count: usize,
    ) -> std::path::PathBuf {
        let tmp_dir_path: std::path::PathBuf = tmp_dir_path.into();
        let filename = format!("{}.png", frame_count);
        tmp_dir_path.join(std::path::Path::new(&filename))
    }

    fn render_single<P: Into<std::path::PathBuf>>(
        &self,
        composition: &Composition,
        tmp_dir_path: P,
        frame_number: usize,
    ) -> anyhow::Result<()> {
        let file_path = Self::generate_filepath(tmp_dir_path, frame_number);

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

        pixmap.save_png(file_path)?;

        Ok(())
    }
}

pub trait CliArgument {
    fn build_cli_argument(&self) -> Vec<OsString>;
}

pub trait CliCommand {
    fn build_command(&self, out_path: &std::path::Path, tmp_path: &std::path::Path) -> Command;
}
