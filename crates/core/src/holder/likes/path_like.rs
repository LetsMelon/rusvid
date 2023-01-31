use geo::{Centroid, LineString, Polygon};
use resvg::usvg::{PathData, PathSegment};

use crate::holder::likes::utils::{coord2_to_point, point_to_coord2};
use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::point::Point;

// TODO merge delta constants
const DELTA: f64 = 0.0001;
const BOUNDING_BOX_STEPS: u32 = 10;

#[derive(Debug, Clone, Copy)]
pub enum PathLike {
    Move(Point),
    Line(Point),
    /// end_point, control_point_start, control_point_end
    CurveTo(Point, Point, Point),
    Close,
}

impl PartialEq<PathLike> for PathLike {
    fn eq(&self, other: &PathLike) -> bool {
        use PathLike::*;

        fn compare_points(p1: &Point, p2: &Point) -> bool {
            p1.abs_diff_eq(*p2, DELTA)
        }

        match (self, other) {
            (Close, Close) => true,
            (Move(p1), Move(p2)) => compare_points(p1, p2),
            (Line(p1), Line(p2)) => compare_points(p1, p2),
            (CurveTo(pe1, pc11, pc12), CurveTo(pe2, pc21, pc22)) => {
                compare_points(pe1, pe2) && compare_points(pc11, pc21) && compare_points(pc12, pc22)
            }
            _ => false,
        }
    }
}

impl PathLike {
    pub(crate) fn to_geo_polygon(path: &[PathLike]) -> Polygon {
        let mut last_move = Point::new_symmetric(0.0);
        let mut last_point = Point::new_symmetric(0.0);
        let exterior = path
            .iter()
            .map(|path_like| match *path_like {
                PathLike::Move(p) => {
                    last_move = p.clone();
                    last_point = p.clone();
                    vec![(p.x(), p.y())]
                }
                PathLike::Line(p) => {
                    last_point = p.clone();
                    vec![(p.x(), p.y())]
                }
                PathLike::CurveTo(end, c_s, c_e) => {
                    use flo_curves::bezier::Curve;
                    use flo_curves::*;

                    let curve = Curve::from_points(
                        point_to_coord2(&last_point),
                        (point_to_coord2(&c_s), point_to_coord2(&c_e)),
                        point_to_coord2(&end),
                    );

                    let mut lines_on_curve = vec![];
                    for i in 0..BOUNDING_BOX_STEPS {
                        let t = (i as f64) / (BOUNDING_BOX_STEPS as f64);
                        let pos = curve.point_at_pos(t);
                        let as_point = coord2_to_point(&pos);

                        lines_on_curve.push((as_point.x(), as_point.y()));

                        last_point = as_point.clone();
                    }

                    lines_on_curve
                }
                PathLike::Close => {
                    last_point = last_move.clone();
                    vec![(last_move.x(), last_move.y())]
                }
            })
            .flatten()
            .collect::<Vec<(f64, f64)>>();

        Polygon::new(LineString::from(exterior), vec![])
    }

    pub(crate) fn get_center(path: &[PathLike]) -> Point {
        let polygon = PathLike::to_geo_polygon(&path);
        let center = polygon
            .centroid()
            .map(|cord| Point::new(cord.x(), cord.y()))
            .unwrap_or({
                let bounding_box = Self::bounding_box(&path);

                let size = bounding_box.1 - bounding_box.0;
                let center = bounding_box.0 + size / 2.0;

                center
            });

        center
    }

    pub fn bounding_box(path: &[PathLike]) -> (Point, Point) {
        let mut smaller_corner = match path[0] {
            PathLike::Move(p) => p,
            _ => todo!(),
        };
        let mut bigger_corner = smaller_corner;

        let mut last_point = smaller_corner;
        path.iter().for_each(|path| {
            fn compare_and_set(value: &Point, smaller: &mut Point, bigger: &mut Point) {
                if smaller.x() > value.x() {
                    *smaller.x_mut() = value.x();
                }
                if smaller.y() > value.y() {
                    *smaller.y_mut() = value.y();
                }

                if bigger.x() < value.x() {
                    *bigger.x_mut() = value.x();
                }
                if bigger.y() < value.y() {
                    *bigger.y_mut() = value.y();
                }
            }

            last_point = match path {
                PathLike::Move(p) => {
                    compare_and_set(p, &mut smaller_corner, &mut bigger_corner);
                    *p
                }
                PathLike::Line(p) => {
                    compare_and_set(p, &mut smaller_corner, &mut bigger_corner);
                    *p
                }
                PathLike::CurveTo(p, c1, c2) => {
                    use flo_curves::bezier::Curve;
                    use flo_curves::*;

                    compare_and_set(p, &mut smaller_corner, &mut bigger_corner);

                    let curve = Curve::from_points(
                        point_to_coord2(&last_point),
                        (point_to_coord2(c1), point_to_coord2(c2)),
                        point_to_coord2(p),
                    );

                    for i in 0..BOUNDING_BOX_STEPS {
                        let t = (i as f64) / (BOUNDING_BOX_STEPS as f64);
                        let pos = curve.point_at_pos(t);
                        let as_point = coord2_to_point(&pos);
                        compare_and_set(&as_point, &mut smaller_corner, &mut bigger_corner);
                    }

                    *p
                }
                _ => last_point,
            };
        });

        (smaller_corner, bigger_corner)
    }

