use std::path::PathBuf;

use anyhow::{bail, Result};
use ffmpeg_sys_next::{AVPixelFormat, SWS_BICUBIC};
use rusvid_core::plane::Plane;

pub mod error;
pub mod status;
mod util;
mod wrapper;

use wrapper::*;

use crate::util::rescale_q;

pub(crate) const PIX_FMT: AVPixelFormat = AVPixelFormat::AV_PIX_FMT_YUV420P;
pub(crate) const SCALE_FLAGS: i32 = SWS_BICUBIC;

pub struct Encoder {
    format_context: FormatContext,
    video_st: Stream,
    context: CodecContext,
    frame: Frame,
    tmp_frame: Frame,
    scale_context: ScaleContext,

    tmp_frame_buf: Vec<u8>,
    current_frame_index: usize,
}

// TODO use custom errors
impl Encoder {
    // TODO replace fps's type with FPS type from `rusvid_lib`
    pub fn new(path: impl Into<PathBuf>, resolution: (u32, u32), fps: usize) -> Result<Encoder> {
        let bit_rate = (resolution.0 * resolution.1 * 4 * (fps as u32)) as usize;
        let time_base = (1_000, fps * 1_000);

        if resolution.0 % 2 != 0 || resolution.1 % 2 != 0 {
            bail!("width and/or height must be a multiple of two");
        }

        let mut format_context = FormatContext::new(path.into())?;

        let codec = Codec::new(&format_context)?;

        let mut video_st = Stream::new(&mut format_context)?;

        let mut context = CodecContext::new(
            &codec,
            &format_context,
            bit_rate,
            resolution,
            time_base,
            1,
            PIX_FMT,
        )?;

        video_st.set_time_base(context.get_time_base());

        // TODO create wrapper for SwsContext
        let scale_context = ScaleContext::new(resolution, resolution)?;

        codec.open_codec(&mut context)?;

        debug_assert_eq!(context.get_pix_fmt(), PIX_FMT);
        let mut frame = Frame::new(PIX_FMT)?;
        frame.set_resolution(resolution);
        frame.set_pts(0);

        let frame_buf = frame.get_raw_buffer(PIX_FMT, resolution)?;

        frame.fill_array(&frame_buf, PIX_FMT, resolution)?;

        let tmp_frame = Frame::new(PIX_FMT)?;

        video_st.set_context(&context)?;

        format_context.print_format();

        format_context.open_output_file()?;

        Ok(Encoder {
            format_context,
            video_st,
            context,
            frame,
            tmp_frame,
            scale_context,

            current_frame_index: 0,
            tmp_frame_buf: Vec::new(),
        })
    }

    pub fn encode_plane(&mut self, plane: Plane) -> Result<()> {
        let width = plane.width();
        let height = plane.height();
        let data = plane.as_data_flatten();

        self.encode(width, height, &data)
    }

    fn encode(&mut self, width: u32, height: u32, data: &[u8]) -> Result<()> {
        let new_buffer_length = (width * height * 4) as usize;
        debug_assert!(data.len() == new_buffer_length);

        if self.tmp_frame_buf.len() != new_buffer_length {
            self.tmp_frame_buf.resize(new_buffer_length, 0);
        }
        self.tmp_frame_buf.clone_from_slice(data);

        self.tmp_frame.set_resolution((width, height));

        self.tmp_frame.fill_array(
            &self.tmp_frame_buf,
            AVPixelFormat::AV_PIX_FMT_RGBA,
            (width, height),
        )?;

        // Convert the snapshot frame to the right format for the destination frame.
        self.scale_context.get_cached_context((width, height))?;
        self.scale_context
            .scale_frames(&self.tmp_frame, height, &mut self.frame)?;

        // Encode the image.
        let packet = Packet::new();
        self.context
            .send_frame(&self.frame, &mut self.format_context, packet)?;

        let value = rescale_q(
            1,
            self.context.get_time_base(),
            self.video_st.get_time_base(),
        );
        self.frame.add_pts(value);

        self.current_frame_index += self.current_frame_index;

        Ok(())
    }

    pub fn finish_stream(mut self) -> Result<()> {
        self.context.send_stream_eof(&mut self.format_context)?;

        self.format_context.write_trailer()?;

        // ? Drop the encoder to free the memory at this point in the code
        drop(self);

        Ok(())
    }
}
