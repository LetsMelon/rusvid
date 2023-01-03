use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::plane::Pixel;

#[derive(Debug, Clone)]
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
}

impl TranslateIntoResvgGeneric<resvg::usvg::Paint> for ColorLike {
    fn translate(&self) -> resvg::usvg::Paint {
        match self {
            ColorLike::Color(c) => resvg::usvg::Paint::Color(resvg::usvg::Color {
                red: c[0],
                green: c[1],
                blue: c[2],
            }),
        }
    }
}

impl TranslateIntoResvgGeneric<resvg::usvg::Opacity> for ColorLike {
    fn translate(&self) -> resvg::usvg::Opacity {
        match self {
            ColorLike::Color(c) => resvg::usvg::Opacity::new_u8(c[3]),
        }
    }
}

impl TranslateIntoResvgGeneric<resvg::usvg::Fill> for ColorLike {
    fn translate(&self) -> resvg::usvg::Fill {
        let paint = self.translate();
        let opacity = self.translate();

        resvg::usvg::Fill {
            paint,
            opacity,
            ..Default::default()
        }
    }
}
