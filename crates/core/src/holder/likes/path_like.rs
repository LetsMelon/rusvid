use resvg::usvg::{PathData, PathSegment};

use crate::point::Point;

const DELTA: f64 = 0.0001;

#[derive(Debug, Clone, Copy)]
pub enum PathLike {
    Move(Point),
    Line(Point),
    Close,
}

impl PartialEq<PathLike> for PathLike {
    fn eq(&self, other: &PathLike) -> bool {
        use PathLike::*;

        #[inline(always)]
        fn compare_points(p1: &Point, p2: &Point) -> bool {
            p1.abs_diff_eq(*p2, DELTA)
        }

        match (self, other) {
            (Close, Close) => true,
            (Move(p1), Move(p2)) => compare_points(p1, p2),
            (Line(p1), Line(p2)) => compare_points(p1, p2),
            _ => false,
        }
    }
}

impl PathLike {
    pub fn to_usvg_path_segment(&self) -> PathSegment {
        match self {
            PathLike::Move(point) => PathSegment::MoveTo {
                x: point.x(),
                y: point.y(),
            },
            PathLike::Line(point) => PathSegment::LineTo {
                x: point.x(),
                y: point.y(),
            },
            PathLike::Close => PathSegment::ClosePath,
        }
    }

    pub fn to_usvg_path_segments(path: &Vec<PathLike>) -> Vec<PathSegment> {
        path.iter().map(|p| p.to_usvg_path_segment()).collect()
    }

    pub fn type_equal(&self, other: &PathLike) -> bool {
        match (self, other) {
            (PathLike::Move(_), PathLike::Move(_))
            | (PathLike::Line(_), PathLike::Line(_))
            | (PathLike::Close, PathLike::Close) => true,
            _ => false,
        }
    }

    pub fn extend_path_from_self(&self, path: &mut PathData) {
        match self {
            PathLike::Move(point) => path.push_move_to(point.x(), point.y()),
            PathLike::Line(point) => path.push_line_to(point.x(), point.y()),
            PathLike::Close => path.push_close_path(),
        }
    }

    pub fn extend_path_from_slice(path: &mut PathData, slice: &Vec<Self>) {
        slice
            .iter()
            .for_each(|item| item.extend_path_from_self(path))
    }

    pub fn from_path_segment(other: PathSegment) -> PathLike {
        match other {
            PathSegment::MoveTo { x, y } => PathLike::Move(Point::new(x, y)),
            PathSegment::LineTo { x, y } => PathLike::Line(Point::new(x, y)),
            PathSegment::CurveTo {
                x1: _,
                y1: _,
                x2: _,
                y2: _,
                x: _,
                y: _,
            } => todo!("Conversion for PathSegment::CurveTo is not implemented"),
            PathSegment::ClosePath => PathLike::Close,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ! copied from rusvid_lib/src/utils.rs:51
    fn equal_delta(v1: f64, v2: f64, delta: f64) -> bool {
        let diff = (v1 - v2).abs();
        diff <= delta.abs()
    }

    // ! copied from rusvid_lib/src/figures/circle.rs:111
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

    mod to_usvg_path_segment {
        use super::*;

        #[test]
        fn direct_move() {
            assert!(equal_path_segment(
                PathLike::Move(Point::ZERO).to_usvg_path_segment(),
                PathSegment::MoveTo { x: 0.0, y: 0.0 }
            ));
        }

        #[test]
        fn direct_line() {
            assert!(equal_path_segment(
                PathLike::Line(Point::ZERO).to_usvg_path_segment(),
                PathSegment::LineTo { x: 0.0, y: 0.0 }
            ));
        }

        #[test]
        fn direct_close() {
            assert!(equal_path_segment(
                PathLike::Close.to_usvg_path_segment(),
                PathSegment::ClosePath
            ));
        }
    }

    mod equal {
        use crate::holder::likes::path_like::{PathLike, DELTA};
        use crate::point::Point;

        #[test]
        fn path_like_to_path_like() {
            assert_eq!(PathLike::Close, PathLike::Close);
            assert_eq!(PathLike::Move(Point::ONE), PathLike::Move(Point::ONE));
            assert_eq!(
                PathLike::Line(Point::NEG_ONE),
                PathLike::Line(Point::NEG_ONE)
            );

            // ! DELTA
            assert_eq!(
                PathLike::Move(Point::ONE),
                PathLike::Move(Point::new(1.0 - DELTA, 1.0 + DELTA))
            );
            assert_ne!(
                PathLike::Move(Point::ONE),
                PathLike::Move(Point::new(1.0 - 2.0 * DELTA, 1.0 + 2.0 * DELTA))
            );

            assert_ne!(PathLike::Close, PathLike::Move(Point::ZERO));
            assert_ne!(PathLike::Move(Point::ONE), PathLike::Move(Point::NEG_ONE));
            assert_ne!(PathLike::Line(Point::ZERO), PathLike::Line(Point::NEG_ONE));
        }
    }

    #[test]
    fn type_equal() {
        assert!(PathLike::Move(Point::ONE).type_equal(&PathLike::Move(Point::ZERO)));
        assert!(PathLike::Line(Point::ONE).type_equal(&PathLike::Line(Point::ZERO)));
        assert!(PathLike::Close.type_equal(&PathLike::Close));

        assert!(!PathLike::Move(Point::ONE).type_equal(&PathLike::Line(Point::ZERO)));
        assert!(!PathLike::Line(Point::ONE).type_equal(&PathLike::Close));
        assert!(!PathLike::Close.type_equal(&PathLike::Move(Point::ZERO)));
    }
}
