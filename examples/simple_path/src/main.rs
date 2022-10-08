use std::path::PathBuf;
use std::rc::Rc;

use rusvid_lib::animation::prelude::*;
use rusvid_lib::figures::prelude::*;
use rusvid_lib::prelude::*;
use rusvid_lib::usvg::{Fill, NodeKind, Paint, Path};
use rusvid_lib::utils::color_from_hex;

fn main() {
    let resolution = Resolution::HD;

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(24)
        .duration(2)
        .build();

    let layer = composition.create_layer().unwrap();

    let rect_size = Point::new(250.0, 250.0);
    let pixel_position = Point::new(
        (resolution.x() / 2.0) - (rect_size.x / 2.0),
        (resolution.y() / 2.0) - (rect_size.y / 2.0),
    );
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "rect".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("#1212FF".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(rect(
                pixel_position.x,
                pixel_position.y,
                rect_size.x,
                rect_size.y,
            )),
            ..Path::default()
        }))
        .unwrap();

    let out_path = PathBuf::from("simple_path.mp4");
    let tmp_path = PathBuf::from("./out");

    let mut renderer = FfmpegRenderer::new(out_path, tmp_path, FrameImageFormat::Png);
    renderer.render(composition).unwrap()
}
