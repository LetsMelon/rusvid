use uuid::Uuid;

pub fn random_id() -> String {
    Uuid::new_v4().to_string()
}

#[cfg(feature = "resvg")]
pub(crate) trait TranslateIntoResvgGeneric<T> {
    // TODO maybe add somehow a check if `T` is a type from crate `resvg`
    fn translate(&self) -> T;
}

#[cfg(feature = "cairo")]
pub(crate) trait ApplyToCairoContext {
    fn apply(&self, context: &cairo::Context) -> Result<(), Box<dyn std::error::Error>>;
}
