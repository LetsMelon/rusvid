use crate::{path_like::PathLike, ColorLike};

#[derive(Debug)]
pub struct Svg {
    pub path: Vec<PathLike>,
    pub fill_color: ColorLike,
}
