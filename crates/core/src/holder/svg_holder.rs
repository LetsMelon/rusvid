use std::collections::HashMap;

use itertools::Itertools;

use crate::holder::svg_item::SvgItem;
use crate::holder::transform::{Transform, TransformError, TransformLogic};

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SvgHolder {
    pub(crate) items: HashMap<String, HashMapSvgItem>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub(crate) struct HashMapSvgItem {
    pub data: SvgItem,
    pub z_index: usize,
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
        let new_depth = self.max_depth().map(|value| value + 1).unwrap_or(0);

        self.add_item_with_depth(item, new_depth)
    }

    pub fn max_depth(&self) -> Option<usize> {
        if self.items.len() == 0 {
            return None;
        }

        self.items.iter().map(|(_, item)| item.z_index).max()
    }

    pub fn min_depth(&self) -> Option<usize> {
        if self.items.len() == 0 {
            return None;
        }

        self.items.iter().map(|(_, item)| item.z_index).min()
    }

    pub fn add_item_with_depth(&mut self, item: SvgItem, depth: usize) -> String {
        let id = item.id.clone();

        self.items.insert(
            id.clone(),
            HashMapSvgItem {
                data: item,
                z_index: depth,
            },
        );
        id
    }

    #[inline]
    fn get_item_internal(&self, key: impl Into<String>) -> Option<&HashMapSvgItem> {
        self.items.get(&key.into())
    }

    pub fn get_item(&self, key: impl Into<String>) -> Option<&SvgItem> {
        self.get_item_internal(key).map(|item| &item.data)
    }

    #[inline]
    fn get_item_mut_internal(&mut self, key: impl Into<String>) -> Option<&mut HashMapSvgItem> {
        self.items.get_mut(&key.into())
    }

    pub fn get_item_mut(&mut self, key: impl Into<String>) -> Option<&mut SvgItem> {
        self.get_item_mut_internal(key).map(|item| &mut item.data)
    }

    pub fn get_item_depth(&self, key: impl Into<String>) -> Option<usize> {
        self.get_item_internal(key).map(|item| item.z_index)
    }

    /// Changes the depth value of the [`SvgItem`] to the given depth.
    ///
    /// Returns the old depth-value if it has been changed otherwise returns `None`.
    pub fn set_item_depth(&mut self, key: impl Into<String>, depth: usize) -> Option<usize> {
        let item = self.get_item_mut_internal(key)?;

        let old_value = item.z_index;
        item.z_index = depth;

        Some(old_value)
    }
}

impl TransformLogic for SvgHolder {
    fn transform(&mut self, transformation: &Transform) -> Result<(), TransformError> {
        for item in &mut self.items.values_mut() {
            item.data.transform(transformation)?;
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
        for item in self
            .items
            .iter()
            .sorted_by_key(|(_, item)| -(item.z_index as isize))
            .map(|(_, item)| &item.data)
        {
            item.apply(context)?;
        }

        Ok(())
    }
}
