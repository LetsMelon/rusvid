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
        .add_effect(PixelateEffect::new(15, 15))
        .build();
    let frames = composition.frames();

    let layer = composition.create_layer().unwrap();

    let circle_size = 250.0;
    let circle_pos = Point::ZERO;
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "circle".to_string(),
            fill: Some(Fill {
                paint: Paint::Color(color_from_hex("#1212FF".to_string()).unwrap()),
                ..Fill::default()
            }),
            data: Rc::new(circle(circle_pos, circle_size)),
            ..Path::default()
        }))
        .unwrap();

    layer.add_animation(PositionAnimation::new(
        "circle",
        Cubic::new_with_ease_type(
            0,
            frames,
            circle_pos,
            resolution.as_point() - (circle_size * 0.5),
            EaseType::InOut,
        )
        .unwrap(),
    ));

    let mut renderer = FfmpegRenderer::new("comp_effect.mp4", "./out", FrameImageFormat::Png);
    renderer.render(composition).unwrap()
}
