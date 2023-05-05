use rusvid_core::holder::likes::PathLike;
use rusvid_core::holder::polygon::Polygon;
use rusvid_core::point::Point;

// ? magic number ?
const EQ_T_FACTOR: f64 = 1000.0 / 866.025403784438;

pub fn equilateral_triangle_raw(p: Point, side_length: f64) -> Vec<PathLike> {
    let x = p.x();
    let y = p.y();

    vec![
        PathLike::Move(Point::new(x, y)),
        PathLike::Line(Point::new(x + side_length, y)),
        PathLike::Line(Point::new(x + side_length / 2.0, y + y / EQ_T_FACTOR)),
    ]
}

pub fn equilateral_triangle(p: Point, side_length: f64) -> Polygon {
    let mut path = equilateral_triangle_raw(p, side_length);

    path.push(PathLike::Close);

    Polygon::new(&path)
}
