use crate::plane::Pixel;

#[derive(Debug, Clone, Copy)]
pub enum ColorLike {
    Color(Pixel),
}

impl ColorLike {
    pub fn from_resvg_paint(paint: &resvg::usvg::Paint) -> Self {
        use resvg::usvg::Paint;

        match paint {
            Paint::Color(c) => ColorLike::Color([c.red, c.green, c.blue, 255]),
            _ => todo!(),
        }
    }

    pub fn as_resvg_paint(&self) -> resvg::usvg::Paint {
        use resvg::usvg::{Color, Paint};

        match self {
            ColorLike::Color(c) => Paint::Color(Color {
                red: c[0],
                green: c[1],
                blue: c[2],
            }),
        }
    }
}
