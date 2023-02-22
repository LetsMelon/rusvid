use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use image::{DynamicImage, ImageFormat, RgbImage, RgbaImage};
use resvg::tiny_skia::Pixmap;
use thiserror::Error;

use crate::frame_image_format::FrameImageFormat;
use crate::pixel::Pixel;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PlaneError {
    #[error("{0} must be greater than 0")]
    ValueGreaterZero(&'static str),

    #[error("width * height must equal data.len()")]
    CapacityError,

    #[error("Error in crate 'Image'")]
    ImageError,

    #[error("Error in crate 'tiny-skia'")]
    TinySkiaError,

    #[error("Can't get item at coordinates x: {0}, y: {1}")]
    OutOfBound2d(u32, u32),
}

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
    pub fn new(width: SIZE, height: SIZE) -> Result<Self, PlaneError> {
        Self::new_with_fill(width, height, Pixel::ZERO)
    }

    pub fn new_with_fill(width: SIZE, height: SIZE, color: Pixel) -> Result<Self, PlaneError> {
        if width <= 0 {
            return Err(PlaneError::ValueGreaterZero("Width"));
        }
        if height <= 0 {
            return Err(PlaneError::ValueGreaterZero("Height"));
        }

        Ok(Plane {
            width,
            height,
            data: vec![color; (width * height) as usize],
        })
    }

    pub fn from_data(width: SIZE, height: SIZE, data: Vec<Pixel>) -> Result<Self, PlaneError> {
        if width * height != data.len() as SIZE {
            return Err(PlaneError::CapacityError);
        }

        Ok(Self::from_data_unchecked(width, height, data))
    }

    pub fn from_data_unchecked(width: SIZE, height: SIZE, data: Vec<Pixel>) -> Self {
        Plane {
            width,
            height,
            data,
        }
    }

    pub fn as_data(&self) -> &Vec<Pixel> {
        &self.data
    }

    pub fn as_data_mut(&mut self) -> &mut Vec<Pixel> {
        &mut self.data
    }

    pub fn as_data_flatten(&self) -> Vec<u8> {
        self.data.iter().flat_map(|p| p.to_raw()).collect()
    }

    /// Crates a `Result<Plane, PlaneError>` from a `image::RgbaImage`
    pub fn from_rgba_image(image: RgbaImage) -> Result<Self, PlaneError> {
        let width = image.width() as SIZE;
        let height = image.height() as SIZE;

        let mut plane = Plane::new(width, height)?;

        for x in 0..plane.width() {
            for y in 0..plane.height() {
                *plane.pixel_mut_unchecked(x, y) = Pixel::new_raw(image.get_pixel(x, y).0);
            }
        }

        Ok(plane)
    }

    pub fn as_rgb_image(self) -> Result<RgbImage, PlaneError> {
        let buf = self
            .data
            .iter()
            .flat_map(|v| [v[0], v[1], v[2]])
            .collect::<Vec<u8>>();

        assert_eq!(self.width() * self.height() * 3, buf.len() as SIZE);

        RgbImage::from_vec(self.width(), self.height(), buf).ok_or(PlaneError::ImageError)
    }

    pub fn as_rgba_image(self) -> Result<RgbaImage, PlaneError> {
        let buf = self
            .data
            .iter()
            .flat_map(|p| p.to_raw())
            .collect::<Vec<u8>>();

        assert_eq!(self.width() * self.height() * 4, buf.len() as SIZE);

        RgbaImage::from_vec(self.width(), self.height(), buf).ok_or(PlaneError::ImageError)
    }

    pub fn from_dynamic_image(image: DynamicImage) -> Result<Self, PlaneError> {
        let width = image.width() as SIZE;
        let height = image.height() as SIZE;

        let data = image
            .as_bytes()
            .iter()
            .array_chunks()
            .map(|[r, g, b]| Pixel::new(*r, *g, *b, 255))
            .collect::<Vec<_>>();

        Plane::from_data(width, height, data)
    }

    /// Crates a `anyhow::Result<Plane>` from a `resvg::tiny_skia::Pixmap`
    pub fn from_pixmap(pixmap: Pixmap) -> Self {
        let data = pixmap
            .pixels()
            .iter()
            .map(|x| {
                let r = x.red();
                let g = x.green();
                let b = x.blue();
                let a = x.alpha();

                Pixel::new_raw([r, g, b, a])
            })
            .collect::<Vec<Pixel>>();

        Plane {
            width: pixmap.width(),
            height: pixmap.height(),
            data,
        }
    }

    pub fn as_pixmap(self) -> Result<Pixmap, PlaneError> {
        let mut pixmap =
            Pixmap::new(self.width(), self.height()).ok_or(PlaneError::TinySkiaError)?;

        let buf = self
            .data
            .iter()
            .flat_map(|p| p.to_raw())
            .collect::<Vec<u8>>();
        pixmap.data_mut()[..buf.len()].copy_from_slice(&buf[..]);

        Ok(pixmap)
    }

    /// Returns the plane's height in `SIZE`
    pub fn width(&self) -> SIZE {
        self.width
    }

    /// Returns the plane's width in `SIZE`
    pub fn height(&self) -> SIZE {
        self.height
    }

    /// Fill all pixel from the plane with the given color
    pub fn fill(&mut self, color: Pixel) {
        self.data = vec![color; (self.width * self.height) as usize]
    }

    pub fn pixel(&self, x: SIZE, y: SIZE) -> Option<&Pixel> {
        debug_assert_eq!(SIZE::MIN, 0);

        if x > self.width {
            return None;
        }
        if y > self.height {
            return None;
        }

        Some(self.pixel_unchecked(x, y))
    }

    pub fn pixel_unchecked(&self, x: SIZE, y: SIZE) -> &Pixel {
        unsafe { self.data.get_unchecked(position_to_index(x, y, self.width)) }
    }

    pub fn pixel_mut(&mut self, x: SIZE, y: SIZE) -> Option<&mut Pixel> {
        if x > self.width {
            return None;
        }
        if y > self.height {
            return None;
        }

        Some(self.pixel_mut_unchecked(x, y))
    }

    pub fn pixel_mut_unchecked(&mut self, x: SIZE, y: SIZE) -> &mut Pixel {
        unsafe {
            self.data
                .get_unchecked_mut(position_to_index(x, y, self.width))
        }
    }

    pub fn put_pixel(&mut self, x: SIZE, y: SIZE, value: Pixel) -> Result<(), PlaneError> {
        *self.pixel_mut(x, y).ok_or(PlaneError::OutOfBound2d(x, y))? = value;

        Ok(())
    }

    pub fn put_pixel_unchecked(&mut self, x: SIZE, y: SIZE, value: Pixel) {
        *self.pixel_mut_unchecked(x, y) = value;
    }

    pub fn into_coordinate_iter(self) -> CoordinateIterator {
        CoordinateIterator {
            plane: self,
            x: 0,
            y: 0,
        }
    }

    pub fn save_with_format<P: AsRef<Path>>(
        self,
        path: P,
        format: FrameImageFormat,
    ) -> Result<(), PlaneError> {
        match format {
            FrameImageFormat::Png => self.save_as_png(path),
            FrameImageFormat::Bmp => self.save_as_bmp(path),
            FrameImageFormat::Jpg => self.save_as_jpg(path),
        }
    }

    // TODO implement first citizen Plane to Bmp
    pub fn save_as_bmp<P: AsRef<Path>>(self, path: P) -> Result<(), PlaneError> {
        let as_image = self.as_rgba_image()?;
        as_image
            .save_with_format(path, ImageFormat::Bmp)
            .map_err(|_| PlaneError::ImageError)
    }

    pub fn save_as_png<P: AsRef<Path>>(self, path: P) -> Result<(), PlaneError> {
        use png::{BitDepth, ColorType, Compression, Encoder};

        let file = File::create(path).unwrap();
        let mut w = BufWriter::new(file);

        let mut encoder = Encoder::new(&mut w, self.width(), self.height());
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        encoder.set_compression(Compression::Best);

        let mut writer = encoder.write_header().unwrap();

        let data = self.as_data_flatten();
        writer.write_image_data(&data).unwrap();

        Ok(())
    }

    // TODO implement first citizen Plane to JPG
    pub fn save_as_jpg<P: AsRef<Path>>(self, path: P) -> Result<(), PlaneError> {
        let as_image = self.as_rgba_image()?;
        as_image
            .save_with_format(path, ImageFormat::Jpeg)
            .map_err(|_| PlaneError::ImageError)
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
        let result = self.plane.data.get(self.index).copied();

        if result.is_some() {
            self.index += 1;
        }

        result
    }
}

