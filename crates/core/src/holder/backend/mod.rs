use crate::holder::object::Object;
use crate::plane::{Plane, PlaneResult, SIZE};

#[cfg(feature = "resvg")]
pub mod resvg;

#[cfg(feature = "cairo")]
pub mod cairo;

#[cfg(all(feature = "resvg", feature = "cairo"))]
pub type FeatureBackend = resvg::ResvgBackend;

#[cfg(all(feature = "cairo", not(feature = "resvg")))]
pub type FeatureBackend = cairo::CairoBackend;

pub trait Backend {
    fn name() -> &'static str;

    fn render(object: &Object, width: SIZE, height: SIZE) -> PlaneResult<Plane>;
}
