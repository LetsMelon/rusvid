use rusvid_core::holder::likes::color_like::ColorLike;
use rusvid_core::holder::likes::path_like::PathLike;
use rusvid_core::holder::likes::types_like::TypesLike;
use rusvid_core::holder::object::Object;
use rusvid_core::holder::stroke::Stroke;
use rusvid_core::holder::svg_holder::SvgHolder;
use rusvid_core::holder::svg_item::SvgItem;
use rusvid_core::holder::transform::{Transform, TransformLogic};
use rusvid_core::pixel::Pixel;
use rusvid_core::point::Point;

fn render_and_save(object: &Object, name: &str) {
    let plane = object.render(300, 300).unwrap();
    let path = format!("example_simple_transform_{}.png", name);
    plane.save_as_png(path).unwrap()
}

fn main() {
    let mut svg = SvgHolder::new();
    let triangle_id = svg.add_item(SvgItem::new(
        vec![
            PathLike::Move(Point::new(100.0, 100.0)),
            PathLike::Line(Point::new(150.0, 100.0)),
            PathLike::Line(Point::new(120.0, 150.0)),
            PathLike::Close,
        ],
        Some(ColorLike::Color(Pixel::new(0, 255, 100, 255))),
    ));

    let heart_id = svg.add_item(SvgItem::new(
        vec![
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
        ],
        Some(ColorLike::Color(Pixel::new(255, 0, 0, 255))),
    ));

    let mut object = Object::new(TypesLike::Svg(svg));
    render_and_save(&object, "output");

    object
        .transform_by_id(
            &triangle_id,
            &Transform::Stroke(Some(Stroke {
                paint: ColorLike::Color(Pixel::new(100, 50, 120, 255)),
                width: 1.75,
                ..Stroke::default()
            })),
        )
        .unwrap();
    render_and_save(&object, "stroke");

    object
        .transform_by_id(
            &heart_id,
            &Transform::Color(Some(ColorLike::Color(Pixel::new(230, 57, 70, 255)))),
        )
        .unwrap();
    render_and_save(&object, "color");

    object
        .transform_by_id(&triangle_id, &Transform::Position(Point::new(200.0, 200.0)))
        .unwrap();
    render_and_save(&object, "position");

    object
        .transform_by_id(&heart_id, &Transform::Move(Point::new(50.0, 0.0)))
        .unwrap();
    render_and_save(&object, "move");

    object
        .transform_by_id(&triangle_id, &Transform::Scale(Point::new_symmetric(1.5)))
        .unwrap();
    render_and_save(&object, "scale");

    object
        .transform_by_id(&triangle_id, &Transform::Rotate(15.0_f64.to_radians()))
        .unwrap();
    render_and_save(&object, "rotate");

    object
        .transform_by_id(&triangle_id, &Transform::Color(None))
        .unwrap();
    render_and_save(&object, "color_none");

    object
        .transform_by_id(&triangle_id, &Transform::Visibility(false))
        .unwrap();
    render_and_save(&object, "visibility");
}
