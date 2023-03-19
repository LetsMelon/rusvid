use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use chrono::Local;
use fern::{log_file, Dispatch};
use log::{debug, LevelFilter};
use rusvid_lib::animation::position_animation::NewPositionAnimation;
use rusvid_lib::figures::prelude::circle;
use rusvid_lib::layer::LayerType;
use rusvid_lib::prelude::holder::gradient::base::BaseGradient;
use rusvid_lib::prelude::holder::gradient::linear::LinearGradient;
use rusvid_lib::prelude::holder::likes::{ColorLike, PathLike, TypesLike};
use rusvid_lib::prelude::holder::svg_holder::SvgItem;
use rusvid_lib::prelude::*;

fn setup_logger() -> Result<String> {
    let time_stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let log_file_path = format!("output_{}.log", time_stamp);

    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S.%f]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Trace)
        .chain(std::io::stdout())
        .chain(log_file(log_file_path.clone())?)
        .apply()?;
    Ok(log_file_path)
}

fn main() {
    let log_file_path = setup_logger().unwrap();
    debug!("Save log file as {}", log_file_path);

    let resolution = Resolution::FourK;

    let mut composition = Composition::builder()
        .resolution(resolution)
        // .framerate(60)
        // .duration(5)
        .framerate(30)
        .duration(3)
        .build();

    let layer = composition.create_new_layer(LayerType::Svg).unwrap();

    let id = if let TypesLike::Svg(svg_data) = layer.object.data_mut() {
        let rect_raw = vec![
            PathLike::Move(Point::new(200.0, 200.0)),
            PathLike::Line(Point::new(200.0, 500.0)),
            PathLike::Line(Point::new(500.0, 500.0)),
            PathLike::Line(Point::new(500.0, 200.0)),
        ];
        let fill = Some(ColorLike::Color(Pixel::new(255, 0, 0, 255)));

        let rect = SvgItem::new(rect_raw, fill);

        svg_data.add_item(rect)
    } else {
        panic!("Can't add a svg to the layer")
    };

    layer.add_position_animation(NewPositionAnimation::new(
        id,
        (0, 60),
        (Point::ZERO, Point::new(500.0, 750.0)),
        Linear::new(),
    ));

    let circle_position = resolution.as_point() / 2.0;
    let id = if let TypesLike::Svg(svg_data) = layer.object.data_mut() {
        let fill = Some(ColorLike::LinearGradient(LinearGradient::new(
            BaseGradient::new_from_colors(vec![
                Pixel::from_hex_string("9769f0").unwrap(),
                Pixel::from_hex_string("fbc7d4").unwrap(),
            ]),
        )));

        let rect = SvgItem::new(circle(circle_position, 50.0), fill);

        svg_data.add_item(rect)
    } else {
        panic!("Can't add a svg to the layer")
    };

    /*
    layer.add_linear_gradient(LinearGradient {
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
                    offset: StopOffset::ZERO,
                    color: Color::new_rgb(0, 255, 0),
                    opacity: Opacity::ONE,
                },
                Stop {
                    offset: StopOffset::ONE,
                    color: Color::new_rgb(0, 0, 255),
                    opacity: Opacity::ONE,
                },
            ],
        },
    });
    layer.add_linear_gradient(LinearGradient {
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
                    offset: StopOffset::ZERO,
                    color: color_from_hex("9769f0".to_string()).unwrap(),
                    opacity: Opacity::ONE,
                },
                Stop {
                    offset: StopOffset::ONE,
                    color: color_from_hex("fbc7d4".to_string()).unwrap(),
                    opacity: Opacity::ONE,
                },
            ],
        },
    });

    let circle_position = Point::new(700.0, 850.0);
    layer
        .add_to_root(NodeKind::Path(Path {
            id: "circle".to_string(),
            stroke: Some(Stroke {
                paint: layer.get_linear_gradient("lg2").unwrap(),
                width: NonZeroPositiveF64::new(100.0).unwrap(),
                ..Stroke::default()
            }),
            rendering_mode: Default::default(),
            data: Rc::new(circle(circle_position, 600.0)),
            ..Path::default()
        }))
        .unwrap();

    let mut path = equilateral_triangle(Point::new(400.0, 400.0), 350.0);
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
                    f.opacity = NormalizedF64::new(0.75).unwrap();
                    Some(f)
                }
            },
            data: Rc::new(rect(
                pixel_position,
                resolution.as_point() / Point::new(2.0, 3.0),
            )),
            ..Path::default()
        }))
        .unwrap();

    layer.add_animation(PositionAnimation::new(
        "rect",
        Linear::new(0, 200, pixel_position, (1250.0, 500.0).into()).unwrap(),
    ));
    layer.add_animation(PositionAnimation::new(
        "rect",
        Linear::new(220, 290, (1250.0, 500.0).into(), (0.0, 0.0).into()).unwrap(),
    ));
    layer.add_animation(PositionAnimation::new(
        "circle",
        Sine::new(0, 90, circle_position, resolution.as_point() / 2.0).unwrap(),
    ));
    */

    let mut renderer = EmbeddedRenderer::new("out.mp4");
    renderer.render(composition).unwrap();
}
