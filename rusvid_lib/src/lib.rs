#![feature(variant_count)]

pub mod animation;
pub mod composition;
pub mod figures;
pub mod layer;
pub mod metrics;
pub mod renderer;
pub mod resolution;
pub mod types;
pub mod utils;

pub use {rusvid_core as core, rusvid_effect as effect};

pub mod prelude {
    pub use crate::animation::change_color_animation::ChangeColorAnimation;
    pub use crate::animation::position_animation::PositionAnimation;
    pub use crate::animation::set_color_animation::SetColorAnimation;
    pub use crate::animation::{Animation, AnimationType};
    pub use crate::composition::{Composition, CompositionBuilder};
    pub use crate::core::prelude::*;
    pub use crate::effect::library::*;
    pub use crate::effect::{EffectLogic, Element};
    pub use crate::layer::{Layer, LayerType};
    pub use crate::metrics::{MetricsSize, MetricsVideo};
    pub use crate::renderer::embedded::EmbeddedRenderer;
    pub use crate::renderer::ffmpeg::FfmpegRenderer;
    pub use crate::renderer::frame::FrameRenderer;
    #[cfg(feature = "remote_renderer")]
    pub use crate::renderer::remote::RemoteRenderer;
    pub use crate::renderer::Renderer;
    pub use crate::resolution::Resolution;
    pub use crate::types::*;
}
