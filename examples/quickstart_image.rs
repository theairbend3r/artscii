use std::path::PathBuf;

use artscii::core::{
    canvas::{Canvas, Padding},
    decoder::{decoder::Decoder, image::ImageDecoder},
};

fn main() {
    // load image from disk into a Frame
    let path = PathBuf::from("./../test-images/cuddlyferris.png");
    let img_decoder = ImageDecoder::new(path).decode().unwrap();
    let frame = img_decoder.resize(40, 20).to_ascii().unwrap();

    // optionally load canvas to print
    let canvas = Canvas::new(210, 53);
    canvas.render(frame, Padding::Center);
}
