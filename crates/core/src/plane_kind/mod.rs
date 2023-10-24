use crate::pixel::Pixel;
use crate::plane_kind::error::PlaneError;

#[cfg(target_arch = "wasm32")]
pub mod canvas;
pub mod error;
pub mod plane;

pub type PlaneResult<T> = Result<T, PlaneError>;

/// Used as resolution and coordinates
pub type SIZE = u32;

pub trait PlaneLogic: Sized {
    /// Create a new [`Self`] with the given resolution. Fills the item with only ZEROS.
    ///
    /// Warning: this method will allocate `width * height * 4` bytes in memory!
    fn new(width: SIZE, height: SIZE) -> PlaneResult<Self> {
        Self::new_with_fill(width, height, Pixel::ZERO)
    }

    /// Create a new [`Self`] with the given resolution. Fills the item with the given `color`.
    fn new_with_fill(width: SIZE, height: SIZE, color: Pixel) -> PlaneResult<Self> {
        Self::from_data(
            width,
            height,
            // wrapping_mul because I don't want to have a panic if the result would exceed u32 and `from_data` has checks to catch the overflow
            vec![color; (width.wrapping_mul(height)) as usize],
        )
    }

    /// Create a new [`Self`] with the given resolution and fill it with the pixel information from `data`.
    fn from_data(width: SIZE, height: SIZE, data: Vec<Pixel>) -> PlaneResult<Self> {
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

    /// Create a new [`Self`] with the given resolution and fill it with the pixel information from `data`. For more infos about the items inside `data` see [`Pixel`]
    ///
    /// Warning: this function doesn't check if `width * height == data.len()`, `width != 0` or `height != 0`.
    /// It will accept any values!
    ///
    /// For a safe use, it's advised to use the [`from_data`](Self::from_data) method to create a new [`Self`] from data.
    fn from_data_unchecked(width: SIZE, height: SIZE, data: Vec<Pixel>) -> Self;

    /// Returns a reference to the data. For more infos about the items see [`Pixel`]
    fn as_data(&self) -> &Vec<Pixel>;

    /// Returns a mutable reference to the data. For more infos about the items see [`Pixel`]
    fn as_data_mut(&mut self) -> &mut Vec<Pixel>;

    /// Returns the data but flatten. For more infos about the items see [`Pixel::to_raw`](Pixel::to_raw)
    ///
    /// ```rust
    /// use rusvid_core::pixel::Pixel;
    ///
    /// let p = Pixel::new(255, 0, 0, 100);
    ///
    /// assert_eq!(p.to_raw(), [255, 0, 0, 100]);
    /// ```
    fn as_data_flatten(&self) -> Vec<u8> {
        self.as_data().iter().flat_map(|p| p.to_raw()).collect()
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
    fn as_data_packed(&self) -> Vec<u32> {
        self.as_data().iter().map(|p| p.to_raw_packed()).collect()
    }

    /// Returns the plane's height as [`SIZE`]
    fn width(&self) -> SIZE;

    /// Returns the plane's width as [`SIZE`]
    fn height(&self) -> SIZE;

    /// Fill all pixels from the [`Self`] with the given `color`. For more infos see [`Pixel`]
    fn fill(&mut self, color: Pixel);

    /// Get the [`Pixel`] from the given coordinates. Returns `None` if the coordinates are invalid.
    fn pixel(&self, x: SIZE, y: SIZE) -> Option<&Pixel> {
        const_assert_eq!(SIZE::MIN, 0);

        if x > self.width() {
            return None;
        }
        if y > self.height() {
            return None;
        }

        Some(self.pixel_unchecked(x, y))
    }

    /// Get the [`Pixel`] from the given coordinates. Panics if the coordinates are invalid.
    /// For a more safer method see [`pixel`](PlaneLogic::pixel).
    fn pixel_unchecked(&self, x: SIZE, y: SIZE) -> &Pixel;

    /// Get a mutable reference of the [`Pixel`] from the given coordinates. Returns `None` if the coordinates are invalid.
    fn pixel_mut(&mut self, x: SIZE, y: SIZE) -> Option<&mut Pixel> {
        if x > self.width() {
            return None;
        }
        if y > self.height() {
            return None;
        }

        Some(self.pixel_mut_unchecked(x, y))
    }

    /// Get a mutable reference of the [`Pixel`] from the given coordinates. Panics if the coordinates are invalid.
    /// For a more safer method see [`pixel_mut`](PlaneLogic::pixel_mut).
    fn pixel_mut_unchecked(&mut self, x: SIZE, y: SIZE) -> &mut Pixel;

    /// Update the [`Pixel`] with the given coordinates with the `value`. Returns an [`PlaneError`] if the coordinates are invalid.
    fn put_pixel(&mut self, x: SIZE, y: SIZE, value: Pixel) -> PlaneResult<()> {
        *self.pixel_mut(x, y).ok_or(PlaneError::OutOfBound2d(x, y))? = value;

        Ok(())
    }

    /// Update the [`Pixel`] with the given coordinates with the `value`. Panics if the coordinates are invalid.
    /// For a more safer method see [`put_pixel`](PlaneLogic::put_pixel).
    fn put_pixel_unchecked(&mut self, x: SIZE, y: SIZE, value: Pixel) {
        *self.pixel_mut_unchecked(x, y) = value;
    }
}
