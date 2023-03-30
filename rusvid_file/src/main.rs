use std::fs::File;

use rusvid_lib::figures::prelude::circle;
use rusvid_lib::prelude::holder::gradient::base::BaseGradient;
use rusvid_lib::prelude::holder::gradient::linear::LinearGradient;
use rusvid_lib::prelude::holder::gradient::stop::Stop;
use rusvid_lib::prelude::holder::likes::{ColorLike, PathLike, TypesLike};
use rusvid_lib::prelude::holder::object::Object;
use rusvid_lib::prelude::holder::svg_holder::SvgHolder;
use rusvid_lib::prelude::holder::svg_item::SvgItem;
use rusvid_lib::prelude::{FrameRenderer, Pixel};
use rusvid_lib::types::Point;

mod wrapper;

fn main() {
    let mut svg = SvgHolder::new();

    let rect_size = Point::new_symmetric(150.0);
    let rect_pos = Point::new(100.0, 50.0);
    let triangle = SvgItem::new(
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
    svg.add_item(triangle);

    let heart = SvgItem::new(
        circle(Point::new(45.0, 35.0), 1000.0),
        Some(ColorLike::LinearGradient(LinearGradient::new(
            BaseGradient::new_from_colors(vec![
                Pixel::new(255, 0, 0, 255),
                Pixel::new(255, 100, 0, 255),
            ]),
        ))),
    );
    // svg.add_item(heart.bounding_box_rect());
    svg.add_item(heart);

    let object = Object::new(TypesLike::Svg(svg));

    let file = File::create("out.yaml").unwrap();
    serde_yaml::to_writer(file, &object).unwrap();

    // let mut renderer =
    //     FrameRenderer::new_with_file_type("./out", rusvid_lib::prelude::FrameImageFormat::Bmp);
    // renderer.render(comp.translate()).unwrap();
}
