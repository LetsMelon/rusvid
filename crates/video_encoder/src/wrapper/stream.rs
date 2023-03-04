use std::ptr;

use anyhow::{bail, Result};
use ffmpeg_sys_next::{avcodec_parameters_from_context, avformat_new_stream, AVRational, AVStream};

use super::codec_context::CodecContext;
use super::format_context::FormatContext;
use super::WrapperType;
use crate::status::FfmpegSysStatus;

pub struct Stream(*mut AVStream);

impl Stream {
    pub fn new(format_context: &mut FormatContext) -> Result<Self> {
        let video_st = unsafe { avformat_new_stream(format_context.get_inner_mut(), ptr::null()) };

        if video_st.is_null() {
            bail!("Failed to allocate the video stream.");
        }

        unsafe {
            (*video_st).id = ((*format_context.get_inner()).nb_streams - 1) as i32;
        };

        Ok(Stream(video_st))
    }

    pub fn set_time_base(&mut self, time_base: AVRational) {
        unsafe { (*self.get_inner_mut()).time_base = time_base }
    }

    pub fn get_time_base(&self) -> AVRational {
        unsafe { (*self.get_inner()).time_base }
    }

    pub fn set_context(&mut self, context: &CodecContext) -> Result<()> {
        let err =
            unsafe { avcodec_parameters_from_context((*self.0).codecpar, context.get_inner()) };

        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_error() {
            // TODO throw custom error
            dbg!(&status);
            bail!("Failed to set codec parameters.");
        }

        Ok(())
    }
}

impl WrapperType for Stream {
    type OUT = AVStream;

    fn get_inner(&self) -> *const Self::OUT {
        self.0
    }

    fn get_inner_mut(&mut self) -> *mut Self::OUT {
        self.0
    }
}
