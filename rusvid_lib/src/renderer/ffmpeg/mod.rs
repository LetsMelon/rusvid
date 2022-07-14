use anyhow::Result;
use debug_ignore::DebugIgnore;
use std::ffi::OsString;
use std::fmt::Debug;
use std::fs;
use std::ops::Deref;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::rc::Rc;
use usvg::PathData;

use crate::composition::Composition;
use crate::metrics::MetricsVideo;
use crate::renderer::ffmpeg::codec::VideoCodec;
use crate::renderer::ffmpeg::pixel_formats::PixelFormats;
use crate::renderer::png::PngRender;
use crate::renderer::{CliArgument, CliCommand, ImageRender, Renderer};

pub mod codec;
pub mod h264;
pub mod pixel_formats;

#[derive(Debug)]
pub struct FfmpegRenderer {
    pub codec: String, // TODO enum
    pub codec_video: VideoCodec,
    pub pixel_format: Option<PixelFormats>,
    pub framerate: u8,
    pub image_render: DebugIgnore<Box<dyn ImageRender>>,
}

impl Default for FfmpegRenderer {
    fn default() -> Self {
        FfmpegRenderer {
            codec: "copy".to_string(),
            codec_video: VideoCodec::default(),
            pixel_format: Some(PixelFormats::default()),
            framerate: Composition::default().framerate,
            image_render: DebugIgnore(Box::new(PngRender::new(&PathBuf::new()))),
        }
    }
}

impl FfmpegRenderer {
    fn image_render(&self) -> &Box<dyn ImageRender> {
        self.image_render.deref()
    }

    pub fn set_image_render(&mut self, image_render: Box<dyn ImageRender>) {
        self.image_render = DebugIgnore(image_render);
    }
}

impl Renderer for FfmpegRenderer {
    fn render<P: Into<PathBuf>>(
        &self,
        composition: Composition,
        out_path: P,
        tmp_path: P,
        mut position: Rc<PathData>,
    ) -> Result<()> {
        let out_path: PathBuf = out_path.into();
        let tmp_path: PathBuf = tmp_path.into();

        if tmp_path.exists() {
            fs::remove_dir_all(&tmp_path)?;
        }
        fs::create_dir(&tmp_path)?;

        let frames = composition.frames();
        for i in 0..frames {
            println!("{:03}/{:03}", i + 1, frames);

            // image_render.render(&composition, i + 1)?;
            self.image_render().render(&composition, i);

            // TODO: make safe
            // Test 1:
            // let mut reference_position = box_position.borrow_mut();
            // reference_position.transform(UsvgTransform::new_translate(5.0, 4.0));
            unsafe {
                let pd = Rc::get_mut_unchecked(&mut position);
                pd.transform(usvg::Transform::new_translate(5.0, 4.0));
                pd.transform(usvg::Transform::new_rotate(65.0 / (frames as f64)));
            }
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
        println!("Saved as: {:?}", out_path);

        Ok(())
    }
}

impl CliCommand for FfmpegRenderer {
    fn build_command(&self, out_path: &std::path::Path, _tmp_path: &std::path::Path) -> Command {
        let mut command = Command::new(OsString::from("ffmpeg"));

        command.args([
            OsString::from("-framerate"),
            OsString::from(self.framerate.to_string().as_str()),
            OsString::from("-i"),
            OsString::from(format!("./out/%d.{}", self.image_render().file_extension())), // TODO use tmp_path
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