use std::path::{Path, PathBuf};

use crate::composition::Composition;
use crate::renderer::ImageRender;

#[derive(Debug)]
pub struct PngRender {}

impl PngRender {
    pub fn new() -> Self {
        PngRender {}
    }
}

impl ImageRender for PngRender {
    #[inline]
    fn generate_filepath(&self, tmp_dir_path: &Path, frame_count: usize) -> PathBuf {
        let filename = format!("{}.png", frame_count);
        tmp_dir_path.join(std::path::Path::new(&filename))
    }

    #[inline]
    fn file_extension(&self) -> String {
        "png".to_string()
    }

    fn render(
        &self,
        composition: &Composition,
        tmp_dir_path: &Path,
        frame_number: usize,
    ) -> anyhow::Result<()> {
        let file_path = self.generate_filepath(tmp_dir_path, frame_number);

        let pixmap = self.render_pixmap(&composition)?;

        pixmap.save_png(file_path)?;

        Ok(())
    }
}
