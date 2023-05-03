use crate::holder::gradient::stop::Stop;
#[cfg(feature = "resvg")]
use crate::holder::utils::TranslateIntoResvgGeneric;
use crate::pixel::Pixel;

#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BaseGradient {
    // TODO add transformation like 'rotation', ...
    stops: Vec<Stop>,
}

impl BaseGradient {
    pub fn new(stops: Vec<Stop>) -> Self {
        BaseGradient { stops }
    }

    pub fn new_from_colors(colors: Vec<Pixel>) -> Self {
        assert!(colors.len() > 1);

        let count = (colors.len() as f64) - 1.0;
        let stops = colors
            .iter()
            .enumerate()
            .map(|(i, c)| Stop {
                offset: (i as f64) / count,
                color: *c,
            })
            .collect();
        Self::new(stops)
    }

    pub fn stops(&self) -> &Vec<Stop> {
        &self.stops
    }
}

#[cfg(feature = "resvg")]
impl TranslateIntoResvgGeneric<resvg::usvg::BaseGradient> for BaseGradient {
    fn translate(&self) -> resvg::usvg::BaseGradient {
        use resvg::usvg::{SpreadMethod, Transform, Units};

        resvg::usvg::BaseGradient {
            units: Units::ObjectBoundingBox,
            transform: Transform::default(),
            spread_method: SpreadMethod::Pad,
            stops: self.stops.iter().map(|s| s.translate()).collect(),
        }
    }
}
