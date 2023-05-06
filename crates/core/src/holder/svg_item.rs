use crate::holder::likes::color_like::ColorLike;
use crate::holder::likes::path_like::PathLike;
use crate::holder::polygon::Polygon;
use crate::holder::stroke::Stroke;
use crate::holder::transform::{Transform, TransformError, TransformLogic};
use crate::holder::utils::random_id;
#[cfg(feature = "cairo")]
use crate::holder::utils::TranslateIntoCairoGeneric;
#[cfg(feature = "resvg")]
use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::pixel::Pixel;
use crate::point::Point;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SvgItem {
    pub(crate) id: String,
    pub(crate) polygon: Polygon,

    pub(crate) fill_color: Option<ColorLike>,
    pub(crate) stroke: Option<Stroke>,

    pub(crate) visibility: bool,

    last_rotation: f64,
    last_scale: Point,
}

impl SvgItem {
    pub fn new_with_id(
        id: impl Into<String>,
        polygon: Polygon,
        fill_color: Option<ColorLike>,
    ) -> Self {
        Self {
            id: id.into(),
            polygon,
            fill_color,
            stroke: Some(Stroke::default()),
            visibility: true,
            last_rotation: 0.0,
            last_scale: Point::new_symmetric(1.0),
        }
    }

    pub fn new(polygon: Polygon, fill_color: Option<ColorLike>) -> Self {
        Self::new_with_id(random_id(), polygon, fill_color)
    }

    pub fn bounding_box(&self) -> (Point, Point) {
        self.polygon.bounding_box()
    }

    pub fn bounding_box_rect(&self) -> SvgItem {
        let polygon = self.polygon.bounding_box_polygon();

        let mut my_box = SvgItem::new(polygon, None);
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
            Transform::Move(point) => self.polygon.transform(&Transform::Move(*point))?,
            Transform::Position(position) => {
                self.polygon.transform(&Transform::Position(*position))?
            }
            Transform::Color(value) => {
                self.fill_color = value.clone();
            }
            Transform::Stroke(stroke) => {
                self.stroke = stroke.clone();
            }
            Transform::Scale(factor) => {
                let factor_adjusted = *factor / self.last_scale;

                self.polygon.transform(&Transform::Scale(factor_adjusted))?;

                self.last_scale = *factor;
            }
            Transform::Rotate((angle, rotation_point)) => {
                let angle_diff = self.last_rotation - *angle;

                self.polygon
                    .transform(&Transform::Rotate((angle_diff, rotation_point.clone())))?;

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
        PathLike::extend_path_from_slice(&mut path, &self.polygon.path());

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
            data: std::rc::Rc::new(path),
            ..resvg::usvg::Path::default()
        })
    }
}

#[cfg(feature = "cairo")]
impl crate::holder::utils::ApplyToCairoContext for SvgItem {
    fn apply(&self, context: &cairo::Context) -> Result<(), Box<dyn std::error::Error>> {
        // use cairo::{LinearGradient, Pattern};

        // visibility = true
        if self.visibility {
            // apply path
            if self.polygon.path()[0].type_equal(&PathLike::Move(Point::ZERO)) {
                context.new_path();
            }
            for path in self.polygon.iter() {
                path.apply(context)?;
            }
            if self.polygon.path()[self.polygon.path().len() - 1].type_equal(&PathLike::Close) {
                context.close_path();
            }

            let _obj_size = self.bounding_box();

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
                    ColorLike::LinearGradient(lg) => {
                        let gradient = lg.translate_cairo();
                        context.set_source(gradient)?;
                        context.fill()?;
                    }
                    ColorLike::RadialGradient(_) => todo!(),
                }
            }
        }

        Ok(())
    }
}
