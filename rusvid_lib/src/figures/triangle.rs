use resvg::usvg::{PathData, PathSegment};
use rusvid_core::point::Point;

use super::utils::extend_path_from_slice;

const EQ_T_FACTOR: f64 = 1000.0 / 866.025403784438;

pub fn equilateral_triangle_raw(p: Point, side_length: f64) -> Vec<PathSegment> {
    let x = p.x();
    let y = p.y();

    [
        PathSegment::MoveTo { x, y },
        PathSegment::LineTo {
            x: x + side_length,
            y,
        },
        PathSegment::LineTo {
            x: x + side_length / 2.0,
            y: y + y / EQ_T_FACTOR,
        },
        PathSegment::ClosePath,
    ]
    .to_vec()
}

pub fn equilateral_triangle(p: Point, side_length: f64) -> PathData {
    let mut path = PathData::new();

    let triangle_path = equilateral_triangle_raw(p, side_length);
    extend_path_from_slice(&mut path, triangle_path);

    path
}
