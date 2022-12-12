use anyhow::Result;

use crate::holder::image_holder::ImageHolder;
use crate::holder::object::TransformLogic;
use crate::holder::svg_holder::SvgHolder;
use crate::holder::transform::Transform;

#[derive(Debug)]
pub enum TypesLike {
    Svg(SvgHolder),
    Image(ImageHolder),
}

impl TransformLogic for TypesLike {
    fn transform(&mut self, transformation: Transform) -> Result<()> {
        match self {
            TypesLike::Svg(svg) => svg.transform(transformation),
            TypesLike::Image(_) => todo!(),
        }
    }

    fn transform_by_id(&mut self, id: impl Into<String>, transformation: Transform) -> Result<()> {
        match self {
            TypesLike::Svg(svg) => svg.transform_by_id(id, transformation),
            TypesLike::Image(_) => todo!(),
        }
    }
}