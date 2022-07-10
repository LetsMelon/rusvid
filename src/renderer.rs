use anyhow::Result;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

pub fn build_command(tmp_path: &Path, out_path: &Path, framerate: u8) -> Result<Command> {
    // I have no idea from ffmpeg
    // https://stackoverflow.com/questions/39887869/ffmpeg-whatsapp-video-format-not-supported
    // https://stackoverflow.com/questions/3561715/using-ffmpeg-to-encode-a-high-quality-video

    let mut command = Command::new(OsStr::new("ffmpeg"));
    command.args([
        OsStr::new("-framerate"),
        OsStr::new(&(framerate as usize).to_string()),
        // OsStr::new("-pattern_type"),
        // OsStr::new("glob"),
        OsStr::new("-i"),
        OsStr::new("./out/%d.png"), // TODO use tmp_path
        OsStr::new("-f"),
        OsStr::new("mp4"),
        OsStr::new("-q:v"),
        OsStr::new("0"),
        OsStr::new("-c:a"),
        OsStr::new("copy"),
        OsStr::new("-c:v"),
        OsStr::new("libx264"),
        OsStr::new("-profile:v"),
        OsStr::new("baseline"),
        OsStr::new("-level"),
        OsStr::new("3.0"),
        OsStr::new("-pix_fmt"),
        OsStr::new("yuv420p"),
        OsStr::new(out_path),
    ]);

    Ok(command)
}
