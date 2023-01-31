use crate::point::Point;

pub fn point_to_coord2(p: &Point) -> flo_curves::Coord2 {
    flo_curves::Coord2(p.x(), p.y())
}

pub fn coord2_to_point(c: &flo_curves::Coord2) -> Point {
    use flo_curves::*;

    Point::new(c.x(), c.y())
}
