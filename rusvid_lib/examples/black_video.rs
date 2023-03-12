use std::path::PathBuf;

use rusvid_lib::prelude::*;

fn main() {
    let resolution = Resolution::HD;

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(24)
        .duration(2)
        .build();

    let _ = composition.create_layer().unwrap();

    let out_path = PathBuf::from("black_video.mp4");

    let mut renderer = EmbeddedRenderer::new(out_path);
    renderer.render(composition).unwrap();
}
