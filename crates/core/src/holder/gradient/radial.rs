use std::rc::Rc;

use crate::holder::gradient::base::BaseGradient;
use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::point::Point;

#[derive(Debug, Clone)]
pub struct RadialGradient {
    end_circle: Point,
    start_circle_radius: f64,
    start_circle: Point,

    base: BaseGradient,
}

impl RadialGradient {
    pub fn new(
        start_circle: Point,
        start_circle_radius: f64,
        end_circle: Point,
        base: BaseGradient,
    ) -> Self {
        RadialGradient {
            end_circle,
            start_circle_radius,
            start_circle,
            base,
        }
    }
}

impl TranslateIntoResvgGeneric<resvg::usvg::RadialGradient> for RadialGradient {
    fn translate(&self) -> resvg::usvg::RadialGradient {
        resvg::usvg::RadialGradient {
            id: "".to_string(),
            cx: self.end_circle.x(),
            cy: self.end_circle.y(),
            r: resvg::usvg::PositiveF64::new(self.start_circle_radius.abs()).unwrap(),
            fx: self.start_circle.x(),
            fy: self.start_circle.y(),
            base: self.base.translate(),
        }
    }
}

impl TranslateIntoResvgGeneric<resvg::usvg::Paint> for RadialGradient {
    fn translate(&self) -> resvg::usvg::Paint {
        resvg::usvg::Paint::RadialGradient(Rc::new(self.translate()))
    }
}
