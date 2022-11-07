use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, Result};
use rusvid_core::plane::Plane;
use tiny_skia::Pixmap;

use crate::composition::Composition;
use crate::effect::EffectLogic;
use crate::layer::LayerLogic;

pub mod ffmpeg;
pub mod frame_image_format;

fn combine_renders(width: u32, height: u32, images: Vec<Plane>) -> Result<Plane> {
    let mut plane = Plane::new(width, height)?;

    for x in 0..width {
        for y in 0..height {
            let mut r = 0;
            let mut g = 0;
            let mut b = 0;
            let mut a = 0;

            for layer_index in 0..images.len() {
                let c = images[layer_index].pixel_unchecked(x, y);

                let new_r = c[0] as u8;
                let new_g = c[1] as u8;
                let new_b = c[2] as u8;
                let new_a = c[3] as u8;

                match (a, new_a) {
                    (0, 0) => (), // both colors are fully transparent -> do nothing
                    (_, 0) => (), // new color is fully transparent -> do nothing
                    // old color is transparent and the new color overrides it completely
                    (0, _) => {
                        r = new_r;
                        g = new_g;
                        b = new_b;
                        a = new_a;
                    }
                    // mix both colors into a new one
                    (255, 255) => {
                        // TODO add flag if the layer should override the old one or "merge", if merge then use calculation from beneath match closure
                        r = new_r;
                        g = new_g;
                        b = new_b;
                        a = new_a;
                    }
                    // mix both colors into a new one
                    (_, _) => {
                        let bg_r = (r as f64) / 255.0;
                        let bg_g = (g as f64) / 255.0;
                        let bg_b = (b as f64) / 255.0;
                        let bg_a = (a as f64) / 255.0;

                        let fg_r = (new_r as f64) / 255.0;
                        let fg_g = (new_g as f64) / 255.0;
                        let fg_b = (new_b as f64) / 255.0;
                        let fg_a = (new_a as f64) / 255.0;

                        let mix_a = 1.0 - (1.0 - fg_a) * (1.0 - bg_a);
                        let mix_r = fg_r * fg_a / mix_a + bg_r * bg_a * (1.0 - fg_a) / mix_a;
                        let mix_g = fg_g * fg_a / mix_a + bg_g * bg_a * (1.0 - fg_a) / mix_a;
                        let mix_b = fg_b * fg_a / mix_a + bg_b * bg_a * (1.0 - fg_a) / mix_a;

                        a = (mix_a * 255.0) as u8;
                        r = (mix_r * 255.0) as u8;
                        g = (mix_g * 255.0) as u8;
                        b = (mix_b * 255.0) as u8;
                    }
                };
            }

            let c = [r, g, b, a];
            plane.put_pixel_unchecked(x, y, c);
        }
    }

    Ok(plane)
}

fn apply_effects(original: Plane, effects: &Vec<Box<dyn EffectLogic>>) -> Result<Plane> {
    let mut back = original;

    for effect in effects {
        back = effect.apply(back)?;
    }

    Ok(back)
}

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
                usvg::FitTo::Original,
                tiny_skia::Transform::default(),
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
