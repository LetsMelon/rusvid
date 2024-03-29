use crate::plane::Plane;
use crate::point::Point;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub enum ImageMode {
    // TODO's
    // Crop,
    // Scale,
    // Repeat,
    Fit,
}

impl Default for ImageMode {
    fn default() -> Self {
        ImageMode::Fit
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ImageHolder {
    pub(crate) coordinates: Point,
    pub(crate) size: Point,
    pub(crate) data: Plane,
    pub(crate) mode: ImageMode,
}

impl ImageHolder {
    pub fn new_unchecked(coordinates: Point, size: Point, data: Plane, mode: ImageMode) -> Self {
        ImageHolder {
            coordinates,
            size,
            data,
            mode,
        }
    }

    pub fn new_fit(coordinates: Point, data: Plane) -> Self {
        let size = Point::new(data.width() as f64, data.height() as f64);
        let mode = ImageMode::Fit;

        ImageHolder::new_unchecked(coordinates, size, data, mode)
    }
}
