use crate::holder::image_holder::ImageHolder;

#[derive(Debug)]
pub enum TypesLike {
    Svg(crate::holder::svg::Svg),
    Image(ImageHolder),
}
