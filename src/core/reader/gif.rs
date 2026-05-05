use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::core::frame::{ColourStyle, Frame};

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

        // buffer is always Rgba8
        let buffer = nextframe.into_buffer();

        let (width, height) = buffer.dimensions();

        let pixels = buffer.into_raw();
        let frame = Frame::new(pixels, width, height, ColourStyle::Rgba);

        Some(frame)
    }
}
