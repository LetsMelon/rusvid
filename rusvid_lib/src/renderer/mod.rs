use anyhow::bail;
use image::{Rgba, RgbaImage};
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;
use tiny_skia::{Pixmap, PremultipliedColorU8};

use crate::composition::Composition;
use crate::layer::{CacheLogic, Layer, LayerLogic};

pub mod ffmpeg;
pub mod png;
pub mod raw;

pub trait Renderer {
    fn render(&mut self, composition: Composition) -> anyhow::Result<()>;

    fn out_path(&self) -> &Path;
    fn tmp_dir_path(&self) -> &Path;
}

fn render_pixmap_layer(layer: &Layer) -> anyhow::Result<Pixmap> {
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

fn combine_renders(width: u32, height: u32, pixmaps: Vec<Pixmap>) -> RgbaImage {
    let as_pixels: Vec<&[PremultipliedColorU8]> = pixmaps.iter().map(|x| x.pixels()).collect();

    let combined_layer_image = RgbaImage::from_fn(width, height, |x, y| {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut a = 0;

        let array_index = (y * width + x) as usize;
        for layer_index in 0..as_pixels.len() {
            let c = as_pixels[layer_index][array_index].get();

            let new_r = (c & 0xFF) as u8;
            let new_g = ((c >> 8) & 0xFF) as u8;
            let new_b = ((c >> 16) & 0xFF) as u8;
            let new_a = ((c >> 24) & 0xFF) as u8;

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

            /*
            println!(
                "c{} at {:?}({:?}): r {}, g {}, b {}, a {}",
                layer_index,
                (x, y),
                array_index,
                r,
                g,
                b,
                a
            );
             */
        }
        /*
        println!(
            "c at {:?}({:?}): r {}, g {}, b {}, a {}",
            (x, y),
            array_index,
            r,
            g,
            b,
            a
        );
         */

        Rgba([r, g, b, a])
    });

    combined_layer_image
}

pub trait ImageRender {
    fn generate_filepath(&self, tmp_dir_path: &Path, frame_count: usize) -> std::path::PathBuf;
    fn file_extension(&self) -> String;
    fn render(
        &mut self,
        composition: &mut Composition,
        tmp_dir_path: &Path,
        frame_number: usize,
    ) -> anyhow::Result<()>;

    fn set_last_complete_render(&mut self, data: RgbaImage);
    fn get_last_complete_render(&self) -> Option<RgbaImage>;

    fn render_rgba_image(
        &mut self,
        composition: &mut Composition,
        frame_number: &usize,
    ) -> anyhow::Result<RgbaImage> {
        let layers = composition.get_layers_mut();
        if layers.len() == 0 {
            bail!("TODO: error")
        }

        let mut only_used_cache = true;
        let mut pixmaps = Vec::new();
        for layer in layers {
            let has_update = layer.has_update(frame_number);

            let pixmap = match (has_update, &layer.cache) {
                (false, Some(data)) => {
                    println!("Cached layer");
                    data.clone()
                }
                (_, _) => {
                    let pixmap = render_pixmap_layer(layer)?;

                    println!("Set cache for layer");
                    layer.cache = Some(pixmap.clone());
                    only_used_cache = false;

                    pixmap
                }
            };

            pixmaps.push(pixmap);
        }

        let image = match (only_used_cache, self.get_last_complete_render()) {
            (true, Some(data)) => {
                println!("Cached layer combination");
                data.clone()
            }
            _ => {
                let width = composition.resolution.width() as u32;
                let height = composition.resolution.height() as u32;
                let data = combine_renders(width, height, pixmaps);

                println!("Set layer combination cache");
                self.set_last_complete_render(data.clone());

                data
            }
        };

        Ok(image)
    }

    fn render_pixmap(
        &mut self,
        composition: &mut Composition,
        frame_number: &usize,
    ) -> anyhow::Result<Pixmap> {
        let image = self.render_rgba_image(composition, frame_number)?;
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
}

pub trait CliArgument {
    fn build_cli_argument(&self) -> Vec<OsString>;
}

pub trait CliCommand {
    fn build_command(&mut self, out_path: &std::path::Path, tmp_path: &std::path::Path) -> Command;
}
