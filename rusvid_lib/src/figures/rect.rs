use usvg::{PathData, PathSegment};

#[inline]
pub fn rect_raw(x: f64, y: f64, width: f64, height: f64) -> [PathSegment; 5] {
    [
        PathSegment::MoveTo { x, y },
        PathSegment::LineTo { x: x + width, y },
        PathSegment::LineTo {
            x: x + width,
            y: y + height,
        },
        PathSegment::LineTo { x, y: y + height },
        PathSegment::ClosePath,
    ]
}

#[inline]
pub fn rect(x: f64, y: f64, width: f64, height: f64) -> PathData {
    let mut path = PathData::with_capacity(5);
    path.extend_from_slice(&rect_raw(x, y, width, height));
    path
}
