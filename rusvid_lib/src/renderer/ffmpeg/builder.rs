use std::path::PathBuf;

use rusvid_core::frame_image_format::FrameImageFormat;

use crate::prelude::{Composition, FPS};
use crate::renderer::ffmpeg::codec::VideoCodec;
use crate::renderer::ffmpeg::pixel_formats::PixelFormats;
use crate::renderer::ffmpeg::FfmpegRenderer;

#[derive(Debug)]
pub struct FfmpegRendererBuilder {
    codec: String,
    video_codec: VideoCodec,
    pixel_format: Option<PixelFormats>,
    framerate: FPS,
    frame_output_format: FrameImageFormat,
    out_path: PathBuf,
    tmp_dir_path: PathBuf,
}

impl Default for FfmpegRendererBuilder {
    fn default() -> Self {
        Self {
            codec: "copy".to_string(),
            video_codec: Default::default(),
            pixel_format: Some(PixelFormats::default()),
            framerate: Composition::default().framerate,
            frame_output_format: FrameImageFormat::Bmp,
            out_path: PathBuf::from("out.mp4"),
            tmp_dir_path: PathBuf::from("./out"),
        }
    }
}

impl FfmpegRendererBuilder {
    pub fn build(self) -> FfmpegRenderer {
        FfmpegRenderer {
            codec: self.codec,
            codec_video: self.video_codec,
            pixel_format: self.pixel_format,
            framerate: self.framerate,
            frame_output_format: self.frame_output_format,
            out_path: self.out_path,
            tmp_dir_path: self.tmp_dir_path,
        }
    }

    pub fn video_codec(mut self, video_codec: VideoCodec) -> Self {
        self.video_codec = video_codec;
        self
    }

    pub fn pixel_format(mut self, pixel_format: Option<PixelFormats>) -> Self {
        self.pixel_format = pixel_format;
        self
    }

    pub fn framerate(mut self, framerate: FPS) -> Self {
        self.framerate = framerate;
        self
    }

    pub fn frame_output_format(mut self, frame_output_format: FrameImageFormat) -> Self {
        self.frame_output_format = frame_output_format;
        self
    }

    pub fn out_path(mut self, out_path: impl Into<PathBuf>) -> Self {
        self.out_path = out_path.into();
        self
    }

    pub fn tmp_dir_path(mut self, tmp_dir_path: impl Into<PathBuf>) -> Self {
        self.tmp_dir_path = tmp_dir_path.into();
        self
    }
}
