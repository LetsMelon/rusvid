#![feature(iter_array_chunks)]
#![feature(variant_count)]
#![feature(is_some_and)]
#![cfg_attr(coverage_nightly, feature(no_coverage))]

#[macro_use]
extern crate static_assertions;

pub mod frame_image_format;
pub mod histogram;
pub mod holder;
pub mod pixel;
pub mod plane;
pub mod point;
