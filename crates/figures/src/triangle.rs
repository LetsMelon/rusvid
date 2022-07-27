use usvg::{PathData, PathSegment};

const EQ_T_FACTOR: f64 = 1000.0 / 866.025403784438;

pub fn equilateral_triangle_raw(x: f64, y: f64, side_length: f64) -> [PathSegment; 4] {
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
}

pub fn equilateral_triangle(x: f64, y: f64, side_length: f64) -> PathData {
    let mut path = PathData::with_capacity(4);
    path.extend_from_slice(&equilateral_triangle_raw(x, y, side_length));
    path
}