pub struct CoordinateIterator {
    plane: Plane,
    x: SIZE,
    y: SIZE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CoordinateIteratorItem {
    pub pixel: Pixel,
    pub x: SIZE,
    pub y: SIZE,
}

impl Iterator for CoordinateIterator {
    type Item = CoordinateIteratorItem;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self
            .plane
            .data
            .get(position_to_index(self.x, self.y, self.plane.width))
            .map(|p| CoordinateIteratorItem {
                pixel: *p,
                x: self.x,
                y: self.y,
            });

        self.x += 1;
        if self.x >= self.plane.width {
            self.x = 0;
            self.y += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::pixel::Pixel;
    use crate::plane::{Plane, PlaneError};

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
        use crate::plane::{Plane, PlaneError};

        #[test]
        fn just_works() {
            let _ = Plane::new(100, 100).unwrap();
            assert!(true);
        }

        #[test]
        fn errors() {
            let e1 = Plane::new(0, 100);
            assert_eq!(e1.unwrap_err(), PlaneError::ValueGreaterZero("Width"));

            let e1 = Plane::new(100, 0);
            assert_eq!(e1.unwrap_err(), PlaneError::ValueGreaterZero("Height"));
        }
    }

    #[test]
    fn from_data() {
        let p = Plane::from_data(2, 2, vec![Pixel::new(255, 0, 0, 0); 4]).unwrap();
        assert_eq!(p.pixel(0, 0).unwrap(), &Pixel::new(255, 0, 0, 0));

        let e = Plane::from_data(2, 2, vec![Pixel::new(255, 0, 0, 0); 3]);
        assert_eq!(e.unwrap_err(), PlaneError::CapacityError);
    }

