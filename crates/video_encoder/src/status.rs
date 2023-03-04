#[derive(Debug)]
/// Copied from [ffmpeg docs](https://ffmpeg.org/doxygen/2.5/error_8c.html)
pub enum FfmpegSysStatus {
    NoError,
    /// Bitstream filter not found.
    BsfNotFound,
    /// Buffer too small.
    BufferTooSmall,
    /// Internal bug, also see AVERROR_BUG2.
    Bug,
    /// This is semantically identical to AVERROR_BUG it has been introduced in Libav after our AVERROR_BUG and with a modified value.
    Bug2,
    /// Decoder not found.
    DecoderNotFound,
    /// Demuxer not found.
    DemuxerNotFound,
    /// Encoder not found.
    EncoderNotFound,
    /// End of file.
    Eof,
    /// Immediate exit was requested; the called function should not be restarted.
    Exit,
    /// Requested feature is flagged experimental. Set strict_std_compliance if you really want to use it.
    Experimental,
    /// Generic error in an external library.
    External,
    /// Filter not found.
    FilterNotFound,
    /// Server returned 400 Bad Request
    HttpBadRequest,
    /// Server returned 403 Forbidden (access denied)
    HttpForbidden,
    /// Server returned 404 Not Found
    HttpNotFound,
    /// Server returned 4XX Client Error, but not one of 40[0,1,3,4]
    HttpOther4xx,
    /// Server returned 5XX Server Error reply
    HttpServerError,
    /// Server returned 401 Unauthorized (authorization failed)
    HttpUnauthorized,
    /// Input changed between calls. Reconfiguration is required. (can be OR-ed with AVERROR_OUTPUT_CHANGED)
    InputChanged,
    /// Invalid data found when processing input.
    InvalidData,
    /// Muxer not found.
    MuxerNotFound,
    /// Option not found.
    OptionNotFound,
    /// Output changed between calls. Reconfiguration is required. (can be OR-ed with AVERROR_INPUT_CHANGED)
    OutputChanged,
    /// Not yet implemented in FFmpeg, patches welcome.
    Patchwelcome,
    /// Protocol not found.
    ProtocolNotFound,
    /// Stream not found.
    StreamNotFound,
    /// Unknown error, typically from an external library.
    Unknown,
}

impl FfmpegSysStatus {
    pub fn is_ok(&self) -> bool {
        matches!(self, FfmpegSysStatus::NoError)
    }

    pub fn is_error(&self) -> bool {
        !self.is_ok()
    }

