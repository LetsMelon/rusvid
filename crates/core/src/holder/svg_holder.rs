use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::holder::likes::color_like::ColorLike;
use crate::holder::likes::path_like::PathLike;
use crate::holder::object::TransformLogic;
use crate::holder::transform::Transform;
use crate::holder::utils;

#[derive(Debug)]
pub struct SvgItem {
    pub(crate) id: String,
    pub(crate) path: Vec<PathLike>,
    pub(crate) fill_color: ColorLike,

    pub(crate) visibility: bool,
}

impl SvgItem {
    #[inline]
    pub fn new_with_id(id: impl Into<String>, path: Vec<PathLike>, fill_color: ColorLike) -> Self {
        Self {
            id: id.into(),
            path,
            fill_color,
            visibility: true,
        }
    }

    #[inline]
    pub fn new(path: Vec<PathLike>, fill_color: ColorLike) -> Self {
        Self::new_with_id(utils::random_id(), path, fill_color)
    }
}

impl TransformLogic for SvgItem {
    fn transform(&mut self, transformation: Transform) -> Result<()> {
        match transformation {
            Transform::Visibility(value) => self.visibility = value,
            Transform::Move(point) => {
                self.path = self
                    .path
                    .iter()
                    .map(|p| match *p {
                        PathLike::Move(og_p) => PathLike::Move(og_p + point),
                        PathLike::Line(og_p) => PathLike::Line(og_p + point),
                        PathLike::Close => PathLike::Close,
                    })
                    .collect();
            }
            Transform::Position(position) => match self.path[0] {
                PathLike::Move(point) => {
                    let offset = position - point;
                    self.transform(Transform::Move(offset))?
                }
                _ => panic!("First element needs to be a `PathLike::Move`"),
            },
            Transform::Color(value) => {
                self.fill_color = value;
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub struct SvgHolder {
    pub(crate) items: HashMap<String, SvgItem>,
}

impl SvgHolder {
    #[inline]
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: SvgItem) -> String {
        let id = item.id.clone();
        self.items.insert(id.clone(), item);
        id
    }

    pub fn get_item_mut(&mut self, key: impl Into<String>) -> Option<&mut SvgItem> {
        self.items.get_mut(&key.into())
    }
}

impl TransformLogic for SvgHolder {
    fn transform(&mut self, transformation: Transform) -> Result<()> {
        for item in &mut self.items.values_mut() {
            item.transform(transformation)?;
        }

        Ok(())
    }

    fn transform_by_id(&mut self, id: impl Into<String>, transformation: Transform) -> Result<()> {
        let id: String = id.into();
        let item = self
            .get_item_mut(id.clone())
            .context(format!("SvgHolder don't have an item with the id `{}`", id))?;

        item.transform(transformation)
    }
}