    #[test]
    fn as_data() {
        let p = Plane::new(2, 2).unwrap();

        let data = p.as_data();
        assert_eq!(data.len(), 4);
        assert_eq!(data[0], *p.pixel(0, 0).unwrap());
    }

    #[test]
    fn as_data_flatten() {
        let p = Plane::new(2, 2).unwrap();

        let data = p.as_data_flatten();
        assert_eq!(data.len(), 4 * 4);
        assert_eq!(data[0], p.pixel(0, 0).unwrap().get_r());
        assert_eq!(data[1], p.pixel(0, 0).unwrap().get_g());
        assert_eq!(data[2], p.pixel(0, 0).unwrap().get_b());
        assert_eq!(data[3], p.pixel(0, 0).unwrap().get_a());
    }

    mod get_pixel {
        use crate::pixel::Pixel;
        use crate::plane::Plane;

        #[test]
        fn not_mutable() {
            let data = vec![
                Pixel::new_raw([255, 0, 0, 255]),
                Pixel::new_raw([0, 255, 0, 255]),
                Pixel::new_raw([0; 4]),
                Pixel::new_raw([0, 0, 255, 255]),
                Pixel::new_raw([255, 255, 255, 255]),
                Pixel::new_raw([255 / 2; 4]),
            ];
            let plane = Plane::from_data(3, 2, data.clone()).unwrap();

            assert!(plane.pixel(5, 0).is_none());
            assert!(plane.pixel(0, 5).is_none());

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
                Pixel::new_raw([255, 0, 0, 255]),
                Pixel::new_raw([0, 255, 0, 255]),
                Pixel::new_raw([0; 4]),
                Pixel::new_raw([0, 0, 255, 255]),
                Pixel::new_raw([255, 255, 255, 255]),
                Pixel::new_raw([255 / 2; 4]),
            ];
            let mut plane = Plane::from_data(3, 2, data.clone()).unwrap();

            assert!(plane.pixel_mut(5, 0).is_none());
            assert!(plane.pixel_mut(0, 5).is_none());

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

    mod put_pixel {
        use crate::pixel::Pixel;
        use crate::plane::{Plane, PlaneError};

        #[test]
        fn safe() {
            let mut p = Plane::new(2, 2).unwrap();

            assert!(p.put_pixel(0, 0, Pixel::new(255, 0, 0, 255)).is_ok());
            assert_eq!(*p.pixel(0, 0).unwrap(), Pixel::new(255, 0, 0, 255));

            assert_eq!(
                p.put_pixel(3, 0, Pixel::new(255, 0, 0, 255)).unwrap_err(),
                PlaneError::OutOfBound2d(3, 0)
            );
        }

