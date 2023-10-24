use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

use crate::pixel::Pixel;
use crate::plane_kind::{PlaneLogic, SIZE};

#[derive(Debug)]
pub struct Canvas<'a> {
    inner: &'a CanvasRenderingContext2d,
    width: SIZE,
    height: SIZE,
}

impl<'a> Canvas<'a> {
    pub fn new(canvas: &'a CanvasRenderingContext2d, width: SIZE, height: SIZE) -> Self {
        Canvas {
            inner: canvas,
            width,
            height,
        }
    }

    pub fn get_inner_canvas(&self) -> &CanvasRenderingContext2d {
        &self.inner
    }
}

impl<'a> PlaneLogic for Canvas<'a> {
    fn from_data_unchecked(width: SIZE, height: SIZE, data: Vec<Pixel>) -> Self {
        let _data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(
                data.iter()
                    .flat_map(|item| item.to_raw())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            width,
            height,
        )
        .unwrap();

        todo!()
    }

    fn as_data(&self) -> &Vec<Pixel> {
        todo!()
        // &self
        //     .inner
        //     .get_image_data(0.0, 0.0, self.width() as f64, self.height() as f64)
        //     .unwrap()
        //     .data()
        //     .0
        //     .windows(4)
        //     .map(|[r, g, b, a]| Pixel::new(*r, *g, *b, *a))
        //     .collect::<Vec<_>>()
    }

    fn as_data_mut(&mut self) -> &mut Vec<Pixel> {
        todo!()
    }

    fn width(&self) -> SIZE {
        self.width
    }

    fn height(&self) -> SIZE {
        self.height
    }

    fn fill(&mut self, color: Pixel) {
        let raw_color = color.to_raw();

        self.inner.set_fill_style(
            &format!(
                "rgba({},{},{},{})",
                raw_color[0],
                raw_color[1],
                raw_color[2],
                raw_color[3] / 255
            )
            .into(),
        );
        self.inner
            .fill_rect(0.0, 0.0, self.width() as f64, self.height() as f64);
    }

    fn pixel_unchecked(&self, x: SIZE, y: SIZE) -> &Pixel {
        todo!()
    }

    fn pixel_mut_unchecked(&mut self, x: SIZE, y: SIZE) -> &mut Pixel {
        todo!()
    }

    fn put_pixel_unchecked(&mut self, x: SIZE, y: SIZE, value: Pixel) {
        let raw_color = value.to_raw();

        self.inner.set_fill_style(
            &format!(
                "rgba({},{},{},{})",
                raw_color[0],
                raw_color[1],
                raw_color[2],
                raw_color[3] / 255
            )
            .into(),
        );
        self.inner.fill_rect(x as f64, y as f64, 1.0, 1.0);
    }
}
