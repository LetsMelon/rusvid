use std::ptr;

use anyhow::{bail, Result};
use ffmpeg_sys_next::{avcodec_find_encoder, avcodec_open2, AVCodec, AVCodecID};

use super::codec_context::CodecContext;
use super::format_context::FormatContext;
use super::WrapperType;
use crate::status::FfmpegSysStatus;

pub struct Codec(*const AVCodec);

impl Codec {
    pub fn new(format_context: &FormatContext) -> Result<Self> {
        unsafe {
            let fmt = (*format_context.get_inner()).oformat;
            if (*fmt).video_codec == AVCodecID::AV_CODEC_ID_NONE {
                bail!("The selected output container does not support video encoding.")
            }

            let codec = avcodec_find_encoder((*fmt).video_codec);
            if codec.is_null() {
                bail!("Codec not found.");
            }

            Ok(Codec(codec))
        }
    }

    pub fn open_codec(&self, context: &mut CodecContext) -> Result<()> {
        let err =
            unsafe { avcodec_open2(context.get_inner_mut(), self.get_inner(), ptr::null_mut()) };

        let status = FfmpegSysStatus::from_ffmpeg_sys_error(err);
        if status.is_error() {
            dbg!(&status);
            bail!("Could not open the codec.");
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
