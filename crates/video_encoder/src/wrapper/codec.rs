use std::ptr;

use ffmpeg_sys_next::{avcodec_find_encoder, avcodec_open2, AVCodec, AVCodecID};

use crate::error::VideoEncoderError;
use crate::status::FfmpegSysStatus;
use crate::wrapper::codec_context::CodecContext;
use crate::wrapper::format_context::FormatContext;
use crate::wrapper::WrapperType;

pub struct Codec(*const AVCodec);

impl Codec {
    pub fn new(format_context: &FormatContext) -> Result<Self, VideoEncoderError> {
        unsafe {
            let fmt = (*format_context.get_inner()).oformat;
            if (*fmt).video_codec == AVCodecID::AV_CODEC_ID_NONE {
                return Err(VideoEncoderError::FfmpegSysError {
                    message: "The selected output container does not support video encoding.",
                    error_code: FfmpegSysStatus::Unknown,
                });
            }

            let codec = avcodec_find_encoder((*fmt).video_codec);
            if codec.is_null() {
                return Err(VideoEncoderError::FfmpegSysError {
                    message: "Codec not found.",
                    error_code: FfmpegSysStatus::Unknown,
                });
            }

            Ok(Codec(codec))
        }
    }

    pub fn open_codec(&self, context: &mut CodecContext) -> Result<(), VideoEncoderError> {
        let err =
            unsafe { avcodec_open2(context.get_inner_mut(), self.get_inner(), ptr::null_mut()) };

        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_error() {
            return Err(VideoEncoderError::FfmpegSysError {
                message: "Could not open the codec.",
                error_code: status,
            });
        }

        Ok(())
    }
}

impl WrapperType for Codec {
    type OUT = AVCodec;

    fn get_inner(&self) -> *const Self::OUT {
        self.0
    }

    fn get_inner_mut(&mut self) -> *mut Self::OUT {
        self.0 as *mut Self::OUT
    }
}
