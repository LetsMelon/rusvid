use core::ptr;
use std::ffi::CString;
use std::path::PathBuf;

use anyhow::{bail, Result};
use ffmpeg_sys_next::{
    av_dump_format, av_write_trailer, avformat_alloc_output_context2, avformat_free_context,
    avformat_write_header, avio_open, AVFormatContext,
};

use super::WrapperType;
use crate::status::FfmpegSysStatus;
use crate::util::pathbuf_to_cstring;

pub struct FormatContext {
    out_path: CString,

    raw: *mut AVFormatContext,
}

impl FormatContext {
    pub fn new(out_path: PathBuf) -> Result<Self> {
        let path_str = pathbuf_to_cstring(&out_path)?;

        let mut fmt = ptr::null_mut();

        let err = unsafe {
            avformat_alloc_output_context2(
                &mut fmt,
                ptr::null_mut(),
                ptr::null(),
                path_str.as_ptr(),
            )
        };
        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);

        if status.is_error() {
            // TODO log the error but don't return early.
            dbg!(&status);
        }

        if fmt.is_null() {
            let mpeg = CString::new(&b"mpeg"[..]).unwrap();

            let err = unsafe {
                avformat_alloc_output_context2(
                    &mut fmt,
                    ptr::null_mut(),
                    mpeg.as_ptr(),
                    path_str.as_ptr(),
                )
            };
            let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
            if status.is_error() {
                // TODO log the error but don't return early.
                dbg!(&status);
            }
        }

        if fmt.is_null() {
            bail!("Unable to create the output context. Maybe the logs can say more about where the error has happened");
        }

        Ok(FormatContext {
            out_path: path_str,
            raw: fmt,
        })
    }

    pub fn print_format(&self) {
        // TODO capture (=output as string/&str) stdout from ffi call to log it
        unsafe {
            av_dump_format(
                self.get_inner() as *mut AVFormatContext,
                0,
                self.out_path.as_ptr(),
                1,
            )
        }
    }

    pub fn open_output_file(&mut self) -> Result<()> {
        let err = unsafe { avio_open(&mut (*self.get_inner_mut()).pb, self.out_path.as_ptr(), 2) };
        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_error() {
            dbg!(&status);
            bail!("Failed to open the output media file.");
        }

        let err = unsafe { avformat_write_header(self.get_inner_mut(), ptr::null_mut()) };
        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_error() {
            dbg!(&status);
            bail!("Failed to write the stream header to the output media file.")
        }

        Ok(())
    }

    pub fn write_trailer(&mut self) -> Result<()> {
        let err = unsafe { av_write_trailer(self.get_inner_mut()) };
        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_error() {
            dbg!(&status);
            bail!("Error writing trailer.");
        }

        Ok(())
    }
}

impl WrapperType for FormatContext {
    type OUT = AVFormatContext;

    fn get_inner(&self) -> *const Self::OUT {
        self.raw
    }

    fn get_inner_mut(&mut self) -> *mut Self::OUT {
        self.raw
    }
}

impl Drop for FormatContext {
    fn drop(&mut self) {
        unsafe {
            if ffmpeg_sys_next::avio_closep(&mut (*self.raw).pb) < 0 {
                println!("Warning: failed closing output file");
            }
            avformat_free_context(self.get_inner_mut());
        }
    }
}
