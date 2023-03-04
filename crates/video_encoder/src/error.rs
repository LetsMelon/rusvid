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

    #[error("The selected output container does not support video encoding.")]
    ContainerNoVideoEncoding,

    #[error("Codec not found.")]
    CodecNotFound,

    #[error("Could not allocate the video frame.")]
    VideoFrameAllocation,

    #[error("Could not allocate the video stream.")]
    VideoStreamAllocation,

    #[error("Error in creating a scale context.")]
    ScaleContextAllocation,

    #[error("Could not allocate video codec context.")]
    VideoCodecContextAllocation,

    #[error("Error in getting cached scale context.")]
    ScaleContextCached,

    #[error("Unable to create the output context. Maybe the logs can say more about where the error has happened")]
    OutputContextAllocation,

    #[error("can't transform '{from:?}' to '{to:?}'")]
    Transform {
        from: &'static str,
        to: &'static str,
    },

    #[error("An error indicating that an interior nul byte was found in `CString::new`")]
    NulError,
}
