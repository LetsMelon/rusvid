use crate::holder::gradient::base::BaseGradient;
#[cfg(feature = "cairo")]
use crate::holder::utils::TranslateIntoCairoGeneric;
#[cfg(feature = "resvg")]
use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::point::Point;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct LinearGradient {
    point_1: Point,
    point_2: Point,

    base: BaseGradient,
}

impl LinearGradient {
    pub fn new(base: BaseGradient) -> LinearGradient {
        LinearGradient {
            point_1: Point::ZERO,
            point_2: Point::new(1.0, 0.0),
            base,
        }
    }
}

#[cfg(feature = "resvg")]
impl TranslateIntoResvgGeneric<resvg::usvg::LinearGradient> for LinearGradient {
    fn translate(&self) -> resvg::usvg::LinearGradient {
        resvg::usvg::LinearGradient {
            id: "".to_string(),
            x1: self.point_1.x(),
            y1: self.point_1.y(),
            x2: self.point_2.x(),
            y2: self.point_2.y(),
            base: self.base.translate(),
        }
    }
}

#[cfg(feature = "resvg")]
impl TranslateIntoResvgGeneric<resvg::usvg::Paint> for LinearGradient {
    fn translate(&self) -> resvg::usvg::Paint {
        resvg::usvg::Paint::LinearGradient(std::rc::Rc::new(self.translate()))
    }
}

#[cfg(feature = "cairo")]
impl TranslateIntoCairoGeneric<cairo::LinearGradient> for LinearGradient {
    fn translate_cairo(&self) -> cairo::LinearGradient {
        // TODO use real values
        let gradient = cairo::LinearGradient::new(0.0, 0.0, 250.0, 250.0);

        for stop in self.base.stops() {
            gradient.add_color_stop_rgba(
                stop.offset,
                stop.color.get_r_float(),
                stop.color.get_g_float(),
                stop.color.get_b_float(),
                stop.color.get_a_float(),
            );
        }

        gradient
    }
}
