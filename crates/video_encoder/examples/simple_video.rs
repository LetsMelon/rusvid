use rusvid_core::pixel::Pixel;
use rusvid_core::plane::Plane;
use rusvid_video_encoder::*;

fn main() {
    let width = 512;
    let height = 512;
    let fps = 24;

    let mut encoder = Encoder::new("out_simple_video.mp4", (width, height), fps).unwrap();

    let gradient_duration = 24; // duration in seconds
    let frames = (gradient_duration * fps) as f32;

    for i in 0..(frames as usize) {
        let plane = Plane::new_with_fill(
            width as u32,
            height as u32,
            Pixel::new(
                (i & 255) as u8,
                i.wrapping_div(3) as u8,
                i.wrapping_mul(2) as u8,
                255,
            ),
        )
        .unwrap();

        encoder.encode_plane(plane).unwrap();
    }

    encoder.finish_stream().unwrap();
}
