use resvg::usvg::{PathData, PathSegment};

use super::utils::extend_path_from_slice;

const EQ_T_FACTOR: f64 = 1000.0 / 866.025403784438;

pub fn equilateral_triangle_raw(x: f64, y: f64, side_length: f64) -> Vec<PathSegment> {
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

// TODO use Point instead of separate `x` & `y`
pub fn equilateral_triangle(x: f64, y: f64, side_length: f64) -> PathData {
    let mut path = PathData::new();

    let triangle_path = equilateral_triangle_raw(x, y, side_length);
    extend_path_from_slice(&mut path, triangle_path);

    path
}
