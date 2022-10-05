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
        .framerate(30)
        .duration(2)
        .build();
    let frames = composition.frames();

    let layer = composition.create_layer().unwrap();

    let rect_size = Point::new(250.0, 250.0);
    let rect_pos = Point::ZERO;
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "rect".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("#1212FF".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(rect(rect_pos.x, rect_pos.y, rect_size.x, rect_size.y)),
            ..Path::default()
        }))
        .unwrap();

    layer.add_animation(PositionAnimation::new(
        "rect".to_string(),
        Cubic::new_with_ease_type(
            0,
            frames,
            rect_pos,
            Point::new(
                resolution.width() as f64 - rect_size.x,
                resolution.height() as f64 - rect_size.y,
            ),
            EaseType::InOut,
        )
        .unwrap(),
    ));

    let out_path = PathBuf::from("simple_animation.mp4");
    let tmp_path = PathBuf::from("./out");

    let mut renderer = FfmpegRenderer::new(out_path, tmp_path, FrameImageFormat::Png);
    renderer.render(composition).unwrap()
}
