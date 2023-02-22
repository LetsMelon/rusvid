use crate::holder::gradient::linear::LinearGradient;
use crate::holder::gradient::radial::RadialGradient;
use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::pixel::Pixel;

#[derive(Debug, Clone)]
pub enum ColorLike {
    Color(Pixel),
    LinearGradient(LinearGradient),
    RadialGradient(RadialGradient),
}

impl ColorLike {
    pub fn from_resvg_paint(paint: &resvg::usvg::Paint) -> Self {
        use resvg::usvg::Paint;

        match paint {
            Paint::Color(c) => ColorLike::Color(Pixel::new(c.red, c.green, c.blue, 255)),
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
            ColorLike::LinearGradient(l_g) => l_g.translate(),
            ColorLike::RadialGradient(r_g) => r_g.translate(),
        }
    }
}

impl TranslateIntoResvgGeneric<resvg::usvg::Opacity> for ColorLike {
    fn translate(&self) -> resvg::usvg::Opacity {
        match self {
            ColorLike::Color(c) => resvg::usvg::Opacity::new_u8(c[3]),
            // TODO
            ColorLike::LinearGradient(_) | ColorLike::RadialGradient(_) => {
                resvg::usvg::Opacity::ONE
            }
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
