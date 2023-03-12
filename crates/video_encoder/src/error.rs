use thiserror::Error;

use crate::status::FfmpegSysStatus;

#[derive(Error, Debug)]
pub enum VideoEncoderError {
    #[error("error in ffmpeg: '{message:?}' ({error_code:?})")]
    /// [`VideoEncoderError`] can be this variant if the error appeared in ffmpeg specific code.
    ///
    /// For more info see [`FfmpegSysStatus`] and/or [`ffmpeg docs`](https://ffmpeg.org/doxygen/3.3/group__lavu__error.html).
    FfmpegSysError {
        message: &'static str,
        error_code: FfmpegSysStatus,
    },

    #[error("Value must be a multiple of two: '{field:?}'='{value:?}'")]
    /// [`VideoEncoderError`] can be this variant if the `width` and/or `height` of a given resolution isn't a multiple of two.
    ///
    /// For more info see the error message.
    ResolutionError { field: &'static str, value: u32 },

    #[error("can't transform '{from:?}' to '{to:?}'")]
    /// [`VideoEncoderError`] can be this variant if a type can't be transformed into another one.
    ///
    /// For more info see the error message.
    Transform {
        from: &'static str,
        to: &'static str,
    },

    #[error("An error indicating that an interior nul byte was found in `CString::new`")]
    /// [`VideoEncoderError`] can be this variant if the error occurred in [`CString::new`](alloc::ffi::CString).
    ///
    /// For more info see official rust docs about [`CString::new`](alloc::ffi::CString).
    NulError,
}
