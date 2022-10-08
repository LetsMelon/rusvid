use std::ffi::OsString;
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::Result;

use crate::composition::Composition;
use crate::metrics::MetricsVideo;
use crate::renderer::ffmpeg::codec::VideoCodec;
use crate::renderer::ffmpeg::pixel_formats::PixelFormats;
use crate::renderer::frame_image_format::FrameImageFormat;
use crate::renderer::{CliArgument, CliCommand, Renderer};
use crate::types::FPS;

pub mod codec;
pub mod h264;
pub mod pixel_formats;

#[derive(Debug)]
pub struct FfmpegRenderer {
    pub codec: String, // TODO enum
    pub codec_video: VideoCodec,
    pub pixel_format: Option<PixelFormats>,
    pub framerate: FPS,
    frame_output_format: FrameImageFormat,
    out_path: PathBuf,
    tmp_dir_path: PathBuf,
}

impl Default for FfmpegRenderer {
    fn default() -> Self {
        FfmpegRenderer {
            codec: "copy".to_string(),
            codec_video: VideoCodec::default(),
            pixel_format: Some(PixelFormats::default()),
            framerate: Composition::default().framerate,
            frame_output_format: FrameImageFormat::default(),
            out_path: PathBuf::new(),
            tmp_dir_path: PathBuf::new(),
        }
    }
}

impl FfmpegRenderer {
    pub fn new(
        out_path: PathBuf,
        tmp_dir_path: PathBuf,
        frame_output_format: FrameImageFormat,
    ) -> Self {
        FfmpegRenderer {
            out_path,
            tmp_dir_path,
            frame_output_format,
            ..FfmpegRenderer::default()
        }
    }
}

impl Renderer for FfmpegRenderer {
    fn render(&mut self, mut composition: Composition) -> Result<()> {
        self.framerate = composition.framerate;

        let file_extension = self.frame_output_format.file_extension();
        let file_extension_format = self.frame_output_format.as_image_format();
        let out_path = self.out_path().to_path_buf();
        let tmp_path = self.tmp_dir_path().to_path_buf();

        if tmp_path.exists() {
            fs::remove_dir_all(&tmp_path)?;
        }
        fs::create_dir(&tmp_path)?;

        let frames = composition.frames();
        for i in 0..frames {
            // TODO remove hardcoded ":03"
            println!("{:03}/{:03}", i + 1, frames);

            let _ = &composition.update(i)?;

            let file_path = tmp_path.join(format!("{}.{}", i, file_extension));
            let raw_buffer = self.render_rgba_image(&composition)?;

            let _ = raw_buffer.save_with_format(file_path, file_extension_format)?;
        }

        let mut command = self.build_command(&out_path, &tmp_path);
        println!("{:?}", command);

        if out_path.exists() {
            fs::remove_file(&out_path)?;
        }

        command
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()?;
        println!("Saved as: {:?}", &out_path);

        Ok(())
    }

    #[inline]
    fn out_path(&self) -> &Path {
        self.out_path.as_path()
    }

    #[inline]
    fn tmp_dir_path(&self) -> &Path {
        self.tmp_dir_path.as_path()
    }
}

impl CliCommand for FfmpegRenderer {
    fn build_command(&self, out_path: &std::path::Path, _tmp_path: &std::path::Path) -> Command {
        let mut command = Command::new(OsString::from("ffmpeg"));

        command.args([
            OsString::from("-framerate"),
            OsString::from(self.framerate.to_string().as_str()),
            OsString::from("-i"),
            OsString::from(format!(
                "./out/%d.{}",
                self.frame_output_format.file_extension()
            )), // TODO use tmp_path
        ]);

        command.args([OsString::from("-c:a"), OsString::from(self.codec.as_str())]);

        command.args(self.codec_video.build_cli_argument());

        if let Some(pixel_format) = &self.pixel_format {
            command.args(pixel_format.build_cli_argument());
        }

        command.args([
            OsString::from("-r"),
            OsString::from(self.framerate.to_string().as_str()),
        ]);

        command.arg(OsString::from(out_path));

        command
    }
}
