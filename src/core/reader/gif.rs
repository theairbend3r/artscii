use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::core::frame::Frame;

pub struct ReaderGif<'a> {
    inner: image::Frames<'a>,
}

impl ReaderGif<'_> {
    pub fn new(path: PathBuf) -> Self {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let decoder = GifDecoder::new(reader).unwrap();

        let gif_frames = decoder.into_frames();

        Self { inner: gif_frames }
    }
}

impl Iterator for ReaderGif<'_> {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        let nextframe = self.inner.next()?;
        let nextframe = nextframe.ok()?;
        let buffer = nextframe.into_buffer();

        let (width, height) = buffer.dimensions();

        let gray = DynamicImage::ImageRgba8(buffer).to_luma8();
        let pixels = gray.into_raw();
        let frame = Frame::new(pixels, width, height);

        // let mut gray = Vec::new();
        // for pixel in buffer.pixels() {
        //     let [r, g, b, _a] = pixel.0;
        //     let y = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
        //     gray.push(y)
        // }
        //
        // let frame = Frame::new(gray, width, height);

        Some(frame)
    }
}
