use resvg::usvg::{PathData, PathSegment};

#[inline]
pub fn extend_path_from_slice(path: &mut PathData, slice: Vec<PathSegment>) {
    slice.iter().for_each(|p| match *p {
        PathSegment::MoveTo { x, y } => path.push_move_to(x, y),
        PathSegment::LineTo { x, y } => path.push_line_to(x, y),
        PathSegment::CurveTo {
            x1,
            y1,
            x2,
            y2,
            x,
            y,
        } => path.push_curve_to(x1, y1, x2, y2, x, y),
        PathSegment::ClosePath => path.push_close_path(),
    });
}
