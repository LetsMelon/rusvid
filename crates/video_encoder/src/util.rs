use std::ffi::CString;
use std::path::Path;

use ffmpeg_sys_next::{av_rescale_q, AVRational};

use crate::error::VideoEncoderError;

#[inline]
/// Rescales a 64-bit integer by 2 rational numbers.
pub fn rescale_q(a: i64, bq: AVRational, cq: AVRational) -> i64 {
    unsafe { av_rescale_q(a, bq, cq) }
}

#[inline]
/// Transform a [`Path`] into an Result with an [`CString`] or an [`VideoEncoderError`]
///
/// # Errors
/// - [`VideoEncoderError::Transform`]
/// Is thrown if `path` isn't valid unicode.
///
/// - [`VideoEncoderError::NulError`]
/// This function will return an error if the supplied bytes contain an internal 0 byte.
/// The NulError returned will contain the bytes as well as the position of the nul byte.
/// For more info see [`CString::new`](alloc::ffi::CString).
pub fn path_to_cstring(path: &Path) -> Result<CString, VideoEncoderError> {
    let path_str = path.to_str().ok_or_else(|| VideoEncoderError::Transform {
        from: "&std::path::Path",
        to: "&str",
    })?;

    CString::new(path_str).map_err(|_| VideoEncoderError::NulError)
}
