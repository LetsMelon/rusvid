use crate::holder::image_holder::ImageHolder;
use crate::holder::svg_holder::SvgHolder;

#[derive(Debug)]
pub enum TypesLike {
    Svg(SvgHolder),
    Image(ImageHolder),
}
