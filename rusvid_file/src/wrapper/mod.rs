mod composition;
mod layer;
mod object;
mod resolution;

pub use composition::*;
pub use layer::*;
pub use object::*;
pub use resolution::*;

pub trait TranslateIntoRusvidGeneric:
    std::fmt::Debug + serde::Serialize + for<'a> serde::Deserialize<'a>
{
    type OUTPUT;

    fn translate(&self) -> Self::OUTPUT;
}
