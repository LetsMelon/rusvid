use thiserror::Error;

use crate::status::FfmpegSysStatus;

#[derive(Error, Debug)]
pub enum VideoEncoderError {
    #[error("error in ffmpeg: {0:?}")]
    Ffmpeg(FfmpegSysStatus),

    #[error("can't transform '{from:?}' to '{to:?}'")]
    Transform {
        from: &'static str,
        to: &'static str,
    },

    #[error("An error indicating that an interior nul byte was found in `CString::new`")]
    NulError,
}
