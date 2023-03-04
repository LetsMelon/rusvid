use thiserror::Error;

use crate::status::FfmpegSysStatus;

#[derive(Error, Debug)]
pub enum VideoEncoderError {
    #[error("error in ffmpeg: '{message:?}' ({error_code:?})")]
    FfmpegSysError {
        message: &'static str,
        error_code: FfmpegSysStatus,
    },

    #[error("The selected output container does not support video encoding.")]
    ContainerNoVideoEncoding,

    #[error("Codec not found.")]
    CodecNotFound,

    #[error("Could not allocate the video frame.")]
    VideoFrameAllocation,

    #[error("can't transform '{from:?}' to '{to:?}'")]
    Transform {
        from: &'static str,
        to: &'static str,
    },

    #[error("An error indicating that an interior nul byte was found in `CString::new`")]
    NulError,
}
