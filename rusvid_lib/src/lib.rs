#![feature(get_mut_unchecked)]

pub mod animation;
pub mod composition;
pub mod figures;
pub mod layer;
pub mod metrics;
pub mod renderer;
pub mod resolution;
pub mod types;
pub mod utils;

/// Repackage the usvg library so the end-user don't have to install `rusvid-lib` and `usvg`
/// and so that the user always uses the same `usvg` like the library
pub use resvg;
pub use {rusvid_core as core, rusvid_effect as effect};

pub mod prelude {
    pub use crate::animation::curves::Function;
    pub use crate::composition::{Composition, CompositionBuilder};
    pub use crate::core::*;
    pub use crate::effect::library::*;
    pub use crate::effect::{EffectLogic, Element};
    pub use crate::layer::{Layer, LayerLogic};
    pub use crate::metrics::{MetricsSize, MetricsVideo};
    pub use crate::renderer::ffmpeg::FfmpegRenderer;
    pub use crate::renderer::Renderer;
    pub use crate::resolution::Resolution;
    pub use crate::types::*;
}
