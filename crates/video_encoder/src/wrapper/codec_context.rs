use std::ffi::CString;
use std::ptr;

use ffmpeg_sys_next::{
    av_interleaved_write_frame, av_opt_set, avcodec_alloc_context3, avcodec_free_context,
    avcodec_receive_packet, avcodec_send_frame, AVCodecContext, AVCodecID, AVPixelFormat,
    AVRational,
};

use crate::error::VideoEncoderError;
use crate::status::FfmpegSysStatus;
use crate::wrapper::codec::Codec;
use crate::wrapper::format_context::FormatContext;
use crate::wrapper::{Frame, Packet, WrapperType};

// See https://ffmpeg.org/ffmpeg-codecs.html#Options-34
pub enum CodecContextOption<'a> {
    Crf(f32),
    Preset(&'a str),
}

impl<'a> CodecContextOption<'a> {
    fn get_name(&self) -> Result<CString, VideoEncoderError> {
        match self {
            CodecContextOption::Crf(_) => {
                CString::new("crf").map_err(|_| VideoEncoderError::NulError)
            }
            CodecContextOption::Preset(_) => {
                CString::new("preset").map_err(|_| VideoEncoderError::NulError)
            }
        }
    }

    fn get_value(&self) -> Result<CString, VideoEncoderError> {
        match self {
            CodecContextOption::Crf(crf) => {
                CString::new(crf.to_string()).map_err(|_| VideoEncoderError::NulError)
            }
            CodecContextOption::Preset(preset) => {
                CString::new(*preset).map_err(|_| VideoEncoderError::NulError)
            }
        }
    }
}

pub struct CodecContext(*mut AVCodecContext);

impl CodecContext {
    pub fn new(
        codec: &Codec,
        format_context: &FormatContext,
        bit_rate: usize,
        target_resolution: (u32, u32),
        time_base: (usize, usize),
        max_b_frames: usize,
        pix_fmt: AVPixelFormat,
    ) -> Result<Self, VideoEncoderError> {
        let context = unsafe { avcodec_alloc_context3(codec.get_inner()) };
        if context.is_null() {
            return Err(VideoEncoderError::FfmpegSysError {
                message: "Could not allocate video codec context.",
                error_code: FfmpegSysStatus::Unknown,
            });
        }

        unsafe {
            (*context).codec_id = (*(*format_context.get_inner()).oformat).video_codec;
            if (*context).codec_id == AVCodecID::AV_CODEC_ID_MPEG1VIDEO {
                (*context).mb_decision = 2;
            }

            (*context).bit_rate = bit_rate as i64;

            (*context).width = target_resolution.0 as i32;
            (*context).height = target_resolution.1 as i32;

            (*context).time_base = AVRational {
                num: time_base.0 as i32,
                den: time_base.1 as i32,
            };

            (*context).gop_size = 10;
            (*context).max_b_frames = max_b_frames as i32;
            (*context).pix_fmt = pix_fmt;
        };

        Ok(CodecContext(context))
    }

    pub fn set_object_with_value(
        &mut self,
        codec_context_option: CodecContextOption,
    ) -> Result<(), VideoEncoderError> {
        let name = codec_context_option.get_name()?;
        let value = codec_context_option.get_value()?;

        let err = unsafe {
            av_opt_set(
                (*self.get_inner_mut()).priv_data,
                name.as_ptr(),
                value.as_ptr(),
                0,
            )
        };
        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_error() {
            return Err(VideoEncoderError::FfmpegSysError {
                message: "Error in setting option for codec context",
                error_code: status,
            });
        }

        Ok(())
    }

    pub fn send_frame(
        &mut self,
        frame: &Frame,
        format_context: &mut FormatContext,
        mut packet: Packet,
    ) -> Result<(), VideoEncoderError> {
        let err = unsafe { avcodec_send_frame(self.get_inner_mut(), frame.get_inner()) };
        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_error() {
            return Err(VideoEncoderError::FfmpegSysError {
                message: "Error encoding frame.",
                error_code: status,
            });
        }

        let err = unsafe { avcodec_receive_packet(self.get_inner_mut(), packet.get_inner_mut()) };
        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_ok() {
            let err = unsafe {
                av_interleaved_write_frame(format_context.get_inner_mut(), packet.get_inner_mut())
            };
            let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
            if status.is_error() {
                dbg!(&status);
            }

            packet.unref();
        }

        Ok(())
    }

    pub fn send_stream_eof(
        &mut self,
        format_context: &mut FormatContext,
    ) -> Result<(), VideoEncoderError> {
        let err = unsafe { avcodec_send_frame(self.get_inner_mut(), ptr::null()) };
        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_error() {
            return Err(VideoEncoderError::FfmpegSysError {
                message: "Error encoding frame.",
                error_code: status,
            });
        }

        loop {
            let mut packet = Packet::new();

            let err =
                unsafe { avcodec_receive_packet(self.get_inner_mut(), packet.get_inner_mut()) };
            let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
            match status {
                FfmpegSysStatus::NoError => unsafe {
                    av_interleaved_write_frame(
                        format_context.get_inner_mut(),
                        packet.get_inner_mut(),
                    );
                },
                FfmpegSysStatus::Eof => break,
                _ => {}
            }
        }

        Ok(())
    }

    pub fn get_time_base(&self) -> AVRational {
        unsafe { (*self.0).time_base }
    }

    pub fn get_pix_fmt(&self) -> AVPixelFormat {
        unsafe { (*self.0).pix_fmt }
    }
}

impl WrapperType for CodecContext {
    type OUT = AVCodecContext;

    fn get_inner(&self) -> *const Self::OUT {
        self.0
    }

    fn get_inner_mut(&mut self) -> *mut Self::OUT {
        self.0
    }
}

impl Drop for CodecContext {
    fn drop(&mut self) {
        unsafe { avcodec_free_context(&mut self.0) }
    }
}
