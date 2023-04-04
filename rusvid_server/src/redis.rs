#[inline(always)]
pub const fn video_status_prefix() -> &'static str {
    "video_id_"
}

pub fn key_for_video_status(video_id: &String) -> String {
    format!("{}{video_id}", video_status_prefix())
}
