use std::collections::HashMap;

use crate::holder::svg_item::SvgItem;
use crate::holder::transform::{Transform, TransformError, TransformLogic};

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SvgHolder {
    pub(crate) items: HashMap<String, SvgItem>,
}

impl SvgHolder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_with_items(items: Vec<SvgItem>) -> Self {
        let mut svg_holder = Self::new();

        for item in items {
            svg_holder.add_item(item);
        }

        svg_holder
    }

    pub fn add_item(&mut self, item: SvgItem) -> String {
        let id = item.id.clone();
        self.items.insert(id.clone(), item);
        id
    }

    pub fn get_item(&self, key: impl Into<String>) -> Option<&SvgItem> {
        self.items.get(&key.into())
    }

    pub fn get_item_mut(&mut self, key: impl Into<String>) -> Option<&mut SvgItem> {
        self.items.get_mut(&key.into())
    }
}

impl TransformLogic for SvgHolder {
    fn transform(&mut self, transformation: &Transform) -> Result<(), TransformError> {
        for item in &mut self.items.values_mut() {
            item.transform(transformation)?;
        }

        Ok(())
    }

    fn transform_by_id(
        &mut self,
        id: impl Into<String>,
        transformation: &Transform,
    ) -> Result<(), TransformError> {
        let id: String = id.into();
        let item = self
            .get_item_mut(id.clone())
            .ok_or(TransformError::NoItem(id))?;

        item.transform(transformation)
    }
}

#[cfg(feature = "cairo")]
impl crate::holder::utils::ApplyToCairoContext for SvgHolder {
    fn apply(&self, context: &cairo::Context) -> Result<(), Box<dyn std::error::Error>> {
        for (_, item) in &self.items {
            item.apply(context)?;
        }

        Ok(())
    }
}
