use anyhow::Result;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

pub fn build_command(tmp_path: &Path, out_path: &Path, framerate: u8) -> Result<Command> {
    let mut command = Command::new(OsStr::new("ffmpeg"));
    command.args([
        OsStr::new("-framerate"),
        OsStr::new(&(framerate as usize).to_string()),
        // OsStr::new("-pattern_type"),
        // OsStr::new("glob"),
        OsStr::new("-i"),
        OsStr::new("./out/%d.png"), // TODO use tmp_path
        OsStr::new("-c:v"),
        OsStr::new("libx264"),
        OsStr::new("-pix_fmt"),
        OsStr::new("yuv420p"),
        OsStr::new(out_path),
    ]);

    Ok(command)
}
