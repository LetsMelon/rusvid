use rusvid_video_encoder::Encoder;

fn do_sth_with_it(_encoder: Encoder) {}

fn main() {
    let size = 1024;

    for _ in 0..1_000_000_000 {
        let encoder = Encoder::new("out.mp4", (size, size), 24).unwrap();
        do_sth_with_it(std::hint::black_box(encoder));
    }
}
