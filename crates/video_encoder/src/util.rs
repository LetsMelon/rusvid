use std::ffi::CString;
use std::path::Path;

use anyhow::Result;
use ffmpeg_sys_next::{av_rescale_q, AVRational};

#[inline]
/// Rescales a 64-bit integer by 2 rational numbers.
pub fn rescale_q(a: i64, bq: AVRational, cq: AVRational) -> i64 {
    unsafe { av_rescale_q(a, bq, cq) }
}

pub fn pathbuf_to_cstring(path: &Path) -> Result<CString> {
    let path_str = path
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Error while transforming `PathBuf` to `&str`"))?;

    let c_string = CString::new(path_str)?;

    Ok(c_string)
}
