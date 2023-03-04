use std::ptr;

use anyhow::{bail, Result};
use ffmpeg_sys_next::{
    sws_freeContext, sws_getCachedContext, sws_getContext, sws_scale, AVPixelFormat, SwsContext,
};

use super::{Frame, WrapperType};
use crate::status::FfmpegSysStatus;
use crate::{PIX_FMT, SCALE_FLAGS};

pub struct ScaleContext {
    target_resolution: (u32, u32),

    raw: *mut SwsContext,
}

impl ScaleContext {
    pub fn new(src_resolution: (u32, u32), dst_resolution: (u32, u32)) -> Result<Self> {
        let scale_context = unsafe {
            sws_getContext(
                src_resolution.0 as i32,
                src_resolution.1 as i32,
                AVPixelFormat::AV_PIX_FMT_RGB24,
                dst_resolution.0 as i32,
                dst_resolution.1 as i32,
                PIX_FMT,
                SCALE_FLAGS,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        if scale_context.is_null() {
            bail!("Error in creating a scale context.")
        }

        Ok(ScaleContext {
            target_resolution: src_resolution,
            raw: scale_context,
        })
    }

    pub fn get_cached_context(&mut self, src_resolution: (u32, u32)) -> Result<()> {
        let scale_context = unsafe {
            sws_getCachedContext(
                self.get_inner_mut(),
                src_resolution.0 as i32,
                src_resolution.1 as i32,
                AVPixelFormat::AV_PIX_FMT_RGBA,
                self.target_resolution.0 as i32,
                self.target_resolution.1 as i32,
                PIX_FMT,
                SCALE_FLAGS,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };

        if scale_context.is_null() {
            bail!("Error in getting cached scale context.");
        }

        self.raw = scale_context;

        Ok(())
    }

    pub fn scale_frames(
        &mut self,
        src_frame: &Frame,
        src_height: u32,
        dst_frame: &mut Frame,
    ) -> Result<()> {
        let err = unsafe {
            sws_scale(
                self.get_inner_mut(),
                &(*src_frame.get_inner()).data[0] as *const *mut u8 as *const *const u8,
                &(*src_frame.get_inner()).linesize[0],
                0,
                src_height as i32,
                &(*dst_frame.get_inner_mut()).data[0] as *const *mut u8,
                &(*dst_frame.get_inner_mut()).linesize[0],
            )
        };
        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_error() {
            // TODO throw custom error
            dbg!(&status);
        }

        Ok(())
    }
}

impl WrapperType for ScaleContext {
    type OUT = SwsContext;

    fn get_inner(&self) -> *const Self::OUT {
        self.raw
    }

    fn get_inner_mut(&mut self) -> *mut Self::OUT {
        self.raw
    }
}

impl Drop for ScaleContext {
    fn drop(&mut self) {
        unsafe {
            sws_freeContext(self.raw);
        }
    }
}
