use image::{Rgba, RgbaImage};
use tiny_skia::{Pixmap, PremultipliedColorU8};

use crate::layer::LayerTranscoder;
use crate::resolution::Resolution;

#[derive(Debug)]
pub struct CpuLayerTranscoder {
    resolution: Resolution,
}

impl Default for CpuLayerTranscoder {
    fn default() -> Self {
        Self {
            resolution: Default::default(),
        }
    }
}

impl CpuLayerTranscoder {
    fn new(resolution: Resolution) -> Self {
        CpuLayerTranscoder {
            resolution,
            ..CpuLayerTranscoder::default()
        }
    }
}

impl LayerTranscoder for CpuLayerTranscoder {
    fn combine_renders(&self, pixmaps: Vec<Pixmap>) -> RgbaImage {
        let as_pixels: Vec<&[PremultipliedColorU8]> = pixmaps.iter().map(|x| x.pixels()).collect();

        let width = self.resolution.width() as u32;
        let height = self.resolution.height() as u32;

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
            }

            Rgba([r, g, b, a])
        });

        combined_layer_image
    }
}
