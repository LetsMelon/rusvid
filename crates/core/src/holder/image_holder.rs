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
    // TODO remove size
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

#[cfg(feature = "cairo")]
impl crate::holder::utils::ApplyToCairoContext for ImageHolder {
    fn apply(&self, context: &cairo::Context) -> Result<(), Box<dyn std::error::Error>> {
        use cairo::{Format, ImageSurface};

        let width = self.data.width();
        let height = self.data.height();

        let stride = unsafe {
            cairo::ffi::cairo_format_stride_for_width(Format::ARgb32.into(), width as i32)
        };

        let raw_data = self
            .data
            .as_data()
            .iter()
            .flat_map(|p| [p.get_b(), p.get_g(), p.get_r(), p.get_a()])
            .collect::<Vec<_>>()
            .into_boxed_slice();

        let image_surface = ImageSurface::create_for_data(
            raw_data,
            Format::ARgb32,
            width as i32,
            height as i32,
            stride,
        )?;

        context.rectangle(
            self.coordinates.x(),
            self.coordinates.y(),
            width as f64,
            height as f64,
        );
        context.clip();
        context.new_path();

        context.set_source_surface(image_surface, self.coordinates.x(), self.coordinates.y())?;
        context.paint()?;

        Ok(())
    }
}
