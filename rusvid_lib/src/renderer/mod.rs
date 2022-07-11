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
}

pub trait CliArgument {
    fn build_cli_argument(&self) -> Vec<OsString>;
}

pub trait CliCommand {
    fn build_command(&self, out_path: &std::path::Path, tmp_path: &std::path::Path) -> Command;
}
