mod box_blur;
mod color_palette;
mod gaussian_blur;
mod grayscale;
mod pixelate;

#[cfg(feature = "scripting")]
mod scripting;

pub use box_blur::BoxBlur;
pub use color_palette::ColorPaletteEffect;
pub use gaussian_blur::GaussianBlur;
pub use grayscale::GrayscaleEffect;
pub use pixelate::PixelateEffect;
#[cfg(feature = "scripting")]
pub use scripting::ScriptingEffect;

/*
TODO effects to implement

- https://en.wikipedia.org/wiki/Colour_banding
- https://en.wikipedia.org/wiki/Dither
- https://en.wikipedia.org/wiki/Posterization
- https://en.wikipedia.org/wiki/Median_filter
- https://en.wikipedia.org/wiki/Image_noise
 */
