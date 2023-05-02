use resvg::tiny_skia::Pixmap;
use resvg::usvg::{AspectRatio, NodeExt, Size, Tree, ViewBox};

use crate::holder::backend::Backend;
use crate::holder::likes::TypesLike;
use crate::holder::object::Object;
use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::plane::{Plane, PlaneError, PlaneResult, SIZE};

#[derive(Debug)]
pub struct ResvgBackend {}

impl Backend for ResvgBackend {
    fn name(&self) -> &'static str {
        "resvg"
    }

    fn render(&self, object: &Object, width: SIZE, height: SIZE) -> PlaneResult<Plane> {
        match object.data() {
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
                    resvg::FitTo::Original,
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
}
