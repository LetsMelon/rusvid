use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

use image::{DynamicImage, ImageFormat, RgbImage, RgbaImage};
use resvg::tiny_skia::Pixmap;
use thiserror::Error;

use crate::frame_image_format::FrameImageFormat;
use crate::pixel::Pixel;

#[derive(Error, Debug)]
pub enum PlaneError {
    #[error("{0} must be greater than 0")]
    ValueGreaterZero(&'static str),

    #[error("width * height must equal data.len()")]
    ArrayCapacityError,

    #[error("width * height must smaller than {}", SIZE::MAX)]
    CapacityError,

    #[error("Error in crate 'image': {0:?}")]
    ImageError(#[from] image::ImageError),

    #[error("Error in crate 'tiny-skia'")]
    TinySkiaError,

    #[error("Can't get item at coordinates x: {0}, y: {1}")]
    OutOfBound2d(u32, u32),

    #[error("Error from 'std::io': '{0:?}'")]
    IoError(#[from] std::io::Error),

    #[error("Encoding error: {0:?}")]
    EncodingError(String),
}

impl PlaneError {
    pub fn same_variant(&self, other: &PlaneError) -> bool {
        match (self, other) {
            (PlaneError::ValueGreaterZero(_), PlaneError::ValueGreaterZero(_))
            | (PlaneError::ArrayCapacityError, PlaneError::ArrayCapacityError)
            | (PlaneError::CapacityError, PlaneError::CapacityError)
            | (PlaneError::ImageError(_), PlaneError::ImageError(_))
            | (PlaneError::TinySkiaError, PlaneError::TinySkiaError)
            | (PlaneError::OutOfBound2d(_, _), PlaneError::OutOfBound2d(_, _))
            | (PlaneError::IoError(_), PlaneError::IoError(_))
            | (PlaneError::EncodingError(_), PlaneError::EncodingError(_)) => true,
            _ => false,
        }
    }
}

impl PartialEq for PlaneError {
    fn eq(&self, other: &Self) -> bool {
        self.same_variant(other)
    }
}

impl Eq for PlaneError {}

impl From<png::EncodingError> for PlaneError {
    fn from(value: png::EncodingError) -> Self {
        PlaneError::EncodingError(format!("{:?}", value))
    }
}

pub type PlaneResult<T> = Result<T, PlaneError>;

/// Used as resolution and coordinates
pub type SIZE = u32;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
/// Structure to hold pixels for e.g.: a video-frame, an image, ... .
///
/// Can only store pixels as `RGBA` with a bit-depth of `8`, for more info see [`Pixel`]
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
    /// Create a new [`Plane`] with the given resolution. Fills the item with only ZEROS.
    ///
    /// Warning: this method will allocate `width * height * 4` bytes in memory!
    pub fn new(width: SIZE, height: SIZE) -> PlaneResult<Self> {
        Self::new_with_fill(width, height, Pixel::ZERO)
    }

    /// Create a new [`Plane`] with the given resolution. Fills the item with the given `color`.
    pub fn new_with_fill(width: SIZE, height: SIZE, color: Pixel) -> PlaneResult<Self> {
        Self::from_data(
            width,
            height,
            // wrapping_mul because I don't want to have a panic if the result would exceed u32 and `from_data` has checks to catch the overflow
            vec![color; (width.wrapping_mul(height)) as usize],
        )
    }

    /// Create a new [`Plane`] with the given resolution and fill it with the pixel information from `data`.
    pub fn from_data(width: SIZE, height: SIZE, data: Vec<Pixel>) -> PlaneResult<Self> {
        const_assert_eq!(SIZE::MIN, 0);
        if width == 0 {
            return Err(PlaneError::ValueGreaterZero("Width"));
        }
        if height == 0 {
            return Err(PlaneError::ValueGreaterZero("Height"));
        }

        let (_, overflow) = width.overflowing_mul(height);
        if overflow {
            return Err(PlaneError::CapacityError);
        }

        if width * height != data.len() as SIZE {
            return Err(PlaneError::ArrayCapacityError);
        }

        Ok(Self::from_data_unchecked(width, height, data))
    }

    /// Create a new [`Plane`] with the given resolution and fill it with the pixel information from `data`. For more infos about the items inside `data` see [`Pixel`]
    ///
    /// Warning: this function doesn't check if `width * height == data.len()`, `width != 0` or `height != 0`.
    /// It will accept any values!
    ///
    /// For a safe use, it's advised to use the [`from_data`](Plane::from_data) method to create a new [`Plane`] from data.
    pub fn from_data_unchecked(width: SIZE, height: SIZE, data: Vec<Pixel>) -> Self {
        Plane {
            width,
            height,
            data,
        }
    }

    /// Returns a reference to the data. For more infos about the items see [`Pixel`]
    pub fn as_data(&self) -> &Vec<Pixel> {
        &self.data
    }

    /// Returns a mutable reference to the data. For more infos about the items see [`Pixel`]
    pub fn as_data_mut(&mut self) -> &mut Vec<Pixel> {
        &mut self.data
    }

    /// Returns the data but flatten. For more infos about the items see [`Pixel::to_raw`](Pixel::to_raw)
    ///
    /// ```rust
    /// use rusvid_core::pixel::Pixel;
    ///
    /// let p = Pixel::new(255, 0, 0, 100);
    ///
    /// assert_eq!(p.to_raw(), [255, 0, 0, 100]);
    /// ```
    pub fn as_data_flatten(&self) -> Vec<u8> {
        self.data.iter().flat_map(|p| p.to_raw()).collect()
    }

    /// Returns the data but flatten and packed. For more infos about the items see [`Pixel::to_raw_packed`](Pixel::to_raw_packed)
    ///
    /// ```rust
    /// use rusvid_core::pixel::Pixel;
    ///
    /// let p = Pixel::new(0xFF, 0x00, 0xAA, 0x77);
    ///
    /// assert_eq!(p.to_raw_packed(), 0x77FF00AA);
    /// //                            ^-- format: 0xAARRGGBB
    /// ```
    pub fn as_data_packed(&self) -> Vec<u32> {
        self.data.iter().map(|p| p.to_raw_packed()).collect()
    }

    /// Tries to create a [`Plane`] from [`image::RgbaImage`] or returns a [`PlaneError`].
    pub fn from_rgba_image(image: RgbaImage) -> PlaneResult<Self> {
        let width = image.width() as SIZE;
        let height = image.height() as SIZE;

        let mut plane = Plane::new(width, height)?;

        // TODO use an iterator instead of the two for loops
        for x in 0..plane.width() {
            for y in 0..plane.height() {
                *plane.pixel_mut_unchecked(x, y) = Pixel::new_raw(image.get_pixel(x, y).0);
            }
        }

        Ok(plane)
    }

    /// Consumes itself and tries to create an [`image::RgbImage`] or returns a [`PlaneError`].
    pub fn as_rgb_image(self) -> PlaneResult<RgbImage> {
        let buf = self
            .data
            .iter()
            .flat_map(|v| [v[0], v[1], v[2]])
            .collect::<Vec<u8>>();

        if self.width() * self.height() * 3 == buf.len() as SIZE {
            return Err(PlaneError::ArrayCapacityError);
        }

        // TODO maybe find a better way to return a good error
        RgbImage::from_vec(self.width(), self.height(), buf).ok_or(PlaneError::ImageError(
            image::ImageError::Limits(image::error::LimitError::from_kind(
                image::error::LimitErrorKind::DimensionError,
            )),
        ))
    }

    /// Consumes itself and tries to create an [`image::RgbaImage`] or returns a [`PlaneError`].
    pub fn as_rgba_image(self) -> PlaneResult<RgbaImage> {
        let buf = self.as_data_flatten();

        if self.width() * self.height() * 4 == buf.len() as SIZE {
            return Err(PlaneError::ArrayCapacityError);
        }

        // TODO maybe find a better way to return a good error
        RgbaImage::from_vec(self.width(), self.height(), buf).ok_or(PlaneError::ImageError(
            image::ImageError::Limits(image::error::LimitError::from_kind(
                image::error::LimitErrorKind::DimensionError,
            )),
        ))
    }

    /// Tries to create a [`Plane`] from [`image::DynamicImage`] or returns a [`PlaneError`].
    pub fn from_dynamic_image(image: DynamicImage) -> PlaneResult<Self> {
        let width = image.width() as SIZE;
        let height = image.height() as SIZE;

        let data = image
            .as_bytes()
            .chunks(3)
            .map(|channels| Pixel::new(channels[0], channels[1], channels[2], 255))
            .collect::<Vec<_>>();

        Plane::from_data(width, height, data)
    }

    /// Create a [`Plane`] from [`tiny_skia::Pixmap`]
    pub fn from_pixmap(pixmap: Pixmap) -> Self {
        let data = pixmap
            .pixels()
            .iter()
            .map(|x| {
                let c = x.get();
                let bytes = c.to_ne_bytes();

                Pixel::new_raw(bytes)
            })
            .collect::<Vec<Pixel>>();

        Plane::from_data_unchecked(pixmap.width(), pixmap.height(), data)
    }

    /// Consumes itself and tries to create an [`tiny_skia::Pixmap`] or returns a [`PlaneError`].
    pub fn as_pixmap(self) -> PlaneResult<Pixmap> {
        let mut pixmap =
            Pixmap::new(self.width(), self.height()).ok_or(PlaneError::TinySkiaError)?;

        let buf = self.as_data_flatten();
        pixmap.data_mut()[..buf.len()].copy_from_slice(&buf[..]);

        Ok(pixmap)
    }

    /// Returns the plane's height as [`SIZE`]
    pub fn width(&self) -> SIZE {
        self.width
    }

    /// Returns the plane's width as [`SIZE`]
    pub fn height(&self) -> SIZE {
        self.height
    }

    /// Fill all pixels from the [`Plane`] with the given `color`. For more infos see [`Pixel`]
    pub fn fill(&mut self, color: Pixel) {
        self.data.fill(color)
    }

    /// Get the [`Pixel`] from the given coordinates. Returns `None` if the coordinates are invalid.
    pub fn pixel(&self, x: SIZE, y: SIZE) -> Option<&Pixel> {
        const_assert_eq!(SIZE::MIN, 0);

        if x > self.width {
            return None;
        }
        if y > self.height {
            return None;
        }

        Some(self.pixel_unchecked(x, y))
    }

    /// Get the [`Pixel`] from the given coordinates. Panics if the coordinates are invalid.
    /// For a more safer method see [`pixel`](Plane::pixel).
    pub fn pixel_unchecked(&self, x: SIZE, y: SIZE) -> &Pixel {
        unsafe { self.data.get_unchecked(position_to_index(x, y, self.width)) }
    }

    /// Get a mutable reference of the [`Pixel`] from the given coordinates. Returns `None` if the coordinates are invalid.
    pub fn pixel_mut(&mut self, x: SIZE, y: SIZE) -> Option<&mut Pixel> {
        if x > self.width {
            return None;
        }
        if y > self.height {
            return None;
        }

        Some(self.pixel_mut_unchecked(x, y))
    }

    /// Get a mutable reference of the [`Pixel`] from the given coordinates. Panics if the coordinates are invalid.
    /// For a more safer method see [`pixel_mut`](Plane::pixel_mut).
    pub fn pixel_mut_unchecked(&mut self, x: SIZE, y: SIZE) -> &mut Pixel {
        unsafe {
            self.data
                .get_unchecked_mut(position_to_index(x, y, self.width))
        }
    }

    /// Update the [`Pixel`] with the given coordinates with the `value`. Returns an [`PlaneError`] if the coordinates are invalid.
    pub fn put_pixel(&mut self, x: SIZE, y: SIZE, value: Pixel) -> PlaneResult<()> {
        *self.pixel_mut(x, y).ok_or(PlaneError::OutOfBound2d(x, y))? = value;

        Ok(())
    }

    /// Update the [`Pixel`] with the given coordinates with the `value`. Panics if the coordinates are invalid.
    /// For a more safer method see [`put_pixel`](Plane::put_pixel).
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

    /// Save the plane with the given [`FrameImageFormat`] as the `path`.
    ///
    /// See also [`save_as_bmp`](Plane::save_as_bmp), [`save_as_png`](Plane::save_as_png) and [`save_as_jpg`](Plane::save_as_jpg) for the underlying functions.
    ///
    /// If the `path` doesn't have a extension than this function will the corresponding extension of `format` to the path.
    ///
    /// The internal code for the file extension looks something like that:
    ///
    /// ```rust
    /// use std::path::PathBuf;
    /// use rusvid_core::frame_image_format::FrameImageFormat;
    ///
    /// let mut path = PathBuf::from("my_file");
    /// let format = FrameImageFormat::Jpg;
    ///
    /// path.set_extension(format.file_extension());
    ///
    /// assert_eq!(path.to_string_lossy(), "my_file.jpg");
    /// ```
    pub fn save_with_format(
        self,
        path: impl Into<PathBuf>,
        format: FrameImageFormat,
    ) -> PlaneResult<PathBuf> {
        let mut path: PathBuf = path.into();

        if path.extension().is_none() {
            path.set_extension(format.file_extension());
        }

        match format {
            FrameImageFormat::Png => self.save_as_png(path.clone())?,
            FrameImageFormat::Bmp => self.save_as_bmp(path.clone())?,
            FrameImageFormat::Jpg => self.save_as_jpg(path.clone())?,
        };

        Ok(path)
    }

    // TODO implement first citizen `Plane-to-Bmp` function
    /// Saves the [`Plane`] as a `bmp` file with the given `path`.
    pub fn save_as_bmp<P: AsRef<Path>>(self, path: P) -> PlaneResult<()> {
        let as_image = self.as_rgba_image()?;
        as_image.save_with_format(path, ImageFormat::Bmp)?;

        Ok(())
    }

    /// Saves the [`Plane`] as a `png` file with the given `path`.
    pub fn save_as_png<P: AsRef<Path>>(self, path: P) -> PlaneResult<()> {
        let file = File::create(path)?;
        let mut w = BufWriter::new(file);

        self.writer_as_png(&mut w)?;

        Ok(())
    }

    /// Writes the [`Plane`] as a `png` file to the given writer.
    pub fn writer_as_png<W: Write>(&self, writer: &mut W) -> PlaneResult<()> {
        use png::{BitDepth, ColorType, Compression, Encoder};

        let mut encoder = Encoder::new(writer, self.width(), self.height());
        encoder.set_color(ColorType::Rgba);
        encoder.set_depth(BitDepth::Eight);
        encoder.set_compression(Compression::Default);

        let mut writer = encoder.write_header()?;

        let data = self.as_data_flatten();
        writer.write_image_data(&data)?;

        Ok(())
    }

    // TODO implement first citizen `Plane-to-JPG` function
    /// Saves the [`Plane`] as a `jpg` file with the given `path`.
    pub fn save_as_jpg<P: AsRef<Path>>(self, path: P) -> PlaneResult<()> {
        let as_image = self.as_rgba_image()?;
        as_image.save_with_format(path, ImageFormat::Jpeg)?;

        Ok(())
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
        assert_eq!(e.unwrap_err(), PlaneError::ArrayCapacityError);
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
