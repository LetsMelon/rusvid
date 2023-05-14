#![feature(variant_count)]
// TODO remove feature
#![feature(is_some_and)]
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
