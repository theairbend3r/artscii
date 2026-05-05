use std::path::PathBuf;

use image::{DynamicImage, GenericImageView, ImageReader};

use crate::core::frame::{ColourStyle, Frame};

#[derive(Debug)]
pub struct ReaderImage {
    path: PathBuf,
}

impl ReaderImage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn read(&self) -> anyhow::Result<Frame> {
        let img = ImageReader::open(&self.path)?.decode()?;
        let (width, height) = img.dimensions();

        let (pixels, colourstyle) = match img {
            // 4 byte per pixel
            DynamicImage::ImageRgba8(rgba) => (rgba.into_raw(), ColourStyle::Rgba),
            // 3 byte per pixel
            DynamicImage::ImageRgb8(rgb) => (rgb.into_raw(), ColourStyle::Rgb),
            // 1 byte per pixel
            DynamicImage::ImageLuma8(gray) => (gray.into_raw(), ColourStyle::Gray),
            // 4 byte per pixel
            _ => (img.to_rgba8().into_raw(), ColourStyle::Rgba),
        };

        let frame = Frame::new(pixels, width, height, colourstyle);

        Ok(frame)
    }
}
