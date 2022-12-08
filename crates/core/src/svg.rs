use crate::likes::color_like::ColorLike;
use crate::likes::path_like::PathLike;

#[derive(Debug)]
pub struct Svg {
    pub(crate) path: Vec<PathLike>,
    pub(crate) fill_color: ColorLike,
}

impl Svg {
    pub fn new(path: Vec<PathLike>, fill_color: ColorLike) -> Self {
        Svg { path, fill_color }
    }
}