    pub fn from_ffmpeg_sys_error(value: i32) -> Self {
        match value {
            0..=i32::MAX => FfmpegSysStatus::NoError,
            ffmpeg_sys_next::AVERROR_BSF_NOT_FOUND => FfmpegSysStatus::BsfNotFound,
            ffmpeg_sys_next::AVERROR_BUFFER_TOO_SMALL => FfmpegSysStatus::BufferTooSmall,
            ffmpeg_sys_next::AVERROR_BUG => FfmpegSysStatus::Bug,
            ffmpeg_sys_next::AVERROR_BUG2 => FfmpegSysStatus::Bug2,
            ffmpeg_sys_next::AVERROR_DECODER_NOT_FOUND => FfmpegSysStatus::DecoderNotFound,
            ffmpeg_sys_next::AVERROR_DEMUXER_NOT_FOUND => FfmpegSysStatus::DemuxerNotFound,
            ffmpeg_sys_next::AVERROR_ENCODER_NOT_FOUND => FfmpegSysStatus::EncoderNotFound,
            ffmpeg_sys_next::AVERROR_EOF => FfmpegSysStatus::Eof,
            ffmpeg_sys_next::AVERROR_EXIT => FfmpegSysStatus::Exit,
            ffmpeg_sys_next::AVERROR_EXPERIMENTAL => FfmpegSysStatus::Experimental,
            ffmpeg_sys_next::AVERROR_EXTERNAL => FfmpegSysStatus::External,
            ffmpeg_sys_next::AVERROR_FILTER_NOT_FOUND => FfmpegSysStatus::FilterNotFound,
            ffmpeg_sys_next::AVERROR_HTTP_BAD_REQUEST => FfmpegSysStatus::HttpBadRequest,
            ffmpeg_sys_next::AVERROR_HTTP_FORBIDDEN => FfmpegSysStatus::HttpForbidden,
            ffmpeg_sys_next::AVERROR_HTTP_NOT_FOUND => FfmpegSysStatus::HttpNotFound,
            ffmpeg_sys_next::AVERROR_HTTP_OTHER_4XX => FfmpegSysStatus::HttpOther4xx,
            ffmpeg_sys_next::AVERROR_HTTP_SERVER_ERROR => FfmpegSysStatus::HttpServerError,
            ffmpeg_sys_next::AVERROR_HTTP_UNAUTHORIZED => FfmpegSysStatus::HttpUnauthorized,
            ffmpeg_sys_next::AVERROR_INPUT_CHANGED => FfmpegSysStatus::InputChanged,
            ffmpeg_sys_next::AVERROR_INVALIDDATA => FfmpegSysStatus::InvalidData,
            ffmpeg_sys_next::AVERROR_MUXER_NOT_FOUND => FfmpegSysStatus::MuxerNotFound,
            ffmpeg_sys_next::AVERROR_OPTION_NOT_FOUND => FfmpegSysStatus::OptionNotFound,
            ffmpeg_sys_next::AVERROR_OUTPUT_CHANGED => FfmpegSysStatus::OutputChanged,
            ffmpeg_sys_next::AVERROR_PATCHWELCOME => FfmpegSysStatus::Patchwelcome,
            ffmpeg_sys_next::AVERROR_PROTOCOL_NOT_FOUND => FfmpegSysStatus::ProtocolNotFound,
            ffmpeg_sys_next::AVERROR_STREAM_NOT_FOUND => FfmpegSysStatus::StreamNotFound,
            _ => FfmpegSysStatus::Unknown,
        }
    }

    pub fn as_ffmpeg_sys_error(&self) -> i32 {
        match self {
            FfmpegSysStatus::NoError => 0,
            FfmpegSysStatus::BsfNotFound => ffmpeg_sys_next::AVERROR_BSF_NOT_FOUND,
            FfmpegSysStatus::BufferTooSmall => ffmpeg_sys_next::AVERROR_BUFFER_TOO_SMALL,
            FfmpegSysStatus::Bug => ffmpeg_sys_next::AVERROR_BUG,
            FfmpegSysStatus::Bug2 => ffmpeg_sys_next::AVERROR_BUG2,
            FfmpegSysStatus::DecoderNotFound => ffmpeg_sys_next::AVERROR_DECODER_NOT_FOUND,
            FfmpegSysStatus::DemuxerNotFound => ffmpeg_sys_next::AVERROR_DEMUXER_NOT_FOUND,
            FfmpegSysStatus::EncoderNotFound => ffmpeg_sys_next::AVERROR_ENCODER_NOT_FOUND,
            FfmpegSysStatus::Eof => ffmpeg_sys_next::AVERROR_EOF,
            FfmpegSysStatus::Exit => ffmpeg_sys_next::AVERROR_EXIT,
            FfmpegSysStatus::Experimental => ffmpeg_sys_next::AVERROR_EXPERIMENTAL,
            FfmpegSysStatus::External => ffmpeg_sys_next::AVERROR_EXTERNAL,
            FfmpegSysStatus::FilterNotFound => ffmpeg_sys_next::AVERROR_FILTER_NOT_FOUND,
            FfmpegSysStatus::HttpBadRequest => ffmpeg_sys_next::AVERROR_HTTP_BAD_REQUEST,
            FfmpegSysStatus::HttpForbidden => ffmpeg_sys_next::AVERROR_HTTP_FORBIDDEN,
            FfmpegSysStatus::HttpNotFound => ffmpeg_sys_next::AVERROR_HTTP_NOT_FOUND,
            FfmpegSysStatus::HttpOther4xx => ffmpeg_sys_next::AVERROR_HTTP_OTHER_4XX,
            FfmpegSysStatus::HttpServerError => ffmpeg_sys_next::AVERROR_HTTP_SERVER_ERROR,
            FfmpegSysStatus::HttpUnauthorized => ffmpeg_sys_next::AVERROR_HTTP_UNAUTHORIZED,
            FfmpegSysStatus::InputChanged => ffmpeg_sys_next::AVERROR_INPUT_CHANGED,
            FfmpegSysStatus::InvalidData => ffmpeg_sys_next::AVERROR_INVALIDDATA,
            FfmpegSysStatus::MuxerNotFound => ffmpeg_sys_next::AVERROR_MUXER_NOT_FOUND,
            FfmpegSysStatus::OptionNotFound => ffmpeg_sys_next::AVERROR_OPTION_NOT_FOUND,
            FfmpegSysStatus::OutputChanged => ffmpeg_sys_next::AVERROR_OUTPUT_CHANGED,
            FfmpegSysStatus::Patchwelcome => ffmpeg_sys_next::AVERROR_PATCHWELCOME,
            FfmpegSysStatus::ProtocolNotFound => ffmpeg_sys_next::AVERROR_PROTOCOL_NOT_FOUND,
            FfmpegSysStatus::StreamNotFound => ffmpeg_sys_next::AVERROR_STREAM_NOT_FOUND,
            FfmpegSysStatus::Unknown => ffmpeg_sys_next::AVERROR_UNKNOWN,
        }
    }
}

