use std::path::PathBuf;
use std::rc::Rc;

use rusvid_lib::animation::prelude::*;
use rusvid_lib::figures::prelude::*;
use rusvid_lib::prelude::*;
use rusvid_lib::usvg::{
    BaseGradient, LinearGradient, NodeKind, Opacity, Paint, Path, SpreadMethod, Stop, StopOffset,
    Stroke, StrokeWidth, Transform, Units,
};
use rusvid_lib::utils::color_from_hex;

fn main() {
    let resolution = Resolution::FourK;

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(24)
        .duration(5)
        .add_effect(ColorPaletteEffect::new(vec![[0, 0, 0, 255], [255; 4]]))
        .build();

    let layer = composition.create_layer().unwrap(); // Layer::new(composition.resolution());
    layer
        .add_to_defs(NodeKind::LinearGradient(LinearGradient {
            id: "lg2".into(),
            x1: 0.0,
            y1: 0.0,
            x2: 1.0,
            y2: 0.0,
            base: BaseGradient {
                units: Units::ObjectBoundingBox,
                transform: Transform::default(),
                spread_method: SpreadMethod::Pad,
                stops: vec![
                    Stop {
                        offset: StopOffset::new(0.0),
                        color: color_from_hex("9796f0".to_string()).unwrap(),
                        opacity: Opacity::new(1.0),
                    },
                    Stop {
                        offset: StopOffset::new(1.0),
                        color: color_from_hex("fbc7d4".to_string()).unwrap(),
                        opacity: Opacity::new(1.0),
                    },
                ],
            },
        }))
        .unwrap();

    let circle_position = Point::new(700.0, 850.0);
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "circle".to_string(),
            stroke: Some(Stroke {
                paint: Paint::Link("lg2".into()),
                width: StrokeWidth::new(100.0),
                ..Stroke::default()
            }),
            rendering_mode: Default::default(),
            data: Rc::new(circle(circle_position.x, circle_position.y, 600.0)),
            ..Path::default()
        }))
        .unwrap();
    layer.add_animation(PositionAnimation::new(
        "circle".to_string(),
        Elastic::new_with_ease_type(
            0,
            90,
            circle_position,
            Point::new(resolution.x() / 2.0, resolution.y() / 2.0),
            EaseType::Out,
        )
        .unwrap(),
    ));

    /*
    let layer = composition.create_layer().unwrap();
    layer
        .add_to_defs(NodeKind::LinearGradient(LinearGradient {
            id: "lg1".into(),
            x1: 0.0,
            y1: 0.0,
            x2: 1.0,
            y2: 0.0,
            base: BaseGradient {
                units: Units::ObjectBoundingBox,
                transform: Transform::default(),
                spread_method: SpreadMethod::Pad,
                stops: vec![
                    Stop {
                        offset: StopOffset::new(0.0),
                        color: Color::new_rgb(0, 255, 0),
                        opacity: Opacity::new(1.0),
                    },
                    Stop {
                        offset: StopOffset::new(1.0),
                        color: Color::new_rgb(0, 0, 255),
                        opacity: Opacity::new(1.0),
                    },
                ],
            },
        }))
        .unwrap();

    let mut path = equilateral_triangle(400.0, 400.0, 350.0);
    path.transform(Transform::new_rotate(2.5));
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "triangle".to_string(),
            fill: layer.fill_with_link("lg1"),
            data: Rc::new(path),
            ..Path::default()
        }))
        .unwrap();

    let pixel_position = Point::new(20.0, 20.0);
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "rect".to_string(),
            fill: match layer.fill_with_link("lg1") {
                None => None,
                Some(mut f) => {
                    f.opacity = Opacity::new(0.75);
                    Some(f)
                }
            },
            data: Rc::new(rect(
                pixel_position.x,
                pixel_position.y,
                resolution.x() / 2.0,
                resolution.y() / 3.0,
            )),
            ..Path::default()
        }))
        .unwrap();
    layer.add_animation(PositionAnimation::new(
        "rect".to_string(),
        Linear::new(0, 200, pixel_position, (1250.0, 500.0).into()).unwrap(),
    ));
    layer.add_animation(PositionAnimation::new(
        "rect".to_string(),
        Linear::new(220, 290, (1250.0, 500.0).into(), (0.0, 0.0).into()).unwrap(),
    ));
     */

    let out_path = PathBuf::from("out.mp4");
    let tmp_path = PathBuf::from("./out");

    let mut renderer = FfmpegRenderer::new(out_path, tmp_path, FrameImageFormat::Png);
    renderer.render(composition).unwrap()
}
