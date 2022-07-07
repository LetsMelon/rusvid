mod composition;
mod object;
mod resolution;

use std::convert::Infallible;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::str::FromStr;
use usvg::NodeExt;
use usvg::Size;

use crate::composition::Composition;
use crate::resolution::Resolution;

fn main() {
    let mut composition = Composition::new("Test".to_string(), Resolution::Custom(2000, 2000));

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

    let fill = Some(usvg::Fill {
        paint: usvg::Paint::Link("lg1".into()),
        ..usvg::Fill::default()
    });

    composition
        .rtree()
        .root()
        .append_kind(usvg::NodeKind::Path(usvg::Path {
            fill,
            data: Rc::new(usvg::PathData::from_rect(
                usvg::Rect::new(
                    20.0,
                    20.0,
                    composition.resolution().width() as f64 / 2.0,
                    composition.resolution().height() as f64 / 3.0,
                )
                .unwrap(),
            )),
            ..usvg::Path::default()
        }));

    composition
        .save_single(Path::new("./out.png"))
        .expect("Error: while saving as png");
}
