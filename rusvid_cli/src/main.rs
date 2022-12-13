use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use chrono::Local;
use fern::{log_file, Dispatch};
use log::{debug, LevelFilter};
use rusvid_lib::animation::prelude::*;
use rusvid_lib::core::frame_image_format::FrameImageFormat;
use rusvid_lib::figures::prelude::*;
use rusvid_lib::prelude::*;
use rusvid_lib::resvg::usvg::{
    BaseGradient, Color, LinearGradient, NodeKind, NonZeroPositiveF64, NormalizedF64, Opacity,
    Path, SpreadMethod, Stop, StopOffset, Stroke, Transform, Units,
};
use rusvid_lib::utils::color_from_hex;

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
        .framerate(24)
        .duration(5)
        // .add_effect(PixelateEffect::new(15, 15))
        // .add_effect(ColorPaletteEffect::new(vec![
        //     [10, 56, 120, 255],
        //     [100, 100, 0, 255],
        //     [100, 10, 100, 255],
        //     [90, 12, 30, 255],
        // ]))
        // .add_effect(GrayscaleEffect::new())
        // .add_effect(BoxBlur::new(5).unwrap())
        // .add_effect(GaussianBlur::new(3.0))
        .build();

    let layer = composition.create_layer().unwrap(); // Layer::new(composition.resolution());
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
                    color: color_from_hex("9796f0".to_string()).unwrap(),
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
    layer.add_animation(PositionAnimation::new(
        "circle",
        Elastic::new_with_ease_type(
            0,
            90,
            circle_position,
            resolution.as_point() / 2.0,
            EaseType::Out,
        )
        .unwrap(),
    ));
    let pixel_size = 20;
    layer.add_effect(PixelateEffect::new(pixel_size, pixel_size));

    let layer = composition.create_layer().unwrap();
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

    let mut renderer = FfmpegRenderer::new("out.mp4", "./out", FrameImageFormat::Png);
    renderer.render(composition).unwrap()
}
