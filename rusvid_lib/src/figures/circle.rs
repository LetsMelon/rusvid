use std::f64::consts::PI;

const TWO_PI: f64 = PI * 2.0;

use rusvid_core::holder::likes::PathLike;
use rusvid_core::holder::polygon::Polygon;
use rusvid_core::point::Point;

pub fn circle_raw(position: Point, radius: f64) -> Vec<PathLike> {
    let circumference = radius * TWO_PI;
    let n = ((circumference / 32.0) as usize).max(32);

    let alpha = TWO_PI / (n as f64);

    let mut segments = Vec::new();

    segments.push(PathLike::Move(position + Point::new(radius, 0.0)));

    for i in 0..n {
        let i = i as f64;
        segments.push(PathLike::Line(
            position + Point::new((alpha * i).cos() * radius, (alpha * i).sin() * radius),
        ));
    }

    segments
}

pub fn circle(position: Point, radius: f64) -> Polygon {
    let mut path = circle_raw(position, radius);

    path.push(PathLike::Close);

    Polygon::new(&path)
}

/*
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
        let circle = circle_raw(Point::ZERO, 100.0);

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
 */
