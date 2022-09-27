use image::ImageFormat;

#[derive(Debug, Clone, Copy)]
pub enum FrameImageFormat {
    Png,
    Bmp,
    Jpg,
}

impl FrameImageFormat {
    #[inline]
    pub(crate) fn file_extension(&self) -> String {
        match self {
            FrameImageFormat::Png => "png".to_string(),
            FrameImageFormat::Bmp => "bmp".to_string(),
            FrameImageFormat::Jpg => "jpg".to_string(),
        }
    }

    #[inline]
    pub(crate) fn as_image_format(&self) -> ImageFormat {
        match self.clone() {
            FrameImageFormat::Png => ImageFormat::Png,
            FrameImageFormat::Bmp => ImageFormat::Bmp,
            FrameImageFormat::Jpg => ImageFormat::Jpeg,
        }
    }
}

impl Default for FrameImageFormat {
    fn default() -> Self {
        FrameImageFormat::Png
    }
}
