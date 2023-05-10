use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use rusvid_core::frame_image_format::FrameImageFormat;
use tracing::{debug, info};

use crate::composition::Composition;
use crate::metrics::MetricsVideo;
use crate::renderer::Renderer;

#[derive(Debug)]
pub struct FrameRenderer {
    out_dir: PathBuf,
    file_type: FrameImageFormat,
}

impl FrameRenderer {
    pub fn new(out_dir: impl Into<PathBuf>) -> Self {
        Self::new_with_file_type(out_dir, FrameImageFormat::default())
    }

    pub fn new_with_file_type(out_dir: impl Into<PathBuf>, file_type: FrameImageFormat) -> Self {
        FrameRenderer {
            out_dir: out_dir.into(),
            file_type,
        }
    }
}

impl Renderer for FrameRenderer {
    fn render(&mut self, mut composition: Composition) -> Result<()> {
        info!("Using renderer: {:?}", self);

        let out_dir = self.out_dir.to_path_buf();

        if out_dir.exists() {
            fs::remove_dir_all(&out_dir)?;
        }
        fs::create_dir(&out_dir)?;

        let frames = composition.frames();
        info!("frames: {}", frames);
        let frame_number_width = frames.to_string().len();

        for i in 0..frames {
            let frame_number_formatted = format!("{:01$}", i + 1, frame_number_width);
            info!("frame: {}", frame_number_formatted);

            composition.update(i)?;

            let buffer = self.render_single(&composition)?;

            let frame_path = buffer.save_with_format(
                out_dir.join(Path::new(&format!("frame_{frame_number_formatted}"))),
                self.file_type,
            )?;
            debug!("Saved frame at {:?}", frame_path);
        }

        Ok(())
    }

    fn out_path(&self) -> &std::path::Path {
        todo!()
    }

    fn tmp_dir_path(&self) -> &std::path::Path {
        todo!()
    }
}
