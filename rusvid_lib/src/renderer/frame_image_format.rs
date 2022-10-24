use image::ImageFormat;

#[derive(Debug, Clone, Copy)]
pub enum FrameImageFormat {
    Png,
    Bmp,
    Jpg,
    Avif,
}

impl FrameImageFormat {
    #[inline]
    pub(crate) fn file_extension(&self) -> String {
        match self {
            FrameImageFormat::Png => "png".to_string(),
            FrameImageFormat::Bmp => "bmp".to_string(),
            FrameImageFormat::Jpg => "jpg".to_string(),
            FrameImageFormat::Avif => "avif".to_string(),
        }
    }

    #[inline]
    pub(crate) fn as_image_format(&self) -> ImageFormat {
        match self {
            FrameImageFormat::Png => ImageFormat::Png,
            FrameImageFormat::Bmp => ImageFormat::Bmp,
            FrameImageFormat::Jpg => ImageFormat::Jpeg,
            FrameImageFormat::Avif => ImageFormat::Avif,
        }
    }
}

impl Default for FrameImageFormat {
    fn default() -> Self {
        FrameImageFormat::Png
    }
}

#[cfg(test)]
mod tests {
    use image::ImageFormat;

    use super::FrameImageFormat;

    #[test]
    fn get_file_extensions() {
        assert_eq!(&FrameImageFormat::Png.file_extension(), "png");
        assert_eq!(&FrameImageFormat::Bmp.file_extension(), "bmp");
        assert_eq!(&FrameImageFormat::Jpg.file_extension(), "jpg");
        assert_eq!(&FrameImageFormat::Avif.file_extension(), "avif");
    }

    #[test]
    fn converts_into_image_crate() {
        assert_eq!(FrameImageFormat::Png.as_image_format(), ImageFormat::Png);
        assert_eq!(FrameImageFormat::Bmp.as_image_format(), ImageFormat::Bmp);
        assert_eq!(FrameImageFormat::Jpg.as_image_format(), ImageFormat::Jpeg);
        assert_eq!(FrameImageFormat::Avif.as_image_format(), ImageFormat::Avif);
    }
}
