use std::rc::Rc;

use anyhow::{Context, Result};
use rusvid_core::plane::{Plane, SIZE};
use tiny_skia::Pixmap;
use usvg::{AspectRatio, NodeExt, Opacity, PathData, Size, Tree, ViewBox};

use crate::{ColorLike, PathLike, Transform, TypesLike};

#[derive(Debug)]
pub struct Object {
    pub data: TypesLike,
    pub id: String,
    visibility: bool,
}

impl Object {
    pub fn new(id: String, data: TypesLike) -> Self {
        Object {
            data,
            id,
            visibility: true,
        }
    }

    pub fn render(&self, width: SIZE, height: SIZE) -> Result<Plane> {
        let size = Size::new(width as f64, height as f64)
            .context("Width oder height must be greater 0")?;

        let tree = Tree::create(usvg::Svg {
            size,
            view_box: ViewBox {
                rect: size.to_rect(0.0, 0.0),
                aspect: AspectRatio::default(),
            },
        });

        let usvg_path = match &self.data {
            TypesLike::Svg(svg) => PathLike::to_usvg_path_segments(&svg.path),
        };
        let mut path = PathData::with_capacity(usvg_path.len());
        path.extend_from_slice(&usvg_path);

        let color = if let TypesLike::Svg(svg) = &self.data {
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
        } else {
            None
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

    pub fn transform(&mut self, transformation: Transform) -> Result<()> {
        match transformation {
            Transform::Move(point) => match &mut self.data {
                TypesLike::Svg(svg) => {
                    svg.path = svg
                        .path
                        .iter()
                        .map(|p| match p {
                            PathLike::Move(og_p) => PathLike::Move(*og_p + point),
                            PathLike::Line(og_p) => PathLike::Line(*og_p + point),
                            PathLike::Close => PathLike::Close,
                        })
                        .collect::<Vec<PathLike>>()
                }
            },
            Transform::Visibility(visibility) => self.visibility = visibility,
            Transform::Color(color) => match &mut self.data {
                TypesLike::Svg(svg) => {
                    svg.fill_color = color;
                }
            },
            Transform::Position(position) => match &mut self.data {
                TypesLike::Svg(svg) => match svg.path[0] {
                    PathLike::Move(point) => {
                        let offset = position - point;

                        svg.path = svg
                            .path
                            .iter()
                            .map(|p| match p {
                                PathLike::Move(og_p) => PathLike::Move(*og_p + offset),
                                PathLike::Line(og_p) => PathLike::Line(*og_p + offset),
                                PathLike::Close => PathLike::Close,
                            })
                            .collect::<Vec<PathLike>>();
                    }
                    _ => todo!(),
                },
            },
        };

        Ok(())
    }
}
