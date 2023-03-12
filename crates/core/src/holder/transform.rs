use std::fmt::Debug;

use thiserror::Error;

use crate::holder::likes::color_like::ColorLike;
use crate::holder::stroke::Stroke;
use crate::point::Point;

#[derive(Error, Debug)]
pub enum TransformError {
    #[error("`{0}` is not implemented for {1}")]
    NotImplemented(&'static str, String),

    #[error("No item with id `{0}`")]
    NoItem(String),
}

pub trait TransformLogic: Debug {
    fn transform(&mut self, transformation: &Transform) -> Result<(), TransformError>;

    #[allow(unused_variables)]
    fn transform_by_id(
        &mut self,
        id: impl Into<String>,
        transformation: &Transform,
    ) -> Result<(), TransformError> {
        Err(TransformError::NotImplemented(
            "transform_by_id",
            format!("{self:?}"),
        ))
    }
}

#[derive(Debug, Clone)]
/// Visual guide: [Link](https://css-tricks.com/transforms-on-svg-elements/)
pub enum Transform {
    /// Change visibility of the object; `true` = visible, `false` = hidden
    Visibility(bool),

    /// Move the path in space along the `Point`
    Move(Point),

    /// Set the origin point to the `Point`
    Position(Point),

    /// Set the color of the object to `ColorLike`
    Color(Option<ColorLike>),

    /// Change the stroke of the object to the `Option<Stroke>`
    Stroke(Option<Stroke>),

    /// Scale x and y by value
    Scale(Point),

    /// Rotate by angle in radiant
    Rotate(f64),
    // TODO's
    // Opacity(f32), // Should change the alpha chanel in the color
}
