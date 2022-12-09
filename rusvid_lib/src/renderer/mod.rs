use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, Result};
use resvg::tiny_skia::Pixmap;
use rusvid_core::plane::Plane;

use crate::composition::Composition;
use crate::layer::LayerLogic;

pub mod ffmpeg;
pub mod frame_image_format;
mod util;

use util::{apply_effects, combine_renders};

pub trait Renderer {
    fn render(&mut self, composition: Composition) -> Result<()>;

    fn out_path(&self) -> &Path;
    fn tmp_dir_path(&self) -> &Path;

    fn render_single(&self, composition: &Composition) -> Result<Plane> {
        let layers = composition.get_layers();
        if layers.len() == 0 {
            bail!("TODO: error")
        }

        let mut frames = Vec::new();
        for layer in layers {
            let mut pixmap = Pixmap::new(
                composition.resolution().width(),
                composition.resolution().height(),
            )
            .expect("Error while creating pixmap");

            resvg::render(
                layer.rtree().expect("Expect a tree in the given layer"),
                resvg::usvg::FitTo::Original,
                resvg::tiny_skia::Transform::default(),
                pixmap.as_mut(),
            )
            .expect("Error while rendering");
            let mut frame = Plane::from_pixmap(pixmap);

            if layer.effects.len() != 0 {
                frame = apply_effects(frame, &layer.effects)?;
            }

            frames.push(frame);
        }

        let width = composition.resolution.width() as u32;
        let height = composition.resolution.height() as u32;
        let mut plane = combine_renders(width, height, frames)?;

        plane = apply_effects(plane, &composition.effects)?;

        Ok(plane)
    }
}

pub trait CliArgument {
    fn build_cli_argument(&self) -> Vec<OsString>;
}

pub trait CliCommand {
    fn build_command(&self, out_path: &std::path::Path, tmp_path: &std::path::Path) -> Command;
}
