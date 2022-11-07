use anyhow::{anyhow, bail, Result};
use image::RgbaImage;
use tiny_skia::Pixmap;

pub type Pixel = [u8; 4];

pub type SIZE = u32;

#[derive(Debug, Clone)]
pub struct Plane {
    width: SIZE,
    height: SIZE,
    data: Vec<Pixel>,
}

#[inline(always)]
fn position_to_index(x: SIZE, y: SIZE, multi: SIZE) -> usize {
    (x + multi * y) as usize
}

/// Coordinate system used: <https://learn.adafruit.com/adafruit-gfx-graphics-library/coordinate-system-and-units>
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
            data: vec![[0; 4]; (width * height) as usize],
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

    /// Crates a `anyhow::Result<Plane>` from a `image::RgbaImage`
    pub fn from_rgba_image(image: RgbaImage) -> Result<Self> {
        let width = image.width() as SIZE;
        let height = image.height() as SIZE;

        let mut plane = Plane::new(width, height)?;

        for x in 0..plane.width() {
            for y in 0..plane.height() {
                *plane.pixel_mut_unchecked(x, y) = image.get_pixel(x, y).0;
            }
        }

        Ok(plane)
    }

    pub fn as_rgba_image(self) -> Result<RgbaImage> {
        let buf = self.data.iter().flatten().map(|x| *x).collect::<Vec<u8>>();

        assert_eq!(self.width() * self.height() * 4, buf.len() as SIZE);

        RgbaImage::from_vec(self.width(), self.height(), buf)
            .ok_or(anyhow!("Error while creating an `image::RgbaImage`"))
    }

    /// Crates a `anyhow::Result<Plane>` from a `tiny_skia::Pixmap`
    pub fn from_pixmap(pixmap: Pixmap) -> Self {
        let data = pixmap
            .pixels()
            .iter()
            .map(|x| {
                let r = x.red();
                let g = x.green();
                let b = x.blue();
                let a = x.alpha();

                [r, g, b, a]
            })
            .collect::<Vec<Pixel>>();

        Plane {
            width: pixmap.width(),
            height: pixmap.height(),
            data,
        }
    }

    pub fn as_pixmap(self) -> Result<Pixmap> {
        let mut pixmap = Pixmap::new(self.width(), self.height())
            .ok_or(anyhow!("Error while creating an `tiny_skia::Pixmap`"))?;

        let data = pixmap.data_mut();

        let buf = self.data.iter().flatten().map(|x| *x).collect::<Vec<u8>>();
        for i in 0..buf.len() {
            data[i] = buf[i];
        }

        Ok(pixmap)
    }

    #[inline(always)]
    /// Returns the plane's height in `SIZE`
    pub fn width(&self) -> SIZE {
        self.width
    }

    /// Returns the plane's width in `SIZE`
    #[inline(always)]
    pub fn height(&self) -> SIZE {
        self.height
    }

    #[inline]
    pub fn pixel(&self, x: SIZE, y: SIZE) -> Option<&Pixel> {
        if x > self.width {
            return None;
        }
        if y > self.height {
            return None;
        }

        let index = position_to_index(x, y, self.width);
        self.data.get(index)
    }

    #[inline]
    pub fn pixel_unchecked(&self, x: SIZE, y: SIZE) -> &Pixel {
        unsafe { self.data.get_unchecked(position_to_index(x, y, self.width)) }
    }

    #[inline]
    pub fn pixel_mut(&mut self, x: SIZE, y: SIZE) -> Option<&mut Pixel> {
        if x > self.width {
            return None;
        }
        if y > self.height {
            return None;
        }

        self.data.get_mut(position_to_index(x, y, self.width))
    }

    #[inline]
    pub fn pixel_mut_unchecked(&mut self, x: SIZE, y: SIZE) -> &mut Pixel {
        unsafe {
            self.data
                .get_unchecked_mut(position_to_index(x, y, self.width))
        }
    }

    #[inline]
    pub fn put_pixel(&mut self, x: SIZE, y: SIZE, value: Pixel) -> Result<()> {
        *self.pixel_mut(x, y).ok_or(anyhow!("Out off bound error"))? = value;

        Ok(())
    }

    #[inline]
    pub fn put_pixel_unchecked(&mut self, x: SIZE, y: SIZE, value: Pixel) {
        *self.pixel_mut_unchecked(x, y) = value;
    }

    #[inline]
    pub fn pixels(&self) -> &[Pixel] {
        self.data.as_slice()
    }

    #[inline]
    pub fn pixels_mut(&mut self) -> &mut [Pixel] {
        self.data.as_mut_slice()
    }
}

impl IntoIterator for Plane {
    type Item = Pixel;
    type IntoIter = PlaneIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PlaneIntoIterator {
            plane: self,
            index: 0,
        }
    }
}

pub struct PlaneIntoIterator {
    plane: Plane,
    index: usize,
}

impl Iterator for PlaneIntoIterator {
    type Item = Pixel;
    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .plane
            .data
            .get(self.index)
            .and_then(|pixel| Some(*pixel));

