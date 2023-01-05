use crate::holder::likes::ColorLike;
use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::plane::Pixel;

#[derive(Debug, Clone, Copy)]
pub struct Stop {
    pub(crate) offset: f64,
    pub(crate) color: Pixel,
}

impl Stop {
    pub fn new(color: Pixel, offset: f64) -> Self {
        Stop { color, offset }
    }
}

impl TranslateIntoResvgGeneric<resvg::usvg::Stop> for Stop {
    fn translate(&self) -> resvg::usvg::Stop {
        let color = resvg::usvg::Color {
            red: self.color[0],
            green: self.color[1],
            blue: self.color[2],
        };
        let opacity = ColorLike::Color(self.color).translate();

        resvg::usvg::Stop {
            offset: resvg::usvg::NormalizedF64::new(self.offset.abs()).unwrap(),
            color,
            opacity,
        }
    }
}
