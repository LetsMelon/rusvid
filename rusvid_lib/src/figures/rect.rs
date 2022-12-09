use rusvid_core::point::Point;
use usvg::{PathData, PathSegment};

use super::utils::extend_path_from_slice;

#[inline]
pub fn rect_raw(position: Point, size: Point) -> Vec<PathSegment> {
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
    .to_vec()
}

#[inline]
pub fn rect(position: Point, size: Point) -> PathData {
    let mut path = PathData::new();

    let rect_path = rect_raw(position, size);
    extend_path_from_slice(&mut path, rect_path);

    path
}
