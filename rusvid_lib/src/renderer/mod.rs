use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

use anyhow::{bail, Result};
use image::{Rgba, RgbaImage};
use rusvid_core::plane::Plane;
use tiny_skia::{Pixmap, PremultipliedColorU8};

use crate::composition::Composition;
use crate::effect::EffectLogic;
use crate::layer::{Layer, LayerLogic};

pub mod ffmpeg;
pub mod frame_image_format;

fn render_pixmap_layer(layer: &Layer) -> Result<Pixmap> {
    let pixmap_size = layer
        .rtree()
        .expect("Expect a tree in the given layer")
        .svg_node()
        .size
        .to_screen_size();

    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .expect("Error while creating pixmap");
    resvg::render(
        layer.rtree().expect("Expect a tree in the given layer"),
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .expect("Error while rendering");

    Ok(pixmap)
}

fn pixmap_to_rgba_image(pixmap: Pixmap) -> RgbaImage {
    let buf: Vec<u8> = pixmap
        .pixels()
        .iter()
        .flat_map(|x| {
            let r = x.red();
            let g = x.green();
            let b = x.blue();
            let a = x.alpha();

            [r, g, b, a]
        })
        .collect();

    assert_eq!(pixmap.width() * pixmap.height() * 4, buf.len() as u32);

    RgbaImage::from_vec(pixmap.width(), pixmap.height(), buf).unwrap()
}

fn combine_renders(width: u32, height: u32, images: Vec<RgbaImage>) -> RgbaImage {
    RgbaImage::from_fn(width, height, |x, y| {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut a = 0;

        for layer_index in 0..images.len() {
            let c = images[layer_index].get_pixel(x, y);

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

        Rgba([r, g, b, a])
    })
}

fn apply_effects(original: RgbaImage, effects: &Vec<Box<dyn EffectLogic>>) -> Result<RgbaImage> {
    let mut back = original.clone();

    for effect in effects {
        back = effect.apply(back)?;
    }

    Ok(back)
}

pub trait Renderer {
    fn render(&mut self, composition: Composition) -> Result<()>;

    fn out_path(&self) -> &Path;
    fn tmp_dir_path(&self) -> &Path;

    fn render_rgba_image(&self, composition: &Composition) -> Result<RgbaImage> {
        let layers = composition.get_layers();
        if layers.len() == 0 {
            bail!("TODO: error")
        }

        let mut images = Vec::new();
        for layer in layers {
            let mut image = pixmap_to_rgba_image(render_pixmap_layer(layer)?);

            if layer.effects.len() != 0 {
                image = apply_effects(image, &layer.effects)?;
            }

            images.push(image);
        }

        let width = composition.resolution.width() as u32;
        let height = composition.resolution.height() as u32;
        let mut image = combine_renders(width, height, images);

        image = apply_effects(image, &composition.effects)?;

        Ok(image)
    }

    #[deprecated(since = "0.1.2", note = "use `render_plane` instead")]
    fn render_pixmap(&self, composition: &Composition) -> Result<Pixmap> {
        let image = self.render_rgba_image(&composition)?;
        let pixels = image.to_vec();

        let width = composition.resolution().width() as u32;
        let height = composition.resolution().height() as u32;

        let mut pixmap = Pixmap::new(width, height).expect("Error while creating pixmap");
        let data = pixmap.pixels_mut();

        assert_eq!(data.len() * 4, pixels.len());

        for i in 0..data.len() {
            let color = PremultipliedColorU8::from_rgba(
                pixels[(i * 4) + 0],
                pixels[(i * 4) + 1],
                pixels[(i * 4) + 2],
                pixels[(i * 4) + 3],
            )
            .expect("Error while creating color");

            data[i] = color;
        }

        Ok(pixmap)
    }

    fn render_plane(&self, composition: &Composition) -> Result<Plane> {
        let image = self.render_rgba_image(composition)?;
        let plane = Plane::from_rgba_image(image)?;
        Ok(plane)
    }
}

pub trait CliArgument {
    fn build_cli_argument(&self) -> Vec<OsString>;
}

pub trait CliCommand {
    fn build_command(&self, out_path: &std::path::Path, tmp_path: &std::path::Path) -> Command;
}
