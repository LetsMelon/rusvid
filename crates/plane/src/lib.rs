use anyhow::{bail, Result};
#[cfg(feature = "rgba_image")]
use image::RgbaImage;

pub type Pixel = [u8; 4];

pub type SIZE = u32;

#[derive(Debug)]
pub struct Plane {
    pub(crate) width: SIZE,
    pub(crate) height: SIZE,
    pub(crate) data: Vec<Pixel>,
}

impl Plane {
    pub fn new(width: SIZE, height: SIZE) -> Result<Self> {
        if width == 0 {
            bail!("Width must be greater 0");
        }
        if height == 0 {
            bail!("Height must be greater 0");
        }

        Ok(Plane {
            width,
            height,
            data: Vec::with_capacity((width * height) as usize),
        })
    }

    pub fn from_data(width: SIZE, height: SIZE, data: Vec<Pixel>) -> Result<Self> {
        let mut plane = Self::new(width, height)?;

        if plane.width * plane.height != data.len() as SIZE {
            bail!("Data hasn't the right capacity");
        }

        plane.data = data;

        Ok(plane)
    }

    #[cfg(feature = "rgba_image")]
    pub fn from_rgba_image(image: RgbaImage) -> Result<Self> {
        let pixels = image.to_vec();

        let width = image.width() as SIZE;
        let height = image.height() as SIZE;

        let mut plane = Plane::new(width, height)?;
        let data = plane.get_pixels_mut();

        assert_eq!(data.len() * 4, pixels.len());

        for i in 0..data.len() {
            let color = [
                pixels[(i * 4) + 0],
                pixels[(i * 4) + 1],
                pixels[(i * 4) + 2],
                pixels[(i * 4) + 3],
            ];

            data[i] = color;
        }

        Ok(plane)
    }

    // TODO replace with a macro, so that unsafe can use it
    #[doc(hidden)]
    #[inline(always)]
    fn position_to_index(&self, x: SIZE, y: SIZE) -> SIZE {
        x + self.height * y
    }

    /// Coordinate system: <https://py.processing.org/tutorials/drawing/>
    #[inline]
    pub fn get_pixel(&self, x: SIZE, y: SIZE) -> Option<&Pixel> {
        if x > self.width {
            return None;
        }
        if y > self.height {
            return None;
        }

        self.data.get(self.position_to_index(x, y) as usize)
    }

    /// Coordinate system: <https://py.processing.org/tutorials/drawing/>
    #[inline]
    pub fn get_pixel_unchecked(&self, x: SIZE, y: SIZE) -> &Pixel {
        unsafe {
            self.data
                .get_unchecked(self.position_to_index(x, y) as usize)
        }
    }

    /// Coordinate system: <https://py.processing.org/tutorials/drawing/>
    #[inline]
    pub fn get_pixel_mut(&mut self, x: SIZE, y: SIZE) -> Option<&mut Pixel> {
        if x > self.width {
            return None;
        }
        if y > self.height {
            return None;
        }

        self.data.get_mut((y + self.height * x) as usize)
    }

    /// Coordinate system: <https://py.processing.org/tutorials/drawing/>
    #[inline]
    pub fn get_pixel_unchecked_mut(&mut self, x: SIZE, y: SIZE) -> &Pixel {
        unsafe { self.data.get_unchecked_mut((y + self.height * x) as usize) }
    }

    #[inline]
    pub fn get_pixels(&self) -> &[Pixel] {
        self.data.as_slice()
    }

    #[inline]
    pub fn get_pixels_mut(&mut self) -> &mut [Pixel] {
        self.data.as_mut_slice()
    }
}

#[cfg(test)]
mod tests {
    mod new {
        use crate::Plane;

        #[test]
        fn just_works() {
            let _ = Plane::new(100, 100).unwrap();
            assert!(true);
        }
    }

    mod get_pixel {
        use crate::{Pixel, Plane};

        #[test]
        fn just_works() {
            let plane = Plane::from_data(
                2,
                2,
                vec![
                    [255, 0, 0, 255],
                    [0, 255, 0, 255],
                    [0, 0, 255, 255],
                    [255, 255, 255, 255],
                ],
            )
            .unwrap();

            let pixel: Pixel = [255, 0, 0, 255];
            assert_eq!(plane.get_pixel(0, 0).unwrap(), &pixel);
            let pixel: Pixel = [0, 255, 0, 255];
            assert_eq!(plane.get_pixel(1, 0).unwrap(), &pixel);
            let pixel: Pixel = [0, 0, 255, 255];
            assert_eq!(plane.get_pixel(0, 1).unwrap(), &pixel);
            let pixel: Pixel = [255, 255, 255, 255];
            assert_eq!(plane.get_pixel(1, 1).unwrap(), &pixel);

            let pixel: Pixel = [255, 0, 0, 255];
            assert_eq!(plane.get_pixel_unchecked(0, 0), &pixel);
            let pixel: Pixel = [0, 255, 0, 255];
            assert_eq!(plane.get_pixel_unchecked(1, 0), &pixel);
            let pixel: Pixel = [0, 0, 255, 255];
            assert_eq!(plane.get_pixel_unchecked(0, 1), &pixel);
            let pixel: Pixel = [255, 255, 255, 255];
            assert_eq!(plane.get_pixel_unchecked(1, 1), &pixel);
        }
    }
}