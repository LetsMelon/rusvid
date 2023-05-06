use ffmpeg_sys_next::{
    av_frame_alloc, av_frame_free, av_image_fill_arrays, av_image_get_buffer_size, AVFrame,
    AVPixelFormat,
};

use crate::error::VideoEncoderError;
use crate::status::FfmpegSysStatus;
use crate::wrapper::WrapperType;

pub struct Frame(*mut AVFrame);

impl Frame {
    pub fn new(pix_fmt: AVPixelFormat) -> Result<Self, VideoEncoderError> {
        unsafe {
            let frame = av_frame_alloc();
            if frame.is_null() {
                return Err(VideoEncoderError::FfmpegSysError {
                    message: "Could not allocate the video frame.",
                    error_code: FfmpegSysStatus::Unknown,
                });
            }

            (*frame).format = pix_fmt as i32;

            Ok(Frame(frame))
        }
    }

    pub fn set_resolution(&mut self, resolution: (u32, u32)) {
        unsafe {
            (*self.0).width = resolution.0 as i32;
            (*self.0).height = resolution.1 as i32;
        }
    }

    pub fn set_pts(&mut self, value: i64) {
        unsafe { (*self.0).pts = value }
    }

    pub fn get_pts(&self) -> i64 {
        unsafe { (*self.0).pts }
    }

    pub fn add_pts(&mut self, value: i64) {
        self.set_pts(self.get_pts() + value);
    }

    pub fn get_raw_buffer(
        &self,
        pix_fmt: AVPixelFormat,
        target: (u32, u32),
    ) -> Result<Vec<u8>, VideoEncoderError> {
        let nframe_bytes =
            unsafe { av_image_get_buffer_size(pix_fmt, target.0 as i32, target.1 as i32, 16) };

        let status = FfmpegSysStatus::from_ffmpeg_sys_error(nframe_bytes);
        if status.is_error() {
            return Err(VideoEncoderError::FfmpegSysError {
                message: "Error in `av_image_get_buffer_size`",
                error_code: status,
            });
        }

        Ok(vec![0; nframe_bytes as usize])
    }

    pub fn fill_array(
        &mut self,
        frame_buf: &Vec<u8>,
        pix_fmt: AVPixelFormat,
        resolution: (u32, u32),
    ) -> Result<(), VideoEncoderError> {
        let err = unsafe {
            av_image_fill_arrays(
                (*self.0).data.as_mut_ptr(),
                (*self.0).linesize.as_mut_ptr(),
                frame_buf.as_ptr(),
                pix_fmt,
                resolution.0 as i32,
                resolution.1 as i32,
                1,
            )
        };

        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_error() {
            let err = VideoEncoderError::FfmpegSysError {
                message: "error in `av_image_fill_arrays`",
                error_code: status,
            };
            println!("{err:?}");
        }

        Ok(())
    }
}

impl WrapperType for Frame {
    type OUT = AVFrame;

    fn get_inner(&self) -> *const Self::OUT {
        self.0
    }

    fn get_inner_mut(&mut self) -> *mut Self::OUT {
        self.0
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe { av_frame_free(&mut self.0) }
    }
}
