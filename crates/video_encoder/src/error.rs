use thiserror::Error;

use crate::status::FfmpegSysStatus;

#[derive(Error, Debug)]
pub enum VideoEncoderError {
    #[error("error in ffmpeg: '{message:?}' ({error_code:?})")]
    FfmpegSysError {
        message: &'static str,
        error_code: FfmpegSysStatus,
    },

    #[error("Value must be a multiple of two: '{field:?}'='{value:?}'")]
    ResolutionError { field: &'static str, value: u32 },

    #[error("can't transform '{from:?}' to '{to:?}'")]
    Transform {
        from: &'static str,
        to: &'static str,
    },

    #[error("An error indicating that an interior nul byte was found in `CString::new`")]
    NulError,
}
