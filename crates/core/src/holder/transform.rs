use resvg::usvg::Stroke;

use crate::holder::likes::color_like::ColorLike;
use crate::point::Point;

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
    Color(ColorLike),

    /// Change the stroke of the object to the `Option<Stroke>`
    Stroke(Option<Stroke>),
    // TODO's
    // Rotate(f32), // Rotate by angle
    // Scale(Point), // Scale x by Point.x and y by Point.y
}
