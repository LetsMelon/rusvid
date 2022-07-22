use rusvid_lib::prelude::*;
use rusvid_lib::usvg::{
    BaseGradient, Color, LinearGradient, NodeKind, Opacity, Paint, Path, SpreadMethod, Stop,
    StopOffset, Stroke, StrokeWidth, Transform, Units,
};
use rusvid_lib::utils::color_from_hex;
use std::path::PathBuf;
use std::rc::Rc;

fn main() {
    let mut composition = Composition::builder()
        .resolution(Resolution::FourK)
        .framerate(60)
        .duration(5)
        .build();

    composition.add_to_defs(NodeKind::LinearGradient(LinearGradient {
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
    }));
    composition.add_to_defs(NodeKind::LinearGradient(LinearGradient {
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
    }));

    let circle_position = animation::Points::Point2d(700.0, 850.0);
    let circle_path = Rc::new(figures::circle(
        circle_position.x(),
        circle_position.y(),
        600.0,
    ));

    composition.add_to_root(NodeKind::Path(Path {
        stroke: Some(Stroke {
            paint: Paint::Link("lg2".into()),
            width: StrokeWidth::new(10.0),
            ..Stroke::default()
        }),
        rendering_mode: Default::default(),
        data: Rc::new(figures::circle(700.0, 850.0, 600.0)),
        ..Path::default()
    }));

    let mut path = figures::equilateral_triangle(400.0, 400.0, 350.0);
    path.transform(Transform::new_rotate(2.5));
    composition.add_to_root(NodeKind::Path(Path {
        fill: composition.fill_with_link("lg1"),
        data: Rc::new(path),
        ..Path::default()
    }));

    let pixel_position = animation::Points::Point2d(20.0, 20.0);

    let position = Rc::new(figures::rect(
        pixel_position.x(),
        pixel_position.y(),
        composition.resolution().width() as f64 / 2.0,
        composition.resolution().height() as f64 / 3.0,
    ));

    composition.add_to_root(NodeKind::Path(Path {
        fill: match composition.fill_with_link("lg1") {
            None => None,
            Some(mut f) => {
                f.opacity = Opacity::new(0.75);
                Some(f)
            }
        },
        data: Rc::clone(&position),
        ..Path::default()
    }));

    let out_path = PathBuf::from("out.mp4");
    let tmp_path = PathBuf::from("./out");

    // TODO add builder pattern for video- & image-render
    let animation_1 = animation::PositionAnimation::new(
        position.clone(),
        Box::new(
            animation::functions::Linear::new(0, 200, pixel_position, (1250.0, 500.0).into())
                .unwrap(),
        ),
    );
    let animation_2 = animation::PositionAnimation::new(
        position,
        Box::new(
            animation::functions::Linear::new(220, 290, (1250.0, 500.0).into(), (0.0, 0.0).into())
                .unwrap(),
        ),
    );
    let animation_3 = animation::PositionAnimation::new(
        circle_path,
        Box::new(
            animation::functions::S::new(
                0,
                90,
                circle_position,
                animation::Points::Point2d(
                    composition.resolution().width() as f64 / 2.0,
                    composition.resolution().height() as f64 / 2.0,
                ),
            )
            .unwrap(),
        ),
    );

    let mut renderer = FfmpegRenderer::new(out_path, tmp_path);
    renderer.set_image_render(Box::new(PngRender::new()));
    renderer.add_animation(Box::new(animation_1));
    renderer.add_animation(Box::new(animation_2));
    renderer.add_animation(Box::new(animation_3));
    renderer.render(composition).unwrap()
}
