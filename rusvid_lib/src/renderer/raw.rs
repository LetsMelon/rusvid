use crate::composition::Composition;
use image::{ImageBuffer, RgbaImage};
use std::path::PathBuf;
use tiny_skia::{PremultipliedColorU8, ALPHA_U8_OPAQUE};

use crate::renderer::ImageRender;

#[derive(Debug)]
pub struct RawRender {
    tmp_dir_path: PathBuf,
}

impl RawRender {
    pub fn new(tmp_dir_path: &PathBuf) -> Self {
        RawRender {
            tmp_dir_path: tmp_dir_path.clone(),
        }
    }
}

impl ImageRender for RawRender {
    #[inline]
    fn generate_filepath(&self, frame_count: usize) -> PathBuf {
        let filename = format!("{}.bmp", frame_count);
        self.tmp_dir_path.join(std::path::Path::new(&filename))
    }

    #[inline]
    fn file_extension(&self) -> String {
        "bmp".to_string()
    }

    fn render(&self, composition: &Composition, frame_number: usize) -> anyhow::Result<()> {
        let file_path = self.generate_filepath(frame_number);

        let pixmap = self.render_pixmap(&composition)?;

        let width = composition.resolution().width() as u32;
        let height = composition.resolution().height() as u32;

        let pixels = pixmap.pixels();

        let image_buffer: RgbaImage = ImageBuffer::from_fn(width, height, |x, y| {
            let c: PremultipliedColorU8 = pixels[(width * y + x) as usize];

            let c = c.get();

            let mut r = ((c >> 0) & 0xFF) as u8;
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

        image_buffer.save(file_path)?;

        Ok(())
    }
}