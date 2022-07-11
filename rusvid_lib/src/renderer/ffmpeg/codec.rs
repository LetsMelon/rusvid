use crate::renderer::ffmpeg::h264::H264Settings;
use crate::renderer::CliArgument;
use std::ffi::OsString;

#[derive(Debug)]
#[repr(u8)]
pub enum VideoCodec {
    Libx264(H264Settings),
}

impl ToString for VideoCodec {
    fn to_string(&self) -> String {
        match self {
            VideoCodec::Libx264(_) => "libx264".to_string(),
        }
    }
}

impl Default for VideoCodec {
    fn default() -> Self {
        Self::Libx264(H264Settings::default())
    }
}

impl CliArgument for VideoCodec {
    #[inline(always)]
    fn build_cli_argument(&self) -> Vec<OsString> {
        let mut arguments = Vec::new();
        match self {
            VideoCodec::Libx264(config) => {
                arguments.push(OsString::from("-c:v"));
                arguments.push(OsString::from(self.to_string()));
                arguments.extend_from_slice(&config.build_cli_argument());

                arguments
            }
        }
    }
}
