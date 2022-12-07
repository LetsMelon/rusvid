use rusvid_core::point::Point;
use what_the_rast::*;

fn render_and_save<P: AsRef<std::path::Path>>(object: &Object, path: P) {
    let plane = object.render(1000, 1000).unwrap();
    let rgba_image = plane.as_rgba_image().unwrap();
    rgba_image.save(path).unwrap();
}

fn main() {
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

    render_and_save(&object, "test_img.jpg");

    object
        .transform(Transform::Move(Point::new(100.0, 100.0)))
        .unwrap();
    render_and_save(&object, "test_img_after_move.jpg");

    object
        .transform(Transform::Color(ColorLike::Color([0, 255, 200, 255])))
        .unwrap();
    render_and_save(&object, "test_img_after_color.jpg");

    object
        .transform(Transform::Position(Point::new(0.0, 0.0)))
        .unwrap();
    render_and_save(&object, "test_img_after_position.jpg");

    object
        .transforms(vec![
            Transform::Move(Point::new(400.0, 400.0)),
            Transform::Color(ColorLike::Color([100, 100, 255, 255])),
        ])
        .unwrap();
    render_and_save(&object, "test_img_after_transformations.jpg");

    object.transform(Transform::Visibility(false)).unwrap();
    render_and_save(&object, "test_img_after_visibility.jpg");
}
