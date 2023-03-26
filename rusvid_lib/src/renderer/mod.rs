use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, Result};
use rusvid_core::plane::Plane;

use crate::composition::Composition;

pub mod embedded;
pub mod ffmpeg;
pub mod frame;
mod util;

use util::combine_renders;

pub trait Renderer {
    fn render(&mut self, composition: Composition) -> Result<()>;

    fn out_path(&self) -> &Path;
    fn tmp_dir_path(&self) -> &Path;

    fn render_single(&self, composition: &Composition) -> Result<Plane> {
        let layers = composition.get_layers();
        if layers.is_empty() {
            bail!("No layers in composition");
        }

        let resolution = composition.resolution();

        let mut frames = Vec::new();
        for layer in layers {
            let mut plane = layer
                .object
                .render(resolution.width(), resolution.height())?;

            for effect in &layer.effects {
                plane = effect.apply(plane)?;
            }

            frames.push(plane);
        }

        let mut combined = combine_renders(resolution.width(), resolution.height(), frames)?;

        for effect in &composition.effects {
            combined = effect.apply(combined)?;
        }

        Ok(combined)
    }
}

pub trait CliArgument {
    fn build_cli_argument(&self) -> Vec<OsString>;
}

pub trait CliCommand {
    fn build_command(&self, out_path: &std::path::Path, tmp_path: &std::path::Path) -> Command;
}
