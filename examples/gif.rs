use artscii::core::canvas::{Canvas, Padding};
use artscii::core::frame::Frame;
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage};
use std::fs::File;
use std::io::BufReader;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

struct FrameIter<'a> {
    inner: image::Frames<'a>,
}

impl FrameIter<'_> {
    fn new() -> Self {
        let file = File::open("./../test-images/countdown.gif").unwrap();
        let reader = BufReader::new(file);
        let decoder = GifDecoder::new(reader).unwrap();

        let gif_frames = decoder.into_frames();

        Self { inner: gif_frames }
    }
}

impl Iterator for FrameIter<'_> {
    type Item = Frame;

    fn next(&mut self) -> Option<Self::Item> {
        let nextframe = self.inner.next().unwrap();
        let buffer = nextframe.unwrap().into_buffer();

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gif = FrameIter::new();
    let canvas = Canvas::new(210, 53);

    for f in gif {
        // println!("{:?}, {:?}", f.pixels.len(), &f.pixels[..5]);
        let f = f.resize(80, 40).to_ascii().unwrap();

        // clear screen + move cursor to top-left
        print!("\x1b[2J\x1b[H");
        canvas.render(f, Padding::Center);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
