use std::time::{Duration, Instant};

use rusvid_core::holder::likes::path_like::PathLike;
use rusvid_core::holder::likes::types_like::TypesLike;
use rusvid_core::holder::object::Object;
use rusvid_core::holder::polygon::Polygon;
use rusvid_core::holder::svg_holder::SvgHolder;
use rusvid_core::holder::svg_item::SvgItem;
use rusvid_core::point::Point;

fn main() {
    let size = 500 as f64;
    let segments = 1_000;

    let mut svg = SvgHolder::new();

    let mut paths = Vec::new();
    paths.push(PathLike::Move(Point::ZERO));

    for _ in 0..segments {
        paths.push(PathLike::Line(Point::new(
            (rand::random::<f64>() - 0.5) * size * 2.0,
            (rand::random::<f64>() - 0.5) * size * 2.0,
        )));
    }

    paths.push(PathLike::Close);

    let item = SvgItem::new(Polygon::new(&paths), None);
    svg.add_item(item);

    let object = Object::new(TypesLike::Svg(svg));

    let start = Instant::now();
    let plane = object.render(size as u32, size as u32).unwrap();
    let duration = start.elapsed();

    println!("render took: {} ms", duration.as_millis());

    assert!(plane.pixel(0, 0).is_some());
}
