use std::path::PathBuf;
use std::rc::Rc;

use rusvid_lib::figures::prelude::*;
use rusvid_lib::prelude::*;
use rusvid_lib::resvg::usvg::{Fill, NodeKind, Paint, Path};
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
    let pixel_position = (resolution.as_point() / 2.0) - (rect_size / 2.0);
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "rect".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("#1212FF".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(rect(pixel_position, rect_size)),
            ..Path::default()
        }))
        .unwrap();

    let out_path = PathBuf::from("simple_path.mp4");

    let mut renderer = EmbeddedRenderer::new(out_path);
    renderer.render(composition).unwrap()
}
