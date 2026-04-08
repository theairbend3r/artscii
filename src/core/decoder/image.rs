use std::path::PathBuf;

use image::{GenericImageView, ImageReader};

use crate::core::{decoder::decoder::Decoder, frame::Frame};

#[derive(Debug)]
pub struct ImageDecoder {
    path: PathBuf,
}

impl ImageDecoder {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
}

impl Decoder for ImageDecoder {
    fn decode(&self) -> anyhow::Result<Frame> {
        let img = ImageReader::open(&self.path)?.decode()?;
        let (width, height) = img.dimensions();
        let pixels = img.into_luma8().into_raw();

        Ok(Frame {
            pixels,
            width,
            height,
        })
    }
}
