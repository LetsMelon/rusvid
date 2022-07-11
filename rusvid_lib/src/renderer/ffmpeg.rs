use anyhow::Result;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::rc::Rc;
use usvg::PathData;

use crate::composition::Composition;
use crate::metrics::MetricsVideo;
use crate::renderer::Renderer;

pub fn build_command(_tmp_path: &Path, out_path: &Path, framerate: u8) -> Result<Command> {
    let mut command = Command::new(OsStr::new("ffmpeg"));
    command.args([
        OsStr::new("-framerate"),
        OsStr::new(&(framerate as usize).to_string()),
        OsStr::new("-r"),
        OsStr::new(&(framerate as usize).to_string()),
        OsStr::new("-i"),
        OsStr::new("./out/%d.png"), // TODO use tmp_path
        OsStr::new("-c:a"),
        OsStr::new("copy"),
        OsStr::new("-c:v"),
        OsStr::new("libx264"),
        OsStr::new("-crf"),
        OsStr::new("1"),
        OsStr::new("-pix_fmt"),
        OsStr::new("yuv420p"),
        OsStr::new(out_path),
        // ffmpeg -r 60 -i out/%d.png -vcodec libx264 -crf 1 -pix_fmt yuv420p test_crf1.mp4
    ]);

    Ok(command)
}

pub struct FFmpegRenderer {}

impl Default for FFmpegRenderer {
    fn default() -> Self {
        FFmpegRenderer {}
    }
}

impl Renderer for FFmpegRenderer {
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

            let filename = format!("{}.png", i + 1);
            let file_path = tmp_path.join(Path::new(&filename));
            composition.render_single(file_path.as_path())?;

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

        let mut command = build_command(&tmp_path, &out_path, composition.framerate)?;

        if out_path.exists() {
            fs::remove_file(&out_path)?;
        }

        command.output()?;
        println!("Saved as: {:?}", out_path);

        Ok(())
    }
}
