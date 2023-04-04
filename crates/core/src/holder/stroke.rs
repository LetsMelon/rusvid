use resvg::usvg::{NonZeroPositiveF64, NormalizedF64};

use crate::holder::likes::color_like::ColorLike;
use crate::holder::utils::TranslateIntoResvgGeneric;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Stroke {
    pub paint: ColorLike,
    pub dasharray: Option<Vec<f64>>,
    pub dashoffset: f64,
    // pub miterlimit: StrokeMiterlimit,
    pub opacity: f64,
    pub width: f64,
    // pub linecap: LineCap,
    // pub linejoin: LineJoin,
}

impl Default for Stroke {
    fn default() -> Self {
        let default = resvg::usvg::Stroke::default();
        Self::from_resvg_stroke(default)
    }
}

impl Stroke {
    pub fn from_resvg_stroke(stroke: resvg::usvg::Stroke) -> Stroke {
        // should fail when something changes in `resvg::usvg::Stroke` struct
        debug_assert_eq!(std::mem::size_of::<resvg::usvg::Stroke>(), 72);

        let paint = ColorLike::from_resvg_paint(&stroke.paint);

        Stroke {
            paint,
            dasharray: stroke.dasharray,
            dashoffset: stroke.dashoffset as f64,
            opacity: stroke.opacity.get(),
            width: stroke.width.get(),
        }
    }
}

impl TranslateIntoResvgGeneric<resvg::usvg::Stroke> for Stroke {
    fn translate(&self) -> resvg::usvg::Stroke {
        resvg::usvg::Stroke {
            paint: self.paint.translate(),
            dasharray: self.dasharray.clone(),
            dashoffset: self.dashoffset as f32,
            opacity: NormalizedF64::new(self.opacity.abs()).unwrap(),
            width: NonZeroPositiveF64::new(self.width.abs()).unwrap(),
            ..resvg::usvg::Stroke::default()
        }
    }
}
