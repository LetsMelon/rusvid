use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use chrono::Local;
use fern::{log_file, Dispatch};
use log::{debug, LevelFilter};
use rusvid_lib::animation::position_animation::PositionAnimation;
use rusvid_lib::animation::AnimationType;
use rusvid_lib::figures::prelude::circle;
use rusvid_lib::figures::rect::rect;
use rusvid_lib::figures::triangle::equilateral_triangle;
use rusvid_lib::layer::LayerType;
use rusvid_lib::prelude::holder::gradient::base::BaseGradient;
use rusvid_lib::prelude::holder::gradient::linear::LinearGradient;
use rusvid_lib::prelude::holder::likes::{ColorLike, PathLike, TypesLike};
use rusvid_lib::prelude::holder::svg_holder::SvgItem;
use rusvid_lib::prelude::holder::transform::{Transform, TransformLogic};
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
        let fill = Some(ColorLike::Color(Pixel::new(255, 0, 0, 255)));

        let rect = SvgItem::new(
            rect(Point::new_symmetric(200.0), Point::new_symmetric(300.0)),
            fill,
        );

        svg_data.add_item(rect)
    } else {
        panic!("Can't add a svg to the layer")
    };

    layer.add_position_animation(PositionAnimation::new(
        &id,
        (0, 200),
        (Point::new_symmetric(200.0), Point::new(1250.0, 500.0)),
        Linear::new(),
    ));

    layer.add_position_animation(PositionAnimation::new(
        &id,
        (220, 290),
        (Point::new(1250.0, 500.0), Point::ZERO),
        Linear::new(),
    ));

    let circle_position = Point::new(700.0, 850.0);
    let id = if let TypesLike::Svg(svg_data) = layer.object.data_mut() {
        let fill = Some(ColorLike::LinearGradient(LinearGradient::new(
            BaseGradient::new_from_colors(vec![
                Pixel::from_hex_string("9769f0").unwrap(),
                Pixel::from_hex_string("fbc7d4").unwrap(),
            ]),
        )));

        let rect = SvgItem::new(circle(circle_position, 250.0), fill);

        svg_data.add_item(rect)
    } else {
        panic!("Can't add a svg to the layer")
    };

    layer.add_animation(AnimationType::Position(PositionAnimation::new(
        &id,
        (0, 90),
        (circle_position, resolution.as_point() / 2.0),
        Sine::new(),
    )));

    if let TypesLike::Svg(svg_data) = layer.object.data_mut() {
        let fill = Some(ColorLike::LinearGradient(LinearGradient::new(
            BaseGradient::new_from_colors(vec![
                Pixel::new(0, 255, 0, 255),
                Pixel::new(0, 0, 255, 255),
            ]),
        )));

        let mut item = SvgItem::new(equilateral_triangle(Point::new(400.0, 400.0), 350.0), fill);

        item.transform(&Transform::Rotate(2.5)).unwrap();

        svg_data.add_item(item)
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
    */

    let mut renderer = EmbeddedRenderer::new("out.mp4");
    renderer.render(composition).unwrap();
}
