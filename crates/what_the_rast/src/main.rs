#![feature(iter_array_chunks)]

use anyhow::Result;
use rusvid_core::{plane::Plane, point::Point};
use what_the_rast::*;

fn render_and_save<P: AsRef<std::path::Path>>(object: &Object, path: P) {
    let plane = object.render(1000, 1000).unwrap();
    let rgba_image = plane.as_rgba_image().unwrap();
    rgba_image.save(path).unwrap();
}

fn main_path() {
    let mut path = Vec::new();
    path.push(PathLike::Move(Point::new(100.0, 100.0)));
    path.push(PathLike::Line(Point::new(200.0, 100.0)));
    path.push(PathLike::Line(Point::new(200.0, 200.0)));
    path.push(PathLike::Line(Point::new(100.0, 200.0)));
    path.push(PathLike::Close);

    let mut object = Object::new(
        "custom_obj".to_string(),
        TypesLike::Svg(Svg::new(path, ColorLike::Color([255, 100, 100, 255]))),
    );

    println!("{:?}", object);

    render_and_save(&object, "test_svg.jpg");

    object
        .transform(Transform::Move(Point::new(100.0, 100.0)))
        .unwrap();
    render_and_save(&object, "test_svg_after_move.jpg");

    object
        .transform(Transform::Color(ColorLike::Color([0, 255, 200, 255])))
        .unwrap();
    render_and_save(&object, "test_svg_after_color.jpg");

    object
        .transform(Transform::Position(Point::new(0.0, 0.0)))
        .unwrap();
    render_and_save(&object, "test_svg_after_position.jpg");

    object
        .transforms(vec![
            Transform::Move(Point::new(400.0, 400.0)),
            Transform::Color(ColorLike::Color([100, 100, 255, 255])),
        ])
        .unwrap();
    render_and_save(&object, "test_svg_after_transformations.jpg");

    object.transform(Transform::Visibility(false)).unwrap();
    render_and_save(&object, "test_svg_after_visibility.jpg");
}

fn main_image() -> Result<()> {
    let png = image::io::Reader::open("crates/what_the_rast/data/cat.jpg")?.decode()?;

    let width = png.width();
    let height = png.height();

    let data = png
        .as_bytes()
        .clone()
        .iter()
        .array_chunks()
        .map(|[r, g, b]| [*r, *g, *b, 255])
        .collect::<Vec<_>>();

    let cat_image = Plane::from_data(width, height, data)?;

    let image_holder = ImageHolder::new_fit(Point::new(500.0, 300.0), cat_image);

    let mut object = Object::new("plane_image", TypesLike::Image(image_holder));

    render_and_save(&object, "test_image_01.jpg");

    object
        .transform(Transform::Color(ColorLike::Color([0; 4])))
        .unwrap();
    render_and_save(&object, "test_image_02_after_color.jpg");

    object
        .transform(Transform::Move(Point::new(100.0, 50.0)))
        .unwrap();
    render_and_save(&object, "test_image_03_after_move.jpg");

    object
        .transform(Transform::Position(Point::new(300.0, 300.0)))
        .unwrap();
    render_and_save(&object, "test_image_04_after_position.jpg");

    object.transform(Transform::Visibility(false)).unwrap();
    render_and_save(&object, "test_image_05_after_visibility.jpg");

    Ok(())
}

fn main() -> Result<()> {
    main_image();
    // main_path();

    Ok(())
}
