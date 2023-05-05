use geo::Centroid;

use crate::holder::likes::utils::{coord2_to_point, point_to_coord2};
use crate::holder::likes::PathLike;
use crate::holder::transform::{RotationPoint, Transform, TransformError, TransformLogic};
use crate::point::Point;

const BOUNDING_BOX_STEPS: i32 = 10;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
#[repr(transparent)]
pub struct Polygon(Vec<PathLike>);

impl Polygon {
    pub fn new(path: &[PathLike]) -> Self {
        Polygon(path.to_vec())
    }

    pub fn iter(&self) -> impl Iterator<Item = &PathLike> + '_ {
        self.0.iter()
    }

    pub fn path(&self) -> &[PathLike] {
        &self.0
    }

    pub fn path_mut(&mut self) -> &mut [PathLike] {
        &mut self.0
    }

    pub fn center(&self) -> Point {
        let polygon = self.as_geo_polygon();
        let center = polygon
            .centroid()
            .map(|cord| Point::new(cord.x(), cord.y()))
            .unwrap_or({
                let bounding_box = self.bounding_box();

                let size = bounding_box.1 - bounding_box.0;
                let center = bounding_box.0 + size / 2.0;

                center
            });

        center
    }

    pub fn bounding_box(&self) -> (Point, Point) {
        let mut smaller_corner = match self.0[0] {
            PathLike::Move(p) => p,
            item => todo!("unhandled type: {:?}", item),
        };
        let mut bigger_corner = smaller_corner;

        let mut last_point = smaller_corner;
        self.0.iter().for_each(|path| {
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

                    fn point_to_coord2(p: &Point) -> Coord2 {
                        Coord2(p.x(), p.y())
                    }

                    fn coord2_to_point(c: &Coord2) -> Point {
                        Point::new(c.x(), c.y())
                    }

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

    pub fn bounding_box_polygon(&self) -> Polygon {
        let (smaller_corner, bigger_corner) = self.bounding_box();

        // TODO optimize math
        let a = PathLike::Move(smaller_corner);
        let b = PathLike::Line(
            smaller_corner + Point::new(bigger_corner.x() - smaller_corner.x(), 0.0),
        );
        let c = PathLike::Line(
            smaller_corner
                + Point::new(
                    bigger_corner.x() - smaller_corner.x(),
                    bigger_corner.y() - smaller_corner.y(),
                ),
        );
        let d = PathLike::Line(Point::new(
            smaller_corner.x(),
            (smaller_corner
                + Point::new(
                    bigger_corner.x() - smaller_corner.x(),
                    bigger_corner.y() - smaller_corner.y(),
                ))
            .y(),
        ));

        Polygon(vec![a, b, c, d, PathLike::Close])
    }

    pub(crate) fn as_geo_polygon(&self) -> geo::Polygon {
        let mut last_move = Point::new_symmetric(0.0);
        let mut last_point = Point::new_symmetric(0.0);
        let exterior = self
            .0
            .iter()
            .map(|path_like| match *path_like {
                PathLike::Move(p) => {
                    last_move = p.clone();
                    last_point = p.clone();
                    vec![p.as_tuple()]
                }
                PathLike::Line(p) => {
                    last_point = p.clone();
                    vec![p.as_tuple()]
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

                        lines_on_curve.push(as_point.as_tuple());

                        last_point = as_point.clone();
                    }

                    lines_on_curve
                }
                PathLike::Close => {
                    last_point = last_move.clone();
                    vec![last_move.as_tuple()]
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        geo::Polygon::new(geo::LineString::from(exterior), vec![])
    }

    pub(crate) fn from_geo_polygon(polygon: geo::Polygon) -> Self {
        todo!()
    }
}

impl TransformLogic for Polygon {
    fn transform(&mut self, transformation: &Transform) -> Result<(), TransformError> {
        match transformation {
            Transform::Move(point) => self.0.iter_mut().for_each(|p| match p {
                PathLike::Move(og_p) => *og_p = *og_p + *point,
                PathLike::Line(og_p) => *og_p = *og_p + *point,
                PathLike::CurveTo(end, c_s, c_e) => {
                    *end = *end + *point;
                    *c_s = *c_s + *point;
                    *c_e = *c_e + *point;
                }
                PathLike::Close => (),
            }),
            Transform::Position(position) => match self.0[0] {
                PathLike::Move(point) => {
                    let offset = *position - point;
                    self.transform(&Transform::Move(offset))?;
                }
                // TODO use better error
                _ => Err(TransformError::NoItem("idk".to_string()))?,
            },
            Transform::Scale(factor) => {
                let center = PathLike::get_center(&self.0);
                let factor = *factor;

                self.0.iter_mut().for_each(|p| match p {
                    PathLike::Move(value) | PathLike::Line(value) => {
                        let v = (*value - center) * factor;
                        let pos = center + v;

                        *value = pos;
                    }
                    PathLike::CurveTo(end, c_s, c_e) => {
                        let p_e = center + (*end - center) * factor;
                        let p_c_s = center + (*c_s - center) * factor;
                        let p_c_e = center + (*c_e - center) * factor;

                        *end = p_e;
                        *c_s = p_c_s;
                        *c_e = p_c_e;
                    }
                    PathLike::Close => (),
                });
            }
            Transform::Rotate((angle, rot_point)) => {
                let angle = *angle;

                let rotation_point = match rot_point {
                    RotationPoint::Center => self.center(),
                    RotationPoint::Custom(p) => *p,
                };

                self.0.iter_mut().for_each(|p| match p {
                    PathLike::Move(value) | PathLike::Line(value) => {
                        let v = *value - rotation_point;

                        let x = angle.cos() * v.x() - angle.sin() * v.y();
                        let y = angle.sin() * v.x() + angle.cos() * v.y();

                        let pos = rotation_point + Point::new(x, y);

                        *value = pos;
                    }
                    PathLike::CurveTo(end, c_s, c_e) => {
                        let v_e = *end - rotation_point;
                        let v_c_s = *c_s - rotation_point;
                        let v_c_e = *c_e - rotation_point;

                        *end = rotation_point
                            + Point::new(
                                angle.cos() * v_e.x() - angle.sin() * v_e.y(),
                                angle.sin() * v_e.x() + angle.cos() * v_e.y(),
                            );
                        *c_s = rotation_point
                            + Point::new(
                                angle.cos() * v_c_s.x() - angle.sin() * v_c_s.y(),
                                angle.sin() * v_c_s.x() + angle.cos() * v_c_s.y(),
                            );
                        *c_e = rotation_point
                            + Point::new(
                                angle.cos() * v_c_e.x() - angle.sin() * v_c_e.y(),
                                angle.sin() * v_c_e.x() + angle.cos() * v_c_e.y(),
                            );
                    }
                    PathLike::Close => (),
                });
            }
            Transform::Visibility(_) | Transform::Color(_) | Transform::Stroke(_) => (),
        }

        Ok(())
    }
}
