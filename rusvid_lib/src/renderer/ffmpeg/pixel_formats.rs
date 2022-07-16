use std::ffi::OsString;

use crate::renderer::CliArgument;

#[derive(Debug)]
#[repr(u8)]
pub enum PixelFormats {
    Yuv420p,
}

impl Default for PixelFormats {
    fn default() -> Self {
        PixelFormats::Yuv420p
    }
}

impl ToString for PixelFormats {
    fn to_string(&self) -> String {
        match self {
            PixelFormats::Yuv420p => "yuv420p".to_string(),
        }
    }
}

impl CliArgument for PixelFormats {
    #[inline(always)]
    fn build_cli_argument(&self) -> Vec<OsString> {
        vec![
            OsString::from("-pix_fmt"),
            OsString::from(&self.to_string()),
        ]
    }
}
