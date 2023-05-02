use std::collections::HashMap;

use super::utils::ApplyToCairoContext;
use crate::holder::backend::{Backend, FeatureBackend};
use crate::holder::likes::types_like::TypesLike;
use crate::holder::transform::{Transform, TransformError, TransformLogic};
use crate::holder::utils;
use crate::plane::{Plane, PlaneError, SIZE};

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Object {
    data: TypesLike,
    id: String,
}

impl Object {
    pub fn new_with_id(id: impl Into<String>, data: TypesLike) -> Self {
        Object {
            data,
            id: id.into(),
        }
    }

    pub fn new(data: TypesLike) -> Self {
        Self::new_with_id(utils::random_id(), data)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn render(&self, width: SIZE, height: SIZE) -> Result<Plane, PlaneError> {
        FeatureBackend::render(self, width, height)
    }

    pub fn transforms(&mut self, transformations: Vec<Transform>) -> Result<(), TransformError> {
        for transformation in transformations.iter() {
            self.transform(transformation)?;
        }

        Ok(())
    }

    pub fn transform_key_value(
        &mut self,
        transformations: HashMap<&str, &Transform>,
    ) -> Result<(), TransformError> {
        for (id, transformation) in transformations {
            self.transform_by_id(id, transformation)?;
        }

        Ok(())
    }

    pub fn data(&self) -> &TypesLike {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut TypesLike {
        &mut self.data
    }
}

impl TransformLogic for Object {
    fn transform(&mut self, transformation: &Transform) -> Result<(), TransformError> {
        self.data.transform(transformation)
    }

    fn transform_by_id(
        &mut self,
        id: impl Into<String>,
        transformation: &Transform,
    ) -> Result<(), TransformError> {
        self.data.transform_by_id(id, transformation)
    }
}

impl ApplyToCairoContext for Object {
    fn apply(&self, context: &cairo::Context) -> Result<(), Box<dyn std::error::Error>> {
        match &self.data {
            TypesLike::Svg(svg_holder) => svg_holder.apply(context),
            TypesLike::Image(_) => todo!(),
        }
    }
}
