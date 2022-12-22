use anyhow::Result;
use rusvid_core::holder::likes::color_like::ColorLike;
use rusvid_core::holder::likes::path_like::PathLike;
use rusvid_core::holder::likes::types_like::TypesLike;
use rusvid_core::holder::object::Object;
use rusvid_core::holder::stroke::Stroke;
use rusvid_core::holder::svg_holder::{SvgHolder, SvgItem};
use rusvid_core::holder::transform::{Transform, TransformLogic};
use rusvid_core::point::Point;

#[inline]
fn render_and_save(object: &Object, name: &str) -> Result<()> {
    let plane = object.render(300, 300)?;
    let path = format!("example_simple_transform_{}.png", name);
    plane.save_as_png(path)
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
        Some(ColorLike::Color([0, 255, 100, 255])),
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
        Some(ColorLike::Color([255, 0, 0, 255])),
    ));

    let mut object = Object::new(TypesLike::Svg(svg));
    render_and_save(&object, "output").unwrap();

    object
        .transform_by_id(
            &triangle_id,
            &Transform::Stroke(Some(Stroke {
                paint: ColorLike::Color([100, 50, 120, 255]),
                width: 1.75,
                ..Stroke::default()
            })),
        )
        .unwrap();
    render_and_save(&object, "stroke").unwrap();

    object
        .transform_by_id(
            &heart_id,
            &Transform::Color(Some(ColorLike::Color([230, 57, 70, 255]))),
        )
        .unwrap();
    render_and_save(&object, "color").unwrap();

    object
        .transform_by_id(&triangle_id, &Transform::Position(Point::new(200.0, 200.0)))
        .unwrap();
    render_and_save(&object, "position").unwrap();

    object
        .transform_by_id(&heart_id, &Transform::Move(Point::new(50.0, 0.0)))
        .unwrap();
    render_and_save(&object, "move").unwrap();

    object
        .transform_by_id(&triangle_id, &Transform::Color(None))
        .unwrap();
    render_and_save(&object, "color_none").unwrap();

    object
        .transform_by_id(&triangle_id, &Transform::Visibility(false))
        .unwrap();
    render_and_save(&object, "visibility").unwrap();
}
