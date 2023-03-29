use serde::{Deserialize, Serialize};

use crate::holder::image_holder::ImageHolder;
use crate::holder::svg_holder::SvgHolder;
use crate::holder::transform::{Transform, TransformError, TransformLogic};

#[derive(Debug, Serialize, Deserialize)]
pub enum TypesLike {
    Svg(SvgHolder),
    Image(ImageHolder),
}

impl TypesLike {
    pub const VARIANT_COUNT: usize = std::mem::variant_count::<TypesLike>();
}

impl Default for TypesLike {
    fn default() -> Self {
        Self::Svg(Default::default())
    }
}

impl TransformLogic for TypesLike {
    fn transform(&mut self, transformation: &Transform) -> Result<(), TransformError> {
        match self {
            TypesLike::Svg(svg) => svg.transform(transformation),
            TypesLike::Image(_) => todo!(),
        }
    }

    fn transform_by_id(
        &mut self,
        id: impl Into<String>,
        transformation: &Transform,
    ) -> Result<(), TransformError> {
        match self {
            TypesLike::Svg(svg) => svg.transform_by_id(id, transformation),
            TypesLike::Image(_) => todo!(),
        }
    }
}
