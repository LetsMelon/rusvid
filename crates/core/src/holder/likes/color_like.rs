use crate::plane::Pixel;

#[derive(Debug, Clone, Copy)]
pub enum ColorLike {
    Color(Pixel),
}
