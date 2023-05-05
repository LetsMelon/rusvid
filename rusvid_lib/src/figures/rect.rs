use rusvid_core::holder::likes::PathLike;
use rusvid_core::holder::polygon::Polygon;
use rusvid_core::point::Point;

pub fn rect_raw(position: Point, size: Point) -> Vec<PathLike> {
    let x = position.x();
    let y = position.y();
    let width = size.x();
    let height = size.y();

    vec![
        PathLike::Move(Point::new(x, y)),
        PathLike::Line(Point::new(x + width, y)),
        PathLike::Line(Point::new(x + width, y + height)),
        PathLike::Line(Point::new(x, y + height)),
    ]
}

pub fn rect(position: Point, size: Point) -> Polygon {
    let mut path = rect_raw(position, size);

    path.push(PathLike::Close);

    Polygon::new(&path)
}
