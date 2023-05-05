/*
To rebuild all snapshots run `cargo test` with the env variable `TEST_REBUILD`.
To save temporary files from the tests for debugging run `cargo test` with the env variable `TEST_SAVE`.
*/

use std::fmt::Debug;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use image::io::Reader;
use image::{ImageFormat, RgbImage};
use image_compare::rgb_hybrid_compare;
use rusvid_core::holder::likes::color_like::ColorLike;
use rusvid_core::holder::likes::path_like::PathLike;
use rusvid_core::holder::likes::types_like::TypesLike;
use rusvid_core::holder::object::Object;
use rusvid_core::holder::polygon::Polygon;
use rusvid_core::holder::stroke::Stroke;
use rusvid_core::holder::svg_holder::SvgHolder;
use rusvid_core::holder::svg_item::SvgItem;
use rusvid_core::holder::transform::{RotationPoint, Transform, TransformLogic};
use rusvid_core::pixel::Pixel;
use rusvid_core::plane::Plane;
use rusvid_core::point::Point;

const DELTA: f64 = 0.00005;

const SIMPLE_PATH: &'static [u8] = include_bytes!("./data/simple_path.bmp");
const SIMPLE_TRANSFORM: &'static [u8] = include_bytes!("./data/simple_transform.bmp");
const SIMPLE_TRANSFORM_STROKE: &'static [u8] = include_bytes!("./data/simple_transform_stroke.bmp");
const SIMPLE_TRANSFORM_COLOR: &'static [u8] = include_bytes!("./data/simple_transform_color.bmp");
const SIMPLE_TRANSFORM_POSITION: &'static [u8] =
    include_bytes!("./data/simple_transform_position.bmp");
const SIMPLE_TRANSFORM_MOVE: &'static [u8] = include_bytes!("./data/simple_transform_move.bmp");
const SIMPLE_TRANSFORM_COLOR_NONE: &'static [u8] =
    include_bytes!("./data/simple_transform_color_none.bmp");
const SIMPLE_TRANSFORM_VISIBILITY: &'static [u8] =
    include_bytes!("./data/simple_transform_visibility.bmp");

fn rebuild_snapshots<P: AsRef<Path>>(image: &RgbImage, name: P) -> Result<()> {
    if option_env!("TEST_REBUILD").is_some() {
        let mut path_buf = PathBuf::new();
        path_buf.push(std::env::current_dir()?);
        path_buf.push("./tests/data");
        path_buf.push(name);
        path_buf.set_extension("bmp");

        image.save_with_format(path_buf, ImageFormat::Bmp)?
    }

    Ok(())
}

fn save_if_env(image: &RgbImage, name: &str) -> Result<()> {
    if option_env!("TEST_SAVE").is_some() {
        image.save_with_format(format!("{}.png", name), ImageFormat::Png)?;
    }

    Ok(())
}

fn test_image(image: RgbImage, snapshot: &[u8], name: &str) -> Result<()> {
    let mut reader = Reader::new(Cursor::new(snapshot));
    reader.set_format(ImageFormat::Bmp);
    let test_image = reader.decode()?.to_rgb8();

    let result = rgb_hybrid_compare(&image, &test_image)?;

    save_if_env(&image, name)?;

    assert!(
        result.score > (1.0 - DELTA),
        "score: {} (at {})",
        result.score,
        name
    );

    Ok(())
}

fn rebuild_and_test<P: AsRef<Path> + Clone + Debug>(
    plane: Plane,
    name: P,
    snapshot: &[u8],
) -> Result<()> {
    println!("path: {:?}", name);

    let image = plane.as_rgb_image()?;

    let binding = name.clone();
    let test_name = binding
        .as_ref()
        .to_str()
        .context("Can't transform `P` to &str")?;

    rebuild_snapshots(&image, name)?;
    test_image(image, snapshot, test_name)
}

#[test]
fn simple_path() {
    let mut svg = SvgHolder::new();
    let triangle = SvgItem::new(
        Polygon::new(&[
            PathLike::Move(Point::new(100.0, 100.0)),
            PathLike::Line(Point::new(150.0, 100.0)),
            PathLike::Line(Point::new(120.0, 150.0)),
            PathLike::Close,
        ]),
        Some(ColorLike::Color([0, 255, 100, 255].into())),
    );
    svg.add_item(triangle);

    let heart = SvgItem::new(
        Polygon::new(&[
            PathLike::Move(Point::new(100.0, 100.0)),
            PathLike::Line(Point::new(150.0, 50.0)),
            PathLike::CurveTo(
                Point::new(100.0, 25.0),
                Point::new(169.0, 11.0),
                Point::new(119.0, -13.0),
            ),
            PathLike::CurveTo(
                Point::new(50.0, 50.0),
                Point::new(80.0, -13.0),
                Point::new(30.0, 11.0),
            ),
            PathLike::Close,
        ]),
        Some(ColorLike::Color([255, 0, 0, 255].into())),
    );
    svg.add_item(heart);

    let object = Object::new(TypesLike::Svg(svg));

    let plane = object.render(300, 300).unwrap();
    rebuild_and_test(plane, "simple_path", SIMPLE_PATH).unwrap();
}

