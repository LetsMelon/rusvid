use crate::holder::likes::color_like::ColorLike;
use crate::holder::likes::path_like::PathLike;

#[derive(Debug)]
pub struct SvgHolder {
    pub(crate) path: Vec<PathLike>,
    pub(crate) fill_color: ColorLike,
}

impl SvgHolder {
    #[inline]
    pub fn new(path: Vec<PathLike>, fill_color: ColorLike) -> Self {
        Self { path, fill_color }
    }
}
