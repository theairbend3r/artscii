use std::path::PathBuf;

use image::{GenericImageView, ImageReader};

use crate::core::frame::Frame;

#[derive(Debug)]
pub struct ReaderImage {
    path: PathBuf,
}

impl ReaderImage {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn decode(&self) -> anyhow::Result<Frame> {
        let img = ImageReader::open(&self.path)?.decode()?;
        let (width, height) = img.dimensions();

        let gray = img.to_luma8();
        let pixels = gray.into_raw();

        let frame = Frame::new(pixels, width, height);

        Ok(frame)
    }
}
