use rusvid_core::point::Point;

use crate::ColorLike;

#[derive(Debug, Clone, Copy)]
/// Visual guide: [Link](https://css-tricks.com/transforms-on-svg-elements/)
pub enum Transform {
    /// Change visibility of the object; `true` = visible, `false` = hidden
    Visibility(bool),

    /// Move the path in space along the `Point`
    Move(Point),

    /// Set the origin point to the `Point`
    Position(Point),

    /// Set the color of the object to `ColorLike`
    Color(ColorLike),
}