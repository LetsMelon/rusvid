use rusvid_lib::composition::Composition;
use rusvid_lib::figures::circle::circle;
use rusvid_lib::resolution::Resolution;
use rusvid_lib::utils::color_from_hex;
use std::path::Path;
use std::rc::Rc;
use usvg::NodeExt;

fn main() {
    let mut composition = Composition::new("my_first".to_string(), Resolution::FHD);

    composition
        .rtree_mut()
        .append_to_defs(usvg::NodeKind::LinearGradient(usvg::LinearGradient {
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

    composition
        .rtree_mut()
        .append_to_defs(usvg::NodeKind::LinearGradient(usvg::LinearGradient {
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

    let fill = Some(usvg::Fill {
        paint: usvg::Paint::Link("lg2".into()),
        ..usvg::Fill::default()
    });

    composition
        .rtree()
        .root()
        .append_kind(usvg::NodeKind::Path(usvg::Path {
            fill,
            data: Rc::new(circle(700.0, 850.0, 600.0)),
            ..usvg::Path::default()
        }));

    let fill = Some(usvg::Fill {
        paint: usvg::Paint::Link("lg1".into()),
        opacity: usvg::Opacity::new(0.75),
        ..usvg::Fill::default()
    });

    let position = Rc::new(usvg::PathData::from_rect(
        usvg::Rect::new(
            20.0,
            20.0,
            composition.resolution().width() as f64 / 2.0,
            composition.resolution().height() as f64 / 3.0,
        )
        .unwrap(),
    ));

    composition
        .rtree()
        .root()
        .append_kind(usvg::NodeKind::Path(usvg::Path {
            fill,
            data: Rc::clone(&position),
            ..usvg::Path::default()
        }));

    let out = composition.render(Path::new("out.mp4"), Path::new("./out"), position);

    println!("{:?}", out);
}