    pub fn type_equal(&self, other: &PathLike) -> bool {
        debug_assert_eq!(std::mem::variant_count::<PathLike>(), 4);

        matches!(
            (self, other),
            (PathLike::Move(_), PathLike::Move(_))
                | (PathLike::Line(_), PathLike::Line(_))
                | (PathLike::Close, PathLike::Close)
                | (PathLike::CurveTo(_, _, _), PathLike::CurveTo(_, _, _))
        )
    }

    pub fn extend_path_from_self(&self, path: &mut PathData) {
        match self {
            PathLike::Move(point) => path.push_move_to(point.x(), point.y()),
            PathLike::Line(point) => path.push_line_to(point.x(), point.y()),
            PathLike::CurveTo(pe, pc1, pc2) => {
                path.push_curve_to(pc1.x(), pc1.y(), pc2.x(), pc2.y(), pe.x(), pe.y())
            }
            PathLike::Close => path.push_close_path(),
        }
    }

    pub fn extend_path_from_slice(path: &mut PathData, slice: &[Self]) {
        slice
            .iter()
            .for_each(|item| item.extend_path_from_self(path))
    }

    pub fn from_path_segment(other: PathSegment) -> PathLike {
        match other {
            PathSegment::MoveTo { x, y } => PathLike::Move(Point::new(x, y)),
            PathSegment::LineTo { x, y } => PathLike::Line(Point::new(x, y)),
            PathSegment::CurveTo {
                x1,
                y1,
                x2,
                y2,
                x,
                y,
            } => PathLike::CurveTo(Point::new(x, y), Point::new(x1, y1), Point::new(x2, y2)),
            PathSegment::ClosePath => PathLike::Close,
        }
    }
}

impl TranslateIntoResvgGeneric<resvg::usvg::PathSegment> for PathLike {
    fn translate(&self) -> resvg::usvg::PathSegment {
        match self {
            PathLike::Move(point) => PathSegment::MoveTo {
                x: point.x(),
                y: point.y(),
            },
            PathLike::Line(point) => PathSegment::LineTo {
                x: point.x(),
                y: point.y(),
            },
            PathLike::CurveTo(pe, pc1, pc2) => PathSegment::CurveTo {
                x1: pc1.x(),
                y1: pc1.y(),
                x2: pc2.x(),
                y2: pc2.y(),
                x: pe.x(),
                y: pe.y(),
            },
            PathLike::Close => PathSegment::ClosePath,
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
                PathLike::Move(Point::ZERO).translate(),
                PathSegment::MoveTo { x: 0.0, y: 0.0 }
            ));
        }

        #[test]
        fn direct_line() {
            assert!(equal_path_segment(
                PathLike::Line(Point::ZERO).translate(),
                PathSegment::LineTo { x: 0.0, y: 0.0 }
            ));
        }

        #[test]
        fn direct_close() {
            assert!(equal_path_segment(
                PathLike::Close.translate(),
                PathSegment::ClosePath
            ));
        }

        #[test]
        fn direct_curve_to() {
            assert!(equal_path_segment(
                PathLike::CurveTo(Point::ZERO, Point::ONE, Point::NEG_ONE).translate(),
                PathSegment::CurveTo {
                    x1: 1.0,
                    y1: 1.0,
                    x2: -1.0,
                    y2: -1.0,
                    x: 0.0,
                    y: 0.0
                }
            ))
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
            assert_eq!(
                PathLike::CurveTo(Point::ONE, Point::new(100.0, -50.0), Point::ZERO),
                PathLike::CurveTo(Point::ONE, Point::new(100.0, -50.0), Point::ZERO)
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
            assert_ne!(
                PathLike::Line(Point::ZERO),
                PathLike::CurveTo(Point::ONE, Point::new(100.0, -50.0), Point::ZERO)
            );
            assert_ne!(
                PathLike::CurveTo(Point::ONE, Point::new(100.0, -50.0), Point::ZERO),
                PathLike::Line(Point::NEG_ONE)
            );
        }
    }

    #[test]
    fn type_equal() {
        assert!(PathLike::Move(Point::ONE).type_equal(&PathLike::Move(Point::ZERO)));
        assert!(PathLike::Line(Point::ONE).type_equal(&PathLike::Line(Point::ZERO)));
        assert!(PathLike::Close.type_equal(&PathLike::Close));
        assert!(PathLike::CurveTo(Point::ZERO, Point::ONE, Point::NEG_ONE)
            .type_equal(&PathLike::CurveTo(Point::ONE, Point::ZERO, Point::ZERO)));

        assert!(!PathLike::Move(Point::ONE).type_equal(&PathLike::Line(Point::ZERO)));
        assert!(!PathLike::Line(Point::ONE).type_equal(&PathLike::Close));
        assert!(!PathLike::Close.type_equal(&PathLike::Move(Point::ZERO)));
    }
}
