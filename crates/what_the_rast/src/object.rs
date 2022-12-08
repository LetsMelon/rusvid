use std::rc::Rc;

use anyhow::{Context, Result};
use rusvid_core::plane::{Plane, SIZE};
use tiny_skia::Pixmap;
use usvg::{AspectRatio, NodeExt, Opacity, PathData, Size, Tree, ViewBox};

use crate::{ColorLike, PathLike, Transform, TypesLike};

#[derive(Debug)]
pub struct Object {
    data: TypesLike,
    id: String,
    visibility: bool,
}

impl Object {
    pub fn new(id: impl Into<String>, data: TypesLike) -> Self {
        Object {
            data,
            id: id.into(),
            visibility: true,
        }
    }

    pub fn render(&self, width: SIZE, height: SIZE) -> Result<Plane> {
        match &self.data {
            TypesLike::Svg(svg) => {
                let size = Size::new(width as f64, height as f64)
                    .context("Width oder height must be greater 0")?;

                let tree = Tree::create(usvg::Svg {
                    size,
                    view_box: ViewBox {
                        rect: size.to_rect(0.0, 0.0),
                        aspect: AspectRatio::default(),
                    },
                });

                let usvg_path = PathLike::to_usvg_path_segments(&svg.path);
                let mut path = PathData::with_capacity(usvg_path.len());
                path.extend_from_slice(&usvg_path);

                let color = {
                    let channels = if let ColorLike::Color(c) = &svg.fill_color {
                        Some(*c)
                    } else {
                        None
                    };

                    channels.map(|channels| usvg::Fill {
                        paint: usvg::Paint::Color(usvg::Color {
                            red: channels[0],
                            green: channels[1],
                            blue: channels[2],
                        }),
                        opacity: Opacity::new((channels[3] as f64) / 255.0),
                        ..Default::default()
                    })
                };

                tree.root().append_kind(usvg::NodeKind::Path(usvg::Path {
                    id: self.id.clone(),
                    fill: color,
                    visibility: if self.visibility {
                        usvg::Visibility::Visible
                    } else {
                        usvg::Visibility::Hidden
                    },
                    data: Rc::new(path),
                    ..Default::default()
                }));

                let mut pixmap = Pixmap::new(width, height).context("sth error")?;

                resvg::render(
                    &tree,
                    usvg::FitTo::Original,
                    tiny_skia::Transform::default(),
                    pixmap.as_mut(),
                );

                Ok(Plane::from_pixmap(pixmap))
            }
            TypesLike::Image(image_holder) => {
                let mut plane = Plane::new(width, height)?;

                if self.visibility {
                    let x_iter = (image_holder.coordinates.x() as u32)
                        ..((image_holder.coordinates + image_holder.size).x() as u32);

                    for (x_image_cord, x_plane_cord) in x_iter.enumerate() {
                        let x_image_cord = x_image_cord as u32;

                        let y_iter = (image_holder.coordinates.y() as u32)
                            ..((image_holder.coordinates + image_holder.size).y() as u32);

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

    pub fn transform(&mut self, transformation: Transform) -> Result<()> {
        match transformation {
            Transform::Move(point) => match &mut self.data {
                TypesLike::Svg(svg) => {
                    svg.path = svg
                        .path
                        .iter()
                        .map(|p| match *p {
                            PathLike::Move(og_p) => PathLike::Move(og_p + point),
                            PathLike::Line(og_p) => PathLike::Line(og_p + point),
                            PathLike::Close => PathLike::Close,
                        })
                        .collect::<Vec<PathLike>>()
                }
                TypesLike::Image(image_holder) => {
                    // TODO implement `+=` for `Point` += `Point`
                    image_holder.coordinates = image_holder.coordinates + point;
                }
            },
            Transform::Visibility(visibility) => self.visibility = visibility,
            Transform::Color(color) => match &mut self.data {
                TypesLike::Svg(svg) => {
                    svg.fill_color = color;
                }
                TypesLike::Image(_) => {
                    println!("Transformation `Color` has no effect over TypesLike::Image")
                }
            },
            Transform::Position(position) => match &mut self.data {
                TypesLike::Svg(svg) => match svg.path[0] {
                    PathLike::Move(point) => {
                        let offset = position - point;
                        self.transform(Transform::Move(offset))?
                    }
                    _ => panic!("First element needs to be a `PathLike::Move`"),
                },
                TypesLike::Image(image_holder) => {
                    image_holder.coordinates = position;
                }
            },
        };

        Ok(())
    }

    pub fn transforms(&mut self, transformations: Vec<Transform>) -> Result<()> {
        for transformation in transformations.iter() {
            self.transform(*transformation)?;
        }

        Ok(())
    }
}
