use std::rc::Rc;

use rusvid_lib::animation::prelude::*;
use rusvid_lib::figures::prelude::*;
use rusvid_lib::prelude::*;
use rusvid_lib::usvg::{
    BaseGradient, Color, Fill, LinearGradient, NodeKind, Opacity, Paint, Path, SpreadMethod, Stop,
    StopOffset, Transform, Units,
};

fn main() {
    let resolution = Resolution::FHD;

    let mut composition = Composition::builder()
        .resolution(resolution)
        .framerate(30)
        .duration(5)
        .build();

    let frames = composition.frames();

    let background_layer = composition.create_layer().unwrap();

    background_layer
        .add_to_defs(NodeKind::LinearGradient(LinearGradient {
            id: "bg".into(),
            x1: 0.0,
            y1: 0.0,
            x2: 1.0,
            y2: 0.0,
            base: BaseGradient {
                units: Units::ObjectBoundingBox,
                transform: Transform::new_rotate(35.0),
                spread_method: SpreadMethod::Pad,
                stops: vec![
                    Stop {
                        offset: StopOffset::new(0.0),
                        color: Color::new_rgb(0, 215, 255),
                        opacity: Opacity::new(1.0),
                    },
                    Stop {
                        offset: StopOffset::new(0.5),
                        color: Color::new_rgb(9, 9, 121),
                        opacity: Opacity::new(1.0),
                    },
                    Stop {
                        offset: StopOffset::new(1.0),
                        color: Color::new_rgb(0, 215, 255),
                        opacity: Opacity::new(1.0),
                    },
                ],
            },
        }))
        .unwrap();

    let start_pos = resolution.as_point() * Point::NEG_ONE;
    background_layer
        .add_to_root(NodeKind::Path(Path {
            id: "bg_obj".to_string(),
            fill: background_layer.fill_with_link("bg"),
            data: Rc::new(rect(
                start_pos.x,
                start_pos.y,
                resolution.as_point().x * 2.0,
                resolution.as_point().y * 2.0,
            )),
            ..Path::default()
        }))
        .unwrap();

    background_layer.add_animation(PositionAnimation::new(
        "bg_obj".to_string(),
        Linear::new(0, frames, start_pos, Point::ZERO).unwrap(),
    ));

    let grid_layer = composition.create_layer().unwrap();

    let grid_size = Point::new(32.0, 18.0);
    let margin = Point::new(5.0, 5.0);
    let rect_size = (resolution.as_point() - (margin * (grid_size + Point::ONE))) / grid_size;

    for x in 0..(grid_size.x as usize) {
        for y in 0..(grid_size.y as usize) {
            let coordinates_as_point = Point::new(x as f64, y as f64);
            let extra_margin = margin * (coordinates_as_point + Point::ONE);
            let rect_pos = coordinates_as_point * rect_size + extra_margin;

            grid_layer
                .add_to_root(NodeKind::Path(Path {
                    id: "rect".to_string(),
                    fill: Some(Fill {
                        paint: Paint::Color(Color::new_rgb(0, 0, 0)),
                        ..Fill::default()
                    }),
                    data: Rc::new(rect(rect_pos.x, rect_pos.y, rect_size.x, rect_size.y)),
                    ..Path::default()
                }))
                .unwrap();
        }
    }

    let mut renderer = FfmpegRenderer::new("grid.mp4", "./out", FrameImageFormat::Png);
    renderer.render(composition).unwrap()
}
