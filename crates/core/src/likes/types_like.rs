use crate::image_holder::ImageHolder;

#[derive(Debug)]
pub enum TypesLike {
    Svg(crate::svg::Svg),
    Image(ImageHolder),
}
