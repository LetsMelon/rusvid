#![feature(variant_count)]
#![cfg_attr(coverage_nightly, feature(no_coverage))]

#[macro_use]
extern crate static_assertions;

pub mod frame_image_format;
pub mod holder;
pub mod pixel;
pub mod plane;
pub mod point;
#[cfg(feature = "server")]
pub mod server;

pub mod prelude {
    pub use crate::frame_image_format::FrameImageFormat;
    // TODO
    pub use crate::holder;
    pub use crate::pixel::Pixel;
    pub use crate::plane::*;
    pub use crate::point::*;
}

#[cfg(not(any(feature = "resvg", feature = "cairo")))]
compile_error!("Either feature \"resvg\" and/or \"cairo\" must be enabled for rusvid_core.");

// TODO is there something like `compile_warning!`
// #[cfg(all(feature = "resvg", feature = "cairo"))]
// compile_error!("Waring: Features \"resvg\" and \"cairo\" are enabled at the same time, the default backend is \"resvg\"");
