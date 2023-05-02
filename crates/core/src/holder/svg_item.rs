use std::rc::Rc;

use super::utils::ApplyToCairoContext;
use crate::holder::likes::color_like::ColorLike;
use crate::holder::likes::path_like::PathLike;
use crate::holder::stroke::Stroke;
use crate::holder::transform::{Transform, TransformError, TransformLogic};
use crate::holder::utils::random_id;
#[cfg(feature = "resvg")]
use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::pixel::Pixel;
use crate::point::Point;

const BOUNDING_BOX_STEPS: i32 = 10;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SvgItem {
    pub(crate) id: String,
    pub(crate) path: Vec<PathLike>,

    pub(crate) fill_color: Option<ColorLike>,
    pub(crate) stroke: Option<Stroke>,

    pub(crate) visibility: bool,

    last_rotation: f64,
    last_scale: Point,
}

impl SvgItem {
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
            last_rotation: 0.0,
            last_scale: Point::new_symmetric(1.0),
        }
    }

    pub fn new(path: Vec<PathLike>, fill_color: Option<ColorLike>) -> Self {
        Self::new_with_id(random_id(), path, fill_color)
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
            paint: ColorLike::Color(Pixel::new(0, 0, 0, 255)),
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
                self.fill_color = value.clone();
            }
            Transform::Stroke(stroke) => {
                self.stroke = stroke.clone();
            }
            Transform::Scale(factor) => {
                let center = PathLike::get_center(&self.path);

                let factor_adjusted = *factor / self.last_scale;

                self.path = self
                    .path
                    .iter()
                    .map(|p| {
                        let formatted = match *p {
                            PathLike::Move(value) => {
                                let v = (value - center) * factor_adjusted;
                                let pos = center + v;
                                PathLike::Move(pos)
                            }
                            PathLike::Line(value) => {
                                let v = (value - center) * factor_adjusted;
                                let pos = center + v;
                                PathLike::Line(pos)
                            }
                            PathLike::CurveTo(end, c_s, c_e) => {
                                let p_e = center + (end - center) * factor_adjusted;
                                let p_c_s = center + (c_s - center) * factor_adjusted;
                                let p_c_e = center + (c_e - center) * factor_adjusted;

                                PathLike::CurveTo(p_e, p_c_s, p_c_e)
                            }
                            PathLike::Close => PathLike::Close,
                        };

                        formatted
                    })
                    .collect();

                self.last_scale = *factor;
            }
            Transform::Rotate(angle) => {
                let center = PathLike::get_center(&self.path);

                let angle_diff = self.last_rotation - *angle;

                self.path = self
                    .path
                    .iter()
                    .map(|p| {
                        // TODO make helper functions to remove duplicate code
                        let formatted = match *p {
                            PathLike::Move(value) => {
                                let v = value - center;

                                let x = angle_diff.cos() * v.x() - angle_diff.sin() * v.y();
                                let y = angle_diff.sin() * v.x() + angle_diff.cos() * v.y();

                                let pos = center + Point::new(x, y);
                                PathLike::Move(pos)
                            }
                            PathLike::Line(value) => {
                                let v = value - center;

                                let x = angle_diff.cos() * v.x() - angle_diff.sin() * v.y();
                                let y = angle_diff.sin() * v.x() + angle_diff.cos() * v.y();

                                let pos = center + Point::new(x, y);
                                PathLike::Line(pos)
                            }
                            PathLike::CurveTo(end, c_s, c_e) => {
                                let v_e = end - center;
                                let v_c_s = c_s - center;
                                let v_c_e = c_e - center;

                                let e = center
                                    + Point::new(
                                        angle_diff.cos() * v_e.x() - angle_diff.sin() * v_e.y(),
                                        angle_diff.sin() * v_e.x() + angle_diff.cos() * v_e.y(),
                                    );
                                let cs = center
                                    + Point::new(
                                        angle_diff.cos() * v_c_s.x() - angle_diff.sin() * v_c_s.y(),
                                        angle_diff.sin() * v_c_s.x() + angle_diff.cos() * v_c_s.y(),
                                    );
                                let ce = center
                                    + Point::new(
                                        angle_diff.cos() * v_c_e.x() - angle_diff.sin() * v_c_e.y(),
                                        angle_diff.sin() * v_c_e.x() + angle_diff.cos() * v_c_e.y(),
                                    );

                                PathLike::CurveTo(e, cs, ce)
                            }
                            PathLike::Close => PathLike::Close,
                        };

                        formatted
                    })
                    .collect();

                self.last_rotation = *angle;
            }
        };

        Ok(())
    }
}

#[cfg(feature = "resvg")]
impl TranslateIntoResvgGeneric<resvg::usvg::NodeKind> for SvgItem {
    fn translate(&self) -> resvg::usvg::NodeKind {
        use resvg::usvg::*;

        let mut path = PathData::new();
        PathLike::extend_path_from_slice(&mut path, &self.path);

        let fill = self
            .fill_color
            .as_ref()
            .map(|color_like| color_like.translate());

        let visibility = match self.visibility {
            true => Visibility::Visible,
            false => Visibility::Hidden,
        };

        resvg::usvg::NodeKind::Path(resvg::usvg::Path {
            id: self.id.clone(),
            visibility,
            fill,
            stroke: self.stroke.clone().map(|s| s.translate()),
            data: Rc::new(path),
            ..resvg::usvg::Path::default()
        })
    }
}

impl ApplyToCairoContext for SvgItem {
    fn apply(&self, context: &cairo::Context) -> Result<(), Box<dyn std::error::Error>> {
        // visibility = false
        if !self.visibility {
            // apply path
            if self.path[0].type_equal(&PathLike::Move(Point::ZERO)) {
                context.new_path();
            }
            for path in &self.path {
                path.apply(context)?;
            }
            if self.path[self.path.len() - 1].type_equal(&PathLike::Close) {
                context.close_path();
            }

            // apply stroke
            if let Some(stroke) = &self.stroke {
                match &stroke.paint {
                    ColorLike::Color(c) => context.set_source_rgba(
                        c.get_r_float(),
                        c.get_g_float(),
                        c.get_b_float(),
                        c.get_a_float() * stroke.opacity,
                    ),
                    ColorLike::LinearGradient(_) => todo!(),
                    ColorLike::RadialGradient(_) => todo!(),
                }

                if let Some(dasharray) = &stroke.dasharray {
                    context.set_dash(dasharray.as_slice(), stroke.dashoffset);
                }

                context.set_line_width(stroke.width);

                context.stroke_preserve()?;
            }

            // apply fill_color
            if let Some(color_like) = &self.fill_color {
                match color_like {
                    ColorLike::Color(c) => {
                        context.set_source_rgba(
                            c.get_r_float(),
                            c.get_g_float(),
                            c.get_b_float(),
                            c.get_a_float(),
                        );
                        context.fill()?;
                    }
                    ColorLike::LinearGradient(_) => todo!(),
                    ColorLike::RadialGradient(_) => todo!(),
                }
            }
        }

        Ok(())
    }
}
