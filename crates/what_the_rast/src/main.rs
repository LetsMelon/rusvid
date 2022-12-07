use rusvid_core::point::Point;
use what_the_rast::*;

fn main() {
    let mut path = Vec::new();
    path.push(PathLike::Move(Point::new(100.0, 100.0)));
    path.push(PathLike::Line(Point::new(200.0, 100.0)));
    path.push(PathLike::Line(Point::new(200.0, 200.0)));
    path.push(PathLike::Line(Point::new(100.0, 200.0)));
    path.push(PathLike::Close);

    let mut object = Object::new("custom_obj".to_string(), TypesLike::Svg(Svg { path }));

    println!("{:?}", object);

    let plane = object.render(1000, 1000).unwrap();
    let rgba_image = plane.as_rgba_image().unwrap();
    rgba_image.save("test_img.jpg").unwrap();

    object
        .transform(Transform::Move(Point::new(100.0, 100.0)))
        .unwrap();
    let plane = object.render(1000, 1000).unwrap();
    let rgba_image = plane.as_rgba_image().unwrap();
    rgba_image.save("test_img_after_move.jpg").unwrap();

    object.transform(Transform::Visibility(false)).unwrap();
    let plane = object.render(1000, 1000).unwrap();
    let rgba_image = plane.as_rgba_image().unwrap();
    rgba_image.save("test_img_after_visibility.jpg").unwrap();
}
