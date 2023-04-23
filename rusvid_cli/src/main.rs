use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use chrono::Local;
use fern::{log_file, Dispatch};
use log::{debug, LevelFilter};
use rusvid_lib::animation::change_color_animation::ChangeColorAnimation;
use rusvid_lib::animation::position_animation::PositionAnimation;
use rusvid_lib::animation::{AnimationType, EaseType, FunctionType};
use rusvid_lib::figures::prelude::circle;
use rusvid_lib::figures::rect::rect;
use rusvid_lib::figures::triangle::equilateral_triangle;
use rusvid_lib::layer::LayerType;
use rusvid_lib::prelude::holder::gradient::base::BaseGradient;
use rusvid_lib::prelude::holder::gradient::linear::LinearGradient;
use rusvid_lib::prelude::holder::likes::ColorLike;
use rusvid_lib::prelude::holder::svg_item::SvgItem;
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

    let layer = composition.create_layer(LayerType::Svg).unwrap();

    layer
        .add_svg_item({
            let fill = Some(ColorLike::LinearGradient(LinearGradient::new(
                BaseGradient::new_from_colors(vec![
                    Pixel::new(0, 255, 0, 255),
                    Pixel::new(0, 0, 255, 255),
                ]),
            )));

            let mut item =
                SvgItem::new(equilateral_triangle(Point::new(400.0, 400.0), 350.0), fill);

            item.transform(&Transform::Rotate(2.5)).unwrap();

            item
        })
        .unwrap();

    let circle_position = Point::new(700.0, 850.0);
    let circle_id = layer
        .add_svg_item({
            let fill = Some(ColorLike::LinearGradient(LinearGradient::new(
                BaseGradient::new_from_colors(vec![
                    Pixel::from_hex_string("9769f0").unwrap(),
                    Pixel::from_hex_string("fbc7d4").unwrap(),
                ]),
            )));

            let item = SvgItem::new(circle(circle_position, 250.0), fill);

            item
        })
        .unwrap();

    let rect_id = layer
        .add_svg_item({
            let fill = Some(ColorLike::LinearGradient(LinearGradient::new(
                BaseGradient::new_from_colors(vec![
                    Pixel::new(0, 255, 0, 155),
                    Pixel::new(0, 0, 255, 155),
                ]),
            )));

            let item = SvgItem::new(
                rect(
                    Point::new_symmetric(20.0),
                    resolution.as_point() / Point::new(2.0, 3.0),
                ),
                fill,
            );

            item
        })
        .unwrap();

    layer.add_position_animation(PositionAnimation::new(
        &rect_id,
        0..200,
        (Point::new_symmetric(20.0), Point::new(1250.0, 500.0)),
        FunctionType::Linear,
        EaseType::default(),
    ));

    layer.add_position_animation(PositionAnimation::new(
        &rect_id,
        220..290,
        (Point::new(1250.0, 500.0), Point::ZERO),
        FunctionType::Linear,
        EaseType::default(),
    ));

    layer.add_animation(AnimationType::Position(PositionAnimation::new(
        &circle_id,
        0..90,
        (circle_position, resolution.as_point() / 2.0),
        FunctionType::Sine,
        EaseType::default(),
    )));

    layer.add_animation(AnimationType::ChangeColor(ChangeColorAnimation::new(
        &rect_id,
        (0, 100),
        (Pixel::new(255, 100, 0, 255), Pixel::new(255, 0, 255, 255)),
        FunctionType::Sine,
        EaseType::InOut,
    )));

    // composition.save_as_file("out.rusvid").unwrap();
    // let composition = Composition::load_from_file("out.rusvid").unwrap();

    let mut renderer = EmbeddedRenderer::new("out.mp4");
    // let mut renderer = FrameRenderer::new_with_file_type("./out", FrameImageFormat::Bmp);
    // let mut renderer = RemoteRenderer::new("server_out.mp4", "http://127.0.0.1:8080").unwrap();
    renderer.render(composition).unwrap();
}
