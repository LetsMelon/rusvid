use rusvid_core::holder::backend::BackendType;
use rusvid_core::holder::gradient::base::BaseGradient;
use rusvid_core::holder::gradient::linear::LinearGradient;
use rusvid_core::holder::gradient::radial::RadialGradient;
use rusvid_core::holder::gradient::stop::Stop;
use rusvid_core::holder::likes::color_like::ColorLike;
use rusvid_core::holder::likes::path_like::PathLike;
use rusvid_core::holder::likes::types_like::TypesLike;
use rusvid_core::holder::object::Object;
use rusvid_core::holder::svg_holder::SvgHolder;
use rusvid_core::holder::svg_item::SvgItem;
use rusvid_core::pixel::Pixel;
use rusvid_core::point::Point;

const SIZE: u32 = 300;

fn main() {
    let mut svg = SvgHolder::new();

    let rect_size = Point::new_symmetric(150.0);
    let rect_pos = Point::new(100.0, 50.0);
    let rect = SvgItem::new(
        vec![
            PathLike::Move(rect_pos),
            PathLike::Line(rect_size * Point::new(1.0, 0.0) + rect_pos),
            PathLike::Line(rect_size * Point::new(1.0, 1.0) + rect_pos),
            PathLike::Line(rect_size * Point::new(0.0, 1.0) + rect_pos),
            PathLike::Close,
        ],
        Some(ColorLike::LinearGradient(LinearGradient::new(
            BaseGradient::new(vec![
                Stop::new(Pixel::new(2, 0, 36, 255), 0.0),
                Stop::new(Pixel::new(9, 9, 121, 255), 0.35),
                Stop::new(Pixel::new(0, 212, 255, 255), 1.0),
            ]),
        ))),
    );

    // svg.add_item(rect.bounding_box_rect());
    let rect_id = svg.add_item(rect);
    println!("rect: {rect_id}");

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
        Some(ColorLike::LinearGradient(LinearGradient::new(
            BaseGradient::new_from_colors(vec![
                Pixel::new(255, 0, 0, 255),
                Pixel::new(255, 100, 0, 255),
            ]),
        ))),
        // Some(ColorLike::RadialGradient(RadialGradient::new(
        //     Point::new(100.0, 100.0),
        //     50.0,
        //     Point::new(400.0, 400.0),
        //     BaseGradient::new_from_colors(vec![
        //         Pixel::new(255, 100, 0, 255),
        //         Pixel::new(255, 10, 200, 255),
        //     ]),
        // ))),
    );
    // svg.add_item(heart.bounding_box_rect());
    let heart_id = svg.add_item(heart);
    println!("heart: {heart_id}");

    // custom depth
    // svg.set_item_depth(rect_id, 1);
    // svg.set_item_depth(heart_id, 0);

    let mut object = Object::new(TypesLike::Svg(svg));
    object.set_backend(BackendType::Cairo);

    let plane = object.render(SIZE, SIZE).unwrap();
    plane.save_as_png("output.png").unwrap();
}
