use resvg::usvg::{NonZeroPositiveF64, NormalizedF64};

use super::likes::color_like::ColorLike;

#[derive(Debug, Clone)]
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

    pub fn as_resvg_stroke(&self) -> Option<resvg::usvg::Stroke> {
        let mut default = resvg::usvg::Stroke::default();

        default.paint = self.paint.as_resvg_paint();
        default.dasharray = self.dasharray.clone();
        default.dashoffset = self.dashoffset as f32;
        default.opacity = NormalizedF64::new(self.opacity)?;
        default.width = NonZeroPositiveF64::new(self.width)?;

        Some(default)
    }
}
