use anyhow::Result;
use image::RgbaImage;
use std::path::{Path, PathBuf};

use crate::composition::Composition;
use crate::renderer::ImageRender;

#[derive(Debug)]
pub struct RawRender {
    pub(crate) cache: Option<RgbaImage>,
}

impl RawRender {
    pub fn new() -> Self {
        RawRender { cache: None }
    }

    /*
    fn calculate_image_buffer_single(&self, pixmap: &Pixmap, width: u32, height: u32) -> RgbaImage {
        let pixels = pixmap.pixels();

        let image_buffer: RgbaImage = ImageBuffer::from_fn(width, height, |x, y| {
            let c: PremultipliedColorU8 = pixels[(width * y + x) as usize];

            let c = c.get();

            let mut r = (c & 0xFF) as u8;
            let mut g = ((c >> 8) & 0xFF) as u8;
            let mut b = ((c >> 16) & 0xFF) as u8;
            let a = ((c >> 24) & 0xFF) as u8;

            if a != ALPHA_U8_OPAQUE {
                let alpha = a as f64 / 255.0;
                r = (r as f64 / alpha + 0.5) as u8;
                g = (g as f64 / alpha + 0.5) as u8;
                b = (b as f64 / alpha + 0.5) as u8;
            }

            image::Rgba([r, g, b, a])
        });

        image_buffer
    }
     */

    #[inline]
    pub fn calculate_image_buffer(
        &mut self,
        composition: &mut Composition,
        frame_number: &usize,
    ) -> Result<RgbaImage> {
        self.render_rgba_image(composition, frame_number)
    }
}

impl ImageRender for RawRender {
    #[inline]
    fn generate_filepath(&self, tmp_dir_path: &Path, frame_count: usize) -> PathBuf {
        let filename = format!("{}.bmp", frame_count);
        tmp_dir_path.join(std::path::Path::new(&filename))
    }

    #[inline]
    fn file_extension(&self) -> String {
        "bmp".to_string()
    }

    fn render(
        &mut self,
        composition: &mut Composition,
        tmp_dir_path: &Path,
        frame_number: usize,
    ) -> anyhow::Result<()> {
        let file_path = self.generate_filepath(tmp_dir_path, frame_number);

        let image_buffer = self.calculate_image_buffer(composition, &frame_number)?;
        image_buffer.save(file_path)?;

        Ok(())
    }

    fn set_last_complete_render(&mut self, data: RgbaImage) {
        self.cache = Some(data);
    }

    fn get_last_complete_render(&self) -> Option<RgbaImage> {
        self.cache.clone()
    }
}
