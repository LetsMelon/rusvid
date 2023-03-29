use std::rc::Rc;

use serde::{Deserialize, Serialize};

use crate::holder::gradient::base::BaseGradient;
use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::point::Point;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl TranslateIntoResvgGeneric<resvg::usvg::Paint> for LinearGradient {
    fn translate(&self) -> resvg::usvg::Paint {
        resvg::usvg::Paint::LinearGradient(Rc::new(self.translate()))
    }
}
