use rusvid_core::point::Point;

#[derive(Debug)]
pub enum Transform {
    Visibility(bool),
    Move(Point),
}
