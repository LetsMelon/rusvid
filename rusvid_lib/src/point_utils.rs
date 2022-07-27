use usvg::PathSegment;

use crate::animation::curves::Points;

pub fn set_path(segments: &mut [PathSegment], cords: Points) {
    for seg in segments {
        match seg {
            PathSegment::MoveTo { x, y } => {
                apply_to(x, y, &cords);
            }
            PathSegment::LineTo { x, y } => {
                apply_to(x, y, &cords);
            }
            PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => {
                apply_to(x1, y1, &cords);
                apply_to(x2, y2, &cords);
                apply_to(x, y, &cords);
            }
            PathSegment::ClosePath => {}
        }
    }
}

#[inline]
fn apply_to(x: &mut f64, y: &mut f64, cords: &Points) {
    *x = cords.x();
    if let Points::Point2d(_, _) = cords {
        *y = cords.y();
    }
}
