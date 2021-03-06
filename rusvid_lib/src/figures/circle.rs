use usvg::{PathData, PathSegment};

use crate::utils::equal_delta;

#[inline]
fn sin_radius(angle: f64, radius: f64) -> f64 {
    angle.sin() * radius
}

#[inline]
fn cos_radius(angle: f64, radius: f64) -> f64 {
    angle.cos() * radius
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Point2d {
    x: f64,
    y: f64,
}

impl PartialEq for Point2d {
    fn eq(&self, other: &Self) -> bool {
        equal_delta(self.x, other.x, 0.1) && equal_delta(self.y, other.y, 0.1)
    }
}
impl Eq for Point2d {}

#[inline]
pub fn arc_segment(c_x: f64, c_y: f64, radius: f64, a1: f64, a2: f64) -> (PathSegment, Point2d) {
    let start_angle = a1 * (std::f64::consts::PI / 180.0);
    let end_angle = a2 * (std::f64::consts::PI / 180.0);
    let half_angle = (end_angle - start_angle) / 2.0;
    let k = (4.0 / 3.0) * ((1.0 - half_angle.cos()) / half_angle.sin());

    let p1x = c_x + cos_radius(start_angle, radius);
    let p1y = c_y + sin_radius(start_angle, radius);
    let p4x = c_x + cos_radius(end_angle, radius);
    let p4y = c_y + sin_radius(end_angle, radius);
    let p2x = p1x - (k * sin_radius(start_angle, radius));
    let p2y = p1y + (k * cos_radius(start_angle, radius));
    let p3x = p4x + (k * sin_radius(end_angle, radius));
    let p3y = p4y - (k * cos_radius(end_angle, radius));

    (
        PathSegment::CurveTo {
            x1: p2x,
            y1: p2y,
            x2: p3x,
            y2: p3y,
            x: p4x,
            y: p4y,
        },
        Point2d { x: p1x, y: p1y },
    )
}

#[inline]
pub fn arc(c_x: f64, c_y: f64, radius: f64, start_angle: f64, end_angle: f64) -> Vec<PathSegment> {
    let mut segments = Vec::new();

    let mut a2;
    let mut a1 = start_angle;

    let mut total_angle = (360.0_f64).min(end_angle - start_angle);
    while total_angle > 0.0 {
        a2 = a1 + total_angle.min(90.0);

        let (segment, point_2d) = arc_segment(c_x, c_y, radius, a1, a2);
        let point = if a1 == start_angle {
            PathSegment::MoveTo {
                x: point_2d.x,
                y: point_2d.y,
            }
        } else {
            PathSegment::LineTo {
                x: point_2d.x,
                y: point_2d.y,
            }
        };
        segments.push(point);
        segments.push(segment);

        total_angle -= (a2 - a1).abs();
        a1 = a2;
    }

    segments
}

#[inline]
pub fn circle_raw(c_x: f64, c_y: f64, radius: f64) -> Vec<PathSegment> {
    let mut segments = arc(c_x, c_y, radius, 0.0, 360.0);

    segments.push(PathSegment::ClosePath);

    segments
}

#[inline]
pub fn circle(c_x: f64, c_y: f64, radius: f64) -> PathData {
    let mut path = PathData::with_capacity(9);
    path.extend_from_slice(&circle_raw(c_x, c_y, radius));
    path
}

#[cfg(test)]
mod tests {
    use crate::figures::circle::*;
    use crate::utils::equal_delta;

    fn equal_path_segment(p1: PathSegment, p2: PathSegment) -> bool {
        match (p1, p2) {
            (PathSegment::ClosePath, PathSegment::ClosePath) => true,
            (PathSegment::MoveTo { x: x1, y: y1 }, PathSegment::MoveTo { x: x2, y: y2 }) => {
                equal_delta(x1, x2, 0.1) && equal_delta(y1, y2, 0.1)
            }
            (PathSegment::LineTo { x: x1, y: y1 }, PathSegment::LineTo { x: x2, y: y2 }) => {
                equal_delta(x1, x2, 0.1) && equal_delta(y1, y2, 0.1)
            }
            (
                PathSegment::CurveTo {
                    x1: x11,
                    y1: y11,
                    x2: x12,
                    y2: y12,
                    x: x1,
                    y: y1,
                },
                PathSegment::CurveTo {
                    x1: x21,
                    y1: y21,
                    x2: x22,
                    y2: y22,
                    x: x2,
                    y: y2,
                },
            ) => {
                equal_delta(x11, x21, 0.01)
                    && equal_delta(y11, y21, 0.01)
                    && equal_delta(x12, x22, 0.01)
                    && equal_delta(y12, y22, 0.01)
                    && equal_delta(x1, x2, 0.1)
                    && equal_delta(y1, y2, 0.1)
            }
            (_, _) => false,
        }
    }

    #[test]
    fn calculate_circle() {
        let circle = circle_raw(0.0, 0.0, 100.0);

        assert_eq!(circle.len(), 9);

        assert!(equal_path_segment(
            circle[0],
            PathSegment::MoveTo { x: 100.0, y: 0.0 }
        ));
        assert!(equal_path_segment(
            circle[1],
            PathSegment::CurveTo {
                x1: 100.0,
                y1: 55.2284,
                x2: 55.2284,
                y2: 100.0,
                x: 0.0,
                y: 100.0
            }
        ));
        assert!(equal_path_segment(
            circle[2],
            PathSegment::LineTo { x: 0.0, y: 100.0 }
        ));
        assert!(equal_path_segment(
            circle[3],
            PathSegment::CurveTo {
                x1: -55.2284,
                y1: 100.0,
                x2: -100.0,
                y2: 55.2284,
                x: -100.0,
                y: 0.0
            }
        ));
        assert!(equal_path_segment(
            circle[4],
            PathSegment::LineTo { x: -100.0, y: 0.0 }
        ));
        assert!(equal_path_segment(
            circle[5],
            PathSegment::CurveTo {
                x1: -100.0,
                y1: -55.2284,
                x2: -55.2284,
                y2: -100.0,
                x: 0.0,
                y: -100.0
            }
        ));
        assert!(equal_path_segment(
            circle[6],
            PathSegment::LineTo { x: 0.0, y: -100.0 }
        ));
        assert!(equal_path_segment(
            circle[7],
            PathSegment::CurveTo {
                x1: 55.2284,
                y1: -100.0,
                x2: 100.0,
                y2: -55.2284,
                x: 100.0,
                y: 0.0
            }
        ));
        assert!(equal_path_segment(circle[8], PathSegment::ClosePath));
    }
}
