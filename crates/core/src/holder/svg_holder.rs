use std::collections::HashMap;

use crate::holder::likes::color_like::ColorLike;
use crate::holder::likes::path_like::PathLike;
use crate::holder::stroke::Stroke;
use crate::holder::transform::{Transform, TransformError, TransformLogic};
use crate::holder::utils;
use crate::point::Point;

const BOUNDING_BOX_STEPS: i32 = 10;

#[derive(Debug)]
pub struct SvgItem {
    pub(crate) id: String,
    pub(crate) path: Vec<PathLike>,

    pub(crate) fill_color: Option<ColorLike>,
    pub(crate) stroke: Option<Stroke>,

    pub(crate) visibility: bool,
}

impl SvgItem {
    #[inline]
    pub fn new_with_id(
        id: impl Into<String>,
        path: Vec<PathLike>,
        fill_color: Option<ColorLike>,
    ) -> Self {
        Self {
            id: id.into(),
            path,
            fill_color,
            stroke: Some(Stroke::default()),
            visibility: true,
        }
    }

    #[inline]
    pub fn new(path: Vec<PathLike>, fill_color: Option<ColorLike>) -> Self {
        Self::new_with_id(utils::random_id(), path, fill_color)
    }

    pub fn bounding_box(&self) -> (Point, Point) {
        let mut smaller_corner = match self.path[0] {
            PathLike::Move(p) => p,
            _ => todo!(),
        };
        let mut bigger_corner = smaller_corner;

        let mut last_point = smaller_corner;
        self.path.iter().for_each(|path| {
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

                    #[inline]
                    fn point_to_coord2(p: &Point) -> Coord2 {
                        Coord2(p.x(), p.y())
                    }

                    #[inline]
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

    pub fn bounding_box_rect(&self) -> SvgItem {
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

        let mut my_box = SvgItem::new(vec![a, b, c, d, PathLike::Close], None);
        my_box.stroke = Some(Stroke {
            paint: ColorLike::Color([0, 0, 0, 255]),
            width: 3.0,
            ..Default::default()
        });

        my_box
    }
}

impl TransformLogic for SvgItem {
    fn transform(&mut self, transformation: &Transform) -> Result<(), TransformError> {
        match &transformation {
            Transform::Visibility(value) => self.visibility = *value,
            Transform::Move(point) => {
                self.path = self
                    .path
                    .iter()
                    .map(|p| match *p {
                        PathLike::Move(og_p) => PathLike::Move(og_p + *point),
                        PathLike::Line(og_p) => PathLike::Line(og_p + *point),
                        PathLike::CurveTo(end, c1, c2) => {
                            PathLike::CurveTo(end + *point, c1 + *point, c2 + *point)
                        }
                        PathLike::Close => PathLike::Close,
                    })
                    .collect();
            }
            Transform::Position(position) => match self.path[0] {
                PathLike::Move(point) => {
                    let offset = *position - point;
                    self.transform(&Transform::Move(offset))?
                }
                _ => panic!("First element needs to be a `PathLike::Move`"),
            },
            Transform::Color(value) => {
                self.fill_color = *value;
            }
            Transform::Stroke(stroke) => {
                self.stroke = stroke.clone();
            }
            Transform::Scale(factor) => {
                let bounding_box = self.bounding_box();

                let size = bounding_box.1 - bounding_box.0;
                let center = bounding_box.0 + size / 2.0;

                self.path = self
                    .path
                    .iter()
                    .map(|p| {
                        let formatted = match *p {
                            PathLike::Move(value) => {
                                let v = (value - center) * *factor;
                                let pos = center + v;
                                PathLike::Move(pos)
                            }
                            PathLike::Line(value) => {
                                let v = (value - center) * *factor;
                                let pos = center + v;
                                PathLike::Line(pos)
                            }
                            PathLike::CurveTo(_, _, _) => todo!(),
                            PathLike::Close => PathLike::Close,
                        };

                        formatted
                    })
                    .collect();
            }
        };

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct SvgHolder {
    pub(crate) items: HashMap<String, SvgItem>,
}

impl SvgHolder {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(&mut self, item: SvgItem) -> String {
        let id = item.id.clone();
        self.items.insert(id.clone(), item);
        id
    }

    pub fn get_item_mut(&mut self, key: impl Into<String>) -> Option<&mut SvgItem> {
        self.items.get_mut(&key.into())
    }
}

impl TransformLogic for SvgHolder {
    fn transform(&mut self, transformation: &Transform) -> Result<(), TransformError> {
        for item in &mut self.items.values_mut() {
            item.transform(&transformation)?;
        }

        Ok(())
    }

    fn transform_by_id(
        &mut self,
        id: impl Into<String>,
        transformation: &Transform,
    ) -> Result<(), TransformError> {
        let id: String = id.into();
        let item = self
            .get_item_mut(id.clone())
            .ok_or(TransformError::NoItem(id))?;

        item.transform(transformation)
    }
}
