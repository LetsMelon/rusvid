use std::path::PathBuf;

use crate::composition::Composition;
use crate::renderer::ImageRender;

#[derive(Debug)]
pub struct PngRender {
    tmp_dir_path: PathBuf,
}

impl PngRender {
    pub fn new(tmp_dir_path: &PathBuf) -> Self {
        PngRender {
            tmp_dir_path: tmp_dir_path.clone(),
        }
    }
}

impl ImageRender for PngRender {
    #[inline]
    fn generate_filepath(&self, frame_count: usize) -> PathBuf {
        let filename = format!("{}.png", frame_count);
        self.tmp_dir_path.join(std::path::Path::new(&filename))
    }

    #[inline]
    fn file_extension(&self) -> String {
        "png".to_string()
    }

    fn render(&self, composition: &Composition, frame_number: usize) -> anyhow::Result<()> {
        let file_path = self.generate_filepath(frame_number);

        let pixmap = self.render_pixmap(&composition)?;

        pixmap.save_png(file_path)?;

        Ok(())
    }
}
