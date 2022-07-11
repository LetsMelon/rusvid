use rusvid_lib::composition::Composition;
use rusvid_lib::figures::circle::circle;
use rusvid_lib::figures::rect::rect;
use rusvid_lib::figures::triangle::equilateral_triangle;
use rusvid_lib::resolution::Resolution;
use rusvid_lib::types::NodeKind;
use rusvid_lib::utils::color_from_hex;
use std::path::Path;
use std::rc::Rc;
use usvg::{Fill, NodeExt, Paint, ShapeRendering, Stroke, StrokeWidth, Transform};

fn main() {
    let mut composition = Composition::builder()
        .resolution(Resolution::FHD)
        .framerate(30)
        .duration(5)
        .build();

    composition.add_to_defs(NodeKind::LinearGradient(usvg::LinearGradient {
        id: "lg1".into(),
        x1: 0.0,
        y1: 0.0,
        x2: 1.0,
        y2: 0.0,
        base: usvg::BaseGradient {
            units: usvg::Units::ObjectBoundingBox,
            transform: usvg::Transform::default(),
            spread_method: usvg::SpreadMethod::Pad,
            stops: vec![
                usvg::Stop {
                    offset: usvg::StopOffset::new(0.0),
                    color: usvg::Color::new_rgb(0, 255, 0),
                    opacity: usvg::Opacity::new(1.0),
                },
                usvg::Stop {
                    offset: usvg::StopOffset::new(1.0),
                    color: usvg::Color::new_rgb(0, 0, 255),
                    opacity: usvg::Opacity::new(1.0),
                },
            ],
        },
    }));
    composition.add_to_defs(NodeKind::LinearGradient(usvg::LinearGradient {
        id: "lg2".into(),
        x1: 0.0,
        y1: 0.0,
        x2: 1.0,
        y2: 0.0,
        base: usvg::BaseGradient {
            units: usvg::Units::ObjectBoundingBox,
            transform: usvg::Transform::default(),
            spread_method: usvg::SpreadMethod::Pad,
            stops: vec![
                usvg::Stop {
                    offset: usvg::StopOffset::new(0.0),
                    color: color_from_hex("9796f0".to_string()).unwrap(),
                    opacity: usvg::Opacity::new(1.0),
                },
                usvg::Stop {
                    offset: usvg::StopOffset::new(1.0),
                    color: color_from_hex("fbc7d4".to_string()).unwrap(),
                    opacity: usvg::Opacity::new(1.0),
                },
            ],
        },
    }));

    composition.add_to_root(NodeKind::Path(rusvid_lib::types::Path {
        stroke: Some(Stroke {
            paint: Paint::Link("lg2".into()),
            width: StrokeWidth::new(10.0),
            ..Stroke::default()
        }),
        rendering_mode: Default::default(),
        data: Rc::new(circle(700.0, 850.0, 600.0)),
        ..rusvid_lib::types::Path::default()
    }));

    let mut path = equilateral_triangle(400.0, 400.0, 350.0);
    path.transform(Transform::new_rotate(2.5));
    composition.add_to_root(NodeKind::Path(rusvid_lib::types::Path {
        fill: composition.fill_with_link("lg1"),
        data: Rc::new(path),
        ..rusvid_lib::types::Path::default()
    }));

    let position = Rc::new(rect(
        20.0,
        20.0,
        composition.resolution().width() as f64 / 2.0,
        composition.resolution().height() as f64 / 3.0,
    ));

    composition.add_to_root(NodeKind::Path(rusvid_lib::types::Path {
        fill: match composition.fill_with_link("lg1") {
            None => None,
            Some(mut f) => {
                f.opacity = usvg::Opacity::new(0.75);
                Some(f)
            }
        },
        data: Rc::clone(&position),
        ..rusvid_lib::types::Path::default()
    }));

    composition
        .render(Path::new("out.mp4"), Path::new("./out"), position)
        .unwrap();
}
