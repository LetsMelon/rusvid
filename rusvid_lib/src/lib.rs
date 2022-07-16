#![feature(get_mut_unchecked)]

pub mod composition;
pub mod figures;
pub mod metrics;
pub mod renderer;
pub mod resolution;
pub mod types;
pub mod utils;

/// Repackage the usvg library so the end-user don't have to install `rusvid-lib` and `usvg`
/// and so that the user always uses the same `usvg` like the library
pub mod usvg {
    pub use usvg::*;
}
