use rusvid_core::point::Point;

use crate::ColorLike;

#[derive(Debug)]
pub enum Transform {
    Visibility(bool),
    Move(Point),
    Position(Point),
    Color(ColorLike),
}