        if result.is_some() {
            self.index += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn position_to_index_test() {
        use crate::plane::position_to_index;

        let width = 5;
        // could be used like this:
        // let height = 2;
        // let arr = (0..(width * height)).collect::<Vec<u32>>();

        let index = position_to_index(0, 0, width);
        assert_eq!(index, 0);

        let index = position_to_index(4, 0, width);
        assert_eq!(index, 4);

        let index = position_to_index(0, 1, width);
        assert_eq!(index, 5);

        let index = position_to_index(5, 1, width);
        assert_eq!(index, 10);
    }

    mod new {
        use crate::plane::Plane;

        #[test]
        fn just_works() {
            let _ = Plane::new(100, 100).unwrap();
            assert!(true);
        }
    }

    mod get_pixel {
        use crate::plane::Plane;

        #[test]
        fn not_mutable() {
            let data = vec![
                [255, 0, 0, 255],
                [0, 255, 0, 255],
                [0; 4],
                [0, 0, 255, 255],
                [255, 255, 255, 255],
                [255 / 2; 4],
            ];
            let plane = Plane::from_data(3, 2, data.clone()).unwrap();

            assert_eq!(plane.pixel(0, 0).unwrap(), &data[0]);
            assert_eq!(plane.pixel(1, 0).unwrap(), &data[1]);
            assert_eq!(plane.pixel(2, 0).unwrap(), &data[2]);
            assert_eq!(plane.pixel(0, 1).unwrap(), &data[3]);
            assert_eq!(plane.pixel(1, 1).unwrap(), &data[4]);
            assert_eq!(plane.pixel(2, 1).unwrap(), &data[5]);

            assert_eq!(plane.pixel_unchecked(0, 0), &data[0]);
            assert_eq!(plane.pixel_unchecked(1, 0), &data[1]);
            assert_eq!(plane.pixel_unchecked(2, 0), &data[2]);
            assert_eq!(plane.pixel_unchecked(0, 1), &data[3]);
            assert_eq!(plane.pixel_unchecked(1, 1), &data[4]);
            assert_eq!(plane.pixel_unchecked(2, 1), &data[5]);
        }

        #[test]
        fn mutable() {
            let data = vec![
                [255, 0, 0, 255],
                [0, 255, 0, 255],
                [0; 4],
                [0, 0, 255, 255],
                [255, 255, 255, 255],
                [255 / 2; 4],
            ];
            let mut plane = Plane::from_data(3, 2, data.clone()).unwrap();

            assert_eq!(plane.pixel_mut(0, 0).unwrap(), &data[0]);
            assert_eq!(plane.pixel_mut(1, 0).unwrap(), &data[1]);
            assert_eq!(plane.pixel_mut(2, 0).unwrap(), &data[2]);
            assert_eq!(plane.pixel_mut(0, 1).unwrap(), &data[3]);
            assert_eq!(plane.pixel_mut(1, 1).unwrap(), &data[4]);
            assert_eq!(plane.pixel_mut(2, 1).unwrap(), &data[5]);

            assert_eq!(plane.pixel_mut_unchecked(0, 0), &data[0]);
            assert_eq!(plane.pixel_mut_unchecked(1, 0), &data[1]);
            assert_eq!(plane.pixel_mut_unchecked(2, 0), &data[2]);
            assert_eq!(plane.pixel_mut_unchecked(0, 1), &data[3]);
            assert_eq!(plane.pixel_mut_unchecked(1, 1), &data[4]);
            assert_eq!(plane.pixel_mut_unchecked(2, 1), &data[5]);
        }
    }

    mod iterator {
        use crate::plane::Plane;

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

            let mut iter = plane.into_iter();

            assert_eq!(Some([255, 0, 0, 255]), iter.next());
            assert_eq!(Some([0, 255, 0, 255]), iter.next());
            assert_eq!(Some([0, 0, 255, 255]), iter.next());
            assert_eq!(Some([255, 255, 255, 255]), iter.next());
            assert_eq!(None, iter.next());
        }
    }

    mod rgba_image {
        use anyhow::{anyhow, Result};
        use image::{Rgba, RgbaImage};

        use crate::plane::Plane;

        #[test]
        fn from() -> Result<()> {
            fn generate_pixel(x: u32, y: u32) -> [u8; 4] {
                [
                    (x % 255) as u8,
                    ((x + y) % 255) as u8,
                    (y % 255) as u8,
                    ((x + y) & 0xFF) as u8,
                ]
            }

            let width = 20;
            let height = 5;
            let rgba_image = RgbaImage::from_fn(width, height, |x, y| Rgba(generate_pixel(x, y)));
            let plane = Plane::from_rgba_image(rgba_image)?;

            for x in 0..width {
                for y in 0..height {
                    let pixel = plane.pixel(x, y).ok_or(anyhow!("Out-off bound pixel"))?;

                    assert_eq!(pixel, &generate_pixel(x, y), "x: {}, y: {}", x, y);
                }
            }

            Ok(())
        }
    }
}
