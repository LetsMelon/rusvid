use rusvid_core::holder::likes::color_like::ColorLike;
use rusvid_core::holder::likes::path_like::PathLike;
use rusvid_core::holder::likes::types_like::TypesLike;
use rusvid_core::holder::object::Object;
use rusvid_core::holder::svg_holder::{SvgHolder, SvgItem};
use rusvid_core::point::Point;

fn main() {
    let mut svg = SvgHolder::new();
    let rect = SvgItem::new(
        vec![
            PathLike::Move(Point::new(100.0, 100.0)),
            PathLike::Line(Point::new(150.0, 100.0)),
            PathLike::Line(Point::new(120.0, 150.0)),
            PathLike::Close,
        ],
        ColorLike::Color([0, 255, 100, 255]),
    );
    // svg.add_item(rect.bounding_box_rect());
    svg.add_item(rect);

    let heart = SvgItem::new(
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
        ColorLike::Color([255, 0, 0, 255]),
    );
    // svg.add_item(heart.bounding_box_rect());
    svg.add_item(heart);

    let object = Object::new(TypesLike::Svg(svg));

    let plane = object.render(300, 300).unwrap();
    plane.save_as_png("output.png").unwrap();
}
