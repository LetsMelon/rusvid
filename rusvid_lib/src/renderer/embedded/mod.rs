use std::path::{Path, PathBuf};

use anyhow::Result;
use log::{debug, info};
use rusvid_video_encoder::Encoder;

use crate::composition::Composition;
use crate::metrics::MetricsVideo;
use crate::renderer::Renderer;

#[derive(Debug)]
pub struct EmbeddedRenderer {
    out_path: PathBuf,
}

impl EmbeddedRenderer {
    pub fn new(out_path: impl Into<PathBuf>) -> Self {
        EmbeddedRenderer {
            out_path: out_path.into(),
        }
    }
}

impl Renderer for EmbeddedRenderer {
    fn render(&mut self, mut composition: Composition) -> Result<()> {
        info!("Using renderer: {:?}", self);

        let out_path = self.out_path.clone();
        let mut video_encoder = Encoder::new(
            out_path,
            composition.resolution().value(),
            composition.framerate as usize,
        )?;

        let frames = composition.frames();
        info!("frames: {}", frames);
        let frame_number_width = frames.to_string().len();

        for i in 0..frames {
            info!("frame: {:01$}", i + 1, frame_number_width);

            composition.update(i)?;

            let buffer = self.render_single(&composition)?;

            video_encoder.encode_plane(buffer)?;
            debug!("Encoded frame");
        }

        video_encoder.finish_stream()?;

        Ok(())
    }

    fn out_path(&self) -> &Path {
        todo!()
    }

    fn tmp_dir_path(&self) -> &Path {
        todo!()
    }
}
