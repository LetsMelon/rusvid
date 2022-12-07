use std::rc::Rc;
use std::vec;

use anyhow::{Context, Result};
use rusvid_core::plane::{Pixel, Plane, SIZE};
use rusvid_core::point::Point;
use tiny_skia::Pixmap;
use usvg::{
    AspectRatio, NodeExt, Opacity, PathData, PathSegment, Size, StrokeWidth, Tree, ViewBox,
};

pub mod path_like;

use path_like::PathLike;

#[derive(Debug)]
pub enum ColorLike {
    Color(Pixel),
}

#[derive(Debug)]
pub struct Svg {
    pub path: Vec<PathLike>,
    // pub fill_color: Option<ColorLike>,
}

#[derive(Debug)]
pub enum Types {
    Svg(Svg),
}

#[derive(Debug)]
pub struct Object {
    pub data: Types,
    pub id: String,
}

fn path_like_to_path_segment(path: &Vec<PathLike>) -> Vec<PathSegment> {
    path.iter().map(|p| p.to_usvg_path_segment()).collect()
}

#[derive(Debug)]
pub enum Transform {
    Move(Point),
}

impl Object {
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
            Types::Svg(svg) => path_like_to_path_segment(&svg.path),
        };

        let mut path = PathData::with_capacity(usvg_path.len());
        path.extend_from_slice(&usvg_path);

        tree.root().append_kind(usvg::NodeKind::Path(usvg::Path {
            id: self.id.clone(),
            fill: Some(usvg::Fill {
                paint: usvg::Paint::Color(usvg::Color {
                    red: 200,
                    green: 0,
                    blue: 0,
                }),
                opacity: Opacity::new(1.0),
                ..Default::default()
            }),
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
                Types::Svg(svg) => {
                    svg.path = svg
                        .path
                        .iter()
                        .map(|p| match p {
                            PathLike::Move(og_p) => PathLike::Move(*og_p + point),
                            PathLike::Line(og_p) => PathLike::Line(*og_p + point),
                            PathLike::Close => PathLike::Close,
                        })
                        .collect::<Vec<_>>()
                }
            },
        };

        Ok(())
    }
}
