use rusvid_core::point::Point;
use usvg::{PathData, PathSegment};

#[inline]
pub fn rect_raw(position: Point, size: Point) -> [PathSegment; 5] {
    let x = position.x();
    let y = position.y();
    let width = size.x();
    let height = size.y();

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
pub fn rect(position: Point, size: Point) -> PathData {
    let mut path = PathData::with_capacity(5);
    path.extend_from_slice(&rect_raw(position, size));
    path
}