#[test]
fn simple_transform() {
    let mut svg = SvgHolder::new();
    let triangle_id = svg.add_item(SvgItem::new(
        Polygon::new(&[
            PathLike::Move(Point::new(100.0, 100.0)),
            PathLike::Line(Point::new(150.0, 100.0)),
            PathLike::Line(Point::new(120.0, 150.0)),
            PathLike::Close,
        ]),
        Some(ColorLike::Color([0, 255, 120, 255].into())),
    ));

    let heart_id = svg.add_item(SvgItem::new(
        Polygon::new(&[
            PathLike::Move(Point::new(100.0, 100.0)),
            PathLike::Line(Point::new(150.0, 50.0)),
            PathLike::CurveTo(
                Point::new(100.0, 25.0),
                Point::new(169.0, 11.0),
                Point::new(119.0, -13.0),
            ),
            PathLike::CurveTo(
                Point::new(50.0, 50.0),
                Point::new(80.0, -13.0),
                Point::new(30.0, 11.0),
            ),
            PathLike::Close,
        ]),
        Some(ColorLike::Color([200, 100, 20, 255].into())),
    ));

    let mut object = Object::new(TypesLike::Svg(svg));
    let plane = object.render(300, 300).unwrap();
    rebuild_and_test(plane, "simple_transform", SIMPLE_TRANSFORM).unwrap();

    object
        .transform_by_id(
            &triangle_id,
            &Transform::Stroke(Some(Stroke {
                paint: ColorLike::Color([100, 50, 120, 255].into()),
                width: 1.75,
                ..Stroke::default()
            })),
        )
        .unwrap();
    let plane = object.render(300, 300).unwrap();
    rebuild_and_test(plane, "simple_transform_stroke", SIMPLE_TRANSFORM_STROKE).unwrap();

    object
        .transform_by_id(
            &heart_id,
            &Transform::Color(Some(ColorLike::Color(Pixel::new(230, 57, 70, 255)))),
        )
        .unwrap();
    let plane = object.render(300, 300).unwrap();
    rebuild_and_test(plane, "simple_transform_color", SIMPLE_TRANSFORM_COLOR).unwrap();

    object
        .transform_by_id(&triangle_id, &Transform::Position(Point::new(200.0, 200.0)))
        .unwrap();
    let plane = object.render(300, 300).unwrap();
    rebuild_and_test(
        plane,
        "simple_transform_position",
        SIMPLE_TRANSFORM_POSITION,
    )
    .unwrap();

    object
        .transform_by_id(&heart_id, &Transform::Move(Point::new(50.0, 0.0)))
        .unwrap();
    let plane = object.render(300, 300).unwrap();
    rebuild_and_test(plane, "simple_transform_move", SIMPLE_TRANSFORM_MOVE).unwrap();

    object
        .transform_by_id(&triangle_id, &Transform::Color(None))
        .unwrap();
    let plane = object.render(300, 300).unwrap();
    rebuild_and_test(
        plane,
        "simple_transform_color_none",
        SIMPLE_TRANSFORM_COLOR_NONE,
    )
    .unwrap();

    object
        .transform_by_id(&triangle_id, &Transform::Visibility(false))
        .unwrap();
    let plane = object.render(300, 300).unwrap();
    rebuild_and_test(
        plane,
        "simple_transform_visibility",
        SIMPLE_TRANSFORM_VISIBILITY,
    )
    .unwrap();

    object
        .transform_by_id(
            &heart_id,
            &Transform::Rotate((90.0_f64.to_radians(), RotationPoint::Center)),
        )
        .unwrap();
    let plane = object.render(300, 300).unwrap();
    rebuild_snapshots(
        &plane.as_rgb_image().unwrap(),
        "simple_transform_rotation_center",
    )
    .unwrap();

    object
        .transform_by_id(
            &heart_id,
            &Transform::Rotate((
                10.0_f64.to_radians(),
                RotationPoint::Custom(Point::new(150.0, 150.0)),
            )),
        )
        .unwrap();
    let plane = object.render(300, 300).unwrap();
    rebuild_snapshots(
        &plane.as_rgb_image().unwrap(),
        "simple_transform_rotation_custom",
    )
    .unwrap();
}