impl From<i32> for FfmpegSysStatus {
    fn from(value: i32) -> Self {
        Self::from_ffmpeg_sys_error(value)
    }
}

impl From<FfmpegSysStatus> for i32 {
    fn from(value: FfmpegSysStatus) -> Self {
        value.as_ffmpeg_sys_error()
    }
}

#[cfg(test)]
mod tests {
    use super::FfmpegSysStatus;

    fn get_all_errors() -> Vec<FfmpegSysStatus> {
        vec![
            FfmpegSysStatus::BsfNotFound,
            FfmpegSysStatus::BufferTooSmall,
            FfmpegSysStatus::Bug,
            FfmpegSysStatus::Bug2,
            FfmpegSysStatus::DecoderNotFound,
            FfmpegSysStatus::DemuxerNotFound,
            FfmpegSysStatus::EncoderNotFound,
            FfmpegSysStatus::Eof,
            FfmpegSysStatus::Exit,
            FfmpegSysStatus::Experimental,
            FfmpegSysStatus::External,
            FfmpegSysStatus::FilterNotFound,
            FfmpegSysStatus::HttpBadRequest,
            FfmpegSysStatus::HttpForbidden,
            FfmpegSysStatus::HttpNotFound,
            FfmpegSysStatus::HttpOther4xx,
            FfmpegSysStatus::HttpServerError,
            FfmpegSysStatus::HttpUnauthorized,
            FfmpegSysStatus::InputChanged,
            FfmpegSysStatus::InvalidData,
            FfmpegSysStatus::MuxerNotFound,
            FfmpegSysStatus::OptionNotFound,
            FfmpegSysStatus::OutputChanged,
            FfmpegSysStatus::Patchwelcome,
            FfmpegSysStatus::ProtocolNotFound,
            FfmpegSysStatus::StreamNotFound,
            FfmpegSysStatus::Unknown,
        ]
    }

    #[test]
    fn is_ok() {
        assert!(FfmpegSysStatus::NoError.is_ok());

        for status in get_all_errors() {
            assert!(!status.is_ok());
        }
    }

    #[test]
    fn is_error() {
        assert!(!FfmpegSysStatus::NoError.is_error());

        for status in get_all_errors() {
            assert!(status.is_error());
        }
    }
}
