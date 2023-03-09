use std::env;
use std::path::PathBuf;

use rusvid_lib::core::frame_image_format::FrameImageFormat;
use rusvid_lib::prelude::*;

fn main() {
    let resolution = Resolution::HD;

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(24)
        .duration(2)
        .build();

    let path = env::current_dir()
        .unwrap()
        .as_path()
        .join("./rusvid_lib/examples/assets/rect_gradient.svg");
    let layer = Layer::from_file(composition.resolution(), &path).unwrap();
    composition.add_layer(layer);

    let out_path = PathBuf::from("read_svg_file.mp4");
    let tmp_path = PathBuf::from("./out");

    let mut renderer = FfmpegRenderer::new(out_path, tmp_path, FrameImageFormat::Png);
    renderer.render(composition).unwrap()
}
