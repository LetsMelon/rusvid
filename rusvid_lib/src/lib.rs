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
pub use usvg;

pub mod prelude {
    pub use crate::composition::Composition;
    pub use crate::renderer::ffmpeg::FfmpegRenderer;
    pub use crate::renderer::png::PngRender;
    pub use crate::renderer::raw::RawRender;
    pub use crate::renderer::Renderer;
    pub use crate::resolution::Resolution;

    pub mod figures {
        pub use crate::figures::circle::circle;
        pub use crate::figures::rect::rect;
        pub use crate::figures::triangle::equilateral_triangle;
    }
}
