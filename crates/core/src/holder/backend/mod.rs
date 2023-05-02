use crate::holder::object::Object;
use crate::plane::{Plane, PlaneResult, SIZE};

#[cfg(feature = "resvg")]
pub mod resvg;

#[cfg(feature = "cairo")]
pub mod cairo;

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum BackendType {
    #[cfg(feature = "resvg")]
    Resvg,
    #[cfg(feature = "cairo")]
    Cairo,
}

impl Default for BackendType {
    fn default() -> Self {
        #[cfg(not(any(feature = "resvg", feature = "cairo")))]
        compile_error!(
            "Either feature \"resvg\" and/or \"cairo\" must be enabled for rusvid_core."
        );

        // TODO is there something like `compile_warning!`
        // #[cfg(all(feature = "resvg", feature = "cairo"))]
        // compile_error!("Waring: Features \"resvg\" and \"cairo\" are enabled at the same time, the default backend is \"resvg\"");

        // TODO maybe use `cfg_if!`

        #[cfg(feature = "resvg")]
        return BackendType::Resvg;

        #[cfg(feature = "cairo")]
        return BackendType::Cairo;
    }
}

impl BackendType {
    #[inline]
    pub(crate) fn get_type(&self) -> Box<dyn Backend> {
        match self {
            #[cfg(feature = "resvg")]
            BackendType::Resvg => Box::new(resvg::ResvgBackend {}),
            #[cfg(feature = "cairo")]
            BackendType::Cairo => Box::new(cairo::CairoBackend {}),
        }
    }
}

pub trait Backend {
    fn name(&self) -> &'static str;

    fn render(&self, object: &Object, width: SIZE, height: SIZE) -> PlaneResult<Plane>;
}
