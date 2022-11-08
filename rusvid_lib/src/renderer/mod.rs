use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, Result};
use rusvid_core::plane::{Pixel, Plane};
use tiny_skia::Pixmap;

use crate::composition::Composition;
use crate::effect::EffectLogic;
use crate::layer::LayerLogic;

pub mod ffmpeg;
pub mod frame_image_format;

fn combine_renders(width: u32, height: u32, images: Vec<Plane>) -> Result<Plane> {
    let images_as_data = images
        .iter()
        .map(|i| i.as_data())
        .collect::<Vec<&Vec<Pixel>>>();

    let data = (0..((width * height) as usize))
        .map(|i| {
            images_as_data.iter().fold([0_u8; 4], |acc, value| {
                let value = value[i];

                match (acc[3], value[3]) {
                    (0, 0) => acc, // both colors are fully transparent -> do nothing
                    (_, 0) => acc, // new color is fully transparent -> do nothing
                    // old color is transparent and the new color overrides it completely
                    (0, _) => value,
                    // mix both colors into a new one
                    (255, 255) => {
                        // TODO add flag if the layer should override the old one or "merge", if merge then use calculation from beneath match closure
                        value
                    }
                    // mix both colors into a new one
                    (_, _) => {
                        let bg_r = (acc[0] as f64) / 255.0;
                        let bg_g = (acc[1] as f64) / 255.0;
                        let bg_b = (acc[2] as f64) / 255.0;
                        let bg_a = (acc[3] as f64) / 255.0;

                        let fg_r = (value[0] as f64) / 255.0;
                        let fg_g = (value[1] as f64) / 255.0;
                        let fg_b = (value[2] as f64) / 255.0;
                        let fg_a = (value[3] as f64) / 255.0;

                        let mix_a = 1.0 - (1.0 - fg_a) * (1.0 - bg_a);
                        let mix_r = fg_r * fg_a / mix_a + bg_r * bg_a * (1.0 - fg_a) / mix_a;
                        let mix_g = fg_g * fg_a / mix_a + bg_g * bg_a * (1.0 - fg_a) / mix_a;
                        let mix_b = fg_b * fg_a / mix_a + bg_b * bg_a * (1.0 - fg_a) / mix_a;

                        [
                            (mix_r * 255.0) as u8,
                            (mix_g * 255.0) as u8,
                            (mix_b * 255.0) as u8,
                            (mix_a * 255.0) as u8,
                        ]
                    }
                }
            })
        })
        .collect();

    Ok(Plane::from_data_unchecked(width, height, data))
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
