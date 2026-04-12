use artscii::core::canvas::{Canvas, Padding};
use artscii::core::frame::Frame;
use image::codecs::gif::GifDecoder;
use image::{AnimationDecoder, DynamicImage, ImageDecoder};
use std::fs::File;
use std::io::BufReader;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./../test-images/countdown.gif").unwrap();
    let reader = BufReader::new(file);

    let decoder = GifDecoder::new(reader).unwrap();
    let (width, height) = decoder.dimensions();
    println!("{} {}", width, height);

    let frames = decoder.into_frames();
    let canvas = Canvas::new(210, 53);

    for frame in frames {
        let frame = frame?;

        let gray = DynamicImage::ImageRgba8(frame.into_buffer()).to_luma8();
        let pixels = gray.into_raw();
        let f = Frame::new(pixels, width, height)
            .resize(80, 40)
            .to_ascii()
            .unwrap();

        // clear screen + move cursor to top-left
        print!("\x1b[2J\x1b[H");
        canvas.render(f, Padding::Center);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