        #[test]
        fn unchecked() {
            let mut p = Plane::new(2, 2).unwrap();

            p.put_pixel_unchecked(0, 0, Pixel::new(255, 0, 0, 255));
            assert_eq!(*p.pixel(0, 0).unwrap(), Pixel::new(255, 0, 0, 255));
        }
    }

    mod iterator {
        use crate::pixel::Pixel;
        use crate::plane::Plane;

        #[test]
        fn just_works() {
            let plane = Plane::from_data(
                2,
                2,
                vec![
                    Pixel::new_raw([255, 0, 0, 255]),
                    Pixel::new_raw([0, 255, 0, 255]),
                    Pixel::new_raw([0, 0, 255, 255]),
                    Pixel::new_raw([255, 255, 255, 255]),
                ],
            )
            .unwrap();

            let mut iter = plane.into_iter();

            assert_eq!(Some([255, 0, 0, 255].into()), iter.next());
            assert_eq!(Some([0, 255, 0, 255].into()), iter.next());
            assert_eq!(Some([0, 0, 255, 255].into()), iter.next());
            assert_eq!(Some([255, 255, 255, 255].into()), iter.next());
            assert_eq!(None, iter.next());
        }
    }

    mod coordinate_iterator {
        use crate::pixel::Pixel;
        use crate::plane::Plane;

        #[test]
        fn just_works() {
            let plane = Plane::from_data(
                2,
                2,
                vec![
                    Pixel::new_raw([255, 0, 0, 255]),
                    Pixel::new_raw([0, 255, 0, 255]),
                    Pixel::new_raw([0, 0, 255, 255]),
                    Pixel::new_raw([255, 255, 255, 255]),
                ],
            )
            .unwrap();

            let mut iter = plane.into_coordinate_iter();

            let item = iter.next();
            assert!(item.is_some() && item.is_some_and(|item| item.x == 0 && item.y == 0));
            let item = iter.next();
            assert!(item.is_some() && item.is_some_and(|item| item.x == 1 && item.y == 0));
            let item = iter.next();
            assert!(item.is_some() && item.is_some_and(|item| item.x == 0 && item.y == 1));
            let item = iter.next();
            assert!(item.is_some() && item.is_some_and(|item| item.x == 1 && item.y == 1));
            let item = iter.next();
            assert!(item.is_none());
        }
    }

    mod rgba_image {
        use anyhow::{anyhow, Result};
        use image::{Rgba, RgbaImage};

        use crate::pixel::Pixel;
        use crate::plane::Plane;

        #[test]
        fn from() -> Result<()> {
            fn generate_pixel(x: u32, y: u32) -> Pixel {
                Pixel::new(
                    (x % 255) as u8,
                    ((x + y) % 255) as u8,
                    (y % 255) as u8,
                    ((x + y) & 0xFF) as u8,
                )
            }

            let width = 20;
            let height = 5;
            let rgba_image =
                RgbaImage::from_fn(width, height, |x, y| Rgba(generate_pixel(x, y).to_raw()));
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
    /*
    mod save_as {
        use itertools::*;

        use crate::plane::Plane;

        #[test]
        fn save_as_png() {
            let size = 2_u32.pow(8);

            let plane =
                Plane::from_data(size, size, vec![[255, 100, 0, 255]; (size * size) as usize])
                    .unwrap();

            let saved = plane.clone().save_as_png("test_out.png");
            assert!(saved.is_ok());

            let read_plane = Plane::from_dynamic_image(
                image::io::Reader::open("test_out.png")
                    .unwrap()
                    .decode()
                    .unwrap(),
            )
            .unwrap();

            let same_pixels = (0..size)
                .cartesian_product(0..size)
                .map(|(x, y)| {
                    let p1 = plane.pixel_unchecked(x, y);
                    let p2 = read_plane.pixel_unchecked(x, y);

                    p1[0] == p2[0] && p1[1] == p2[1] && p1[2] == p2[2] && p1[3] == p2[3]
                })
                .fold(true, |mut acc, value| {
                    acc &= value;
                    acc
                });
            assert!(same_pixels)
        }
    }
     */
}
