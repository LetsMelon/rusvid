use rusvid_core::plane::Pixel;

#[derive(Debug, Clone, Copy)]
pub enum ColorLike {
    Color(Pixel),
}
