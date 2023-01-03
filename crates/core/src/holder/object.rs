use std::collections::HashMap;
use std::rc::Rc;

use resvg::tiny_skia::Pixmap;
use resvg::usvg::{AspectRatio, NodeExt, PathData, Size, Tree, ViewBox};

use crate::holder::likes::path_like::PathLike;
use crate::holder::likes::types_like::TypesLike;
use crate::holder::transform::{Transform, TransformError, TransformLogic};
use crate::holder::utils;
use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::plane::{Plane, PlaneError, SIZE};

#[derive(Debug)]
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
        match &self.data {
            TypesLike::Svg(svg) => {
                let size = Size::new(width as f64, height as f64)
                    .ok_or(PlaneError::ValueGreaterZero("width or height"))?;

                let tree = Tree {
                    size,
                    view_box: ViewBox {
                        rect: size.to_rect(0.0, 0.0),
                        aspect: AspectRatio::default(),
                    },
                    root: resvg::usvg::Node::new(resvg::usvg::NodeKind::Group(
                        resvg::usvg::Group::default(),
                    )),
                };

                for item in svg.items.values() {
                    let node_kind = item.translate();
                    tree.root.append_kind(node_kind);
                }

                let mut pixmap = Pixmap::new(width, height).ok_or(PlaneError::TinySkiaError)?;

                resvg::render(
                    &tree,
                    resvg::usvg::FitTo::Original,
                    resvg::tiny_skia::Transform::default(),
                    pixmap.as_mut(),
                );

                Ok(Plane::from_pixmap(pixmap))
            }
            TypesLike::Image(image_holder) => {
                let mut plane = Plane::new(width, height)?;

                // TODO add offset (image.x.abs()) to enumerate
                // image.x < 0
                if true
                /* TODO self.visibility */
                {
                    let x_iter = (image_holder.coordinates.x() as u32).min(width)
                        ..((image_holder.coordinates + image_holder.size).x() as u32).min(width);

                    for (x_image_cord, x_plane_cord) in x_iter.enumerate() {
                        let x_image_cord = x_image_cord as u32;

                        let y_iter = (image_holder.coordinates.y() as u32).min(height)
                            ..((image_holder.coordinates + image_holder.size).y() as u32)
                                .min(height);

                        for (y_image_cord, y_plane_cord) in y_iter.enumerate() {
                            let pixel = image_holder
                                .data
                                .pixel_unchecked(x_image_cord, y_image_cord as u32);

                            plane.put_pixel_unchecked(x_plane_cord, y_plane_cord, *pixel);
                        }
                    }
                }

                Ok(plane)
            }
        }
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
