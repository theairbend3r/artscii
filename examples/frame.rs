use std::path::PathBuf;

use artscii::core::{
    canvas::Canvas,
    decoder::{decoder::Decoder, image::ImageDecoder},
};

fn main() {
    // load image from disk into a Frame
    // let path = PathBuf::from("./../test-images/cuddlyferris.png");
    let path = PathBuf::from("./../test-images/1.png");
    let img_decoder = ImageDecoder::new(path);
    let frame = img_decoder.decode().unwrap();
    let frame = frame.to_ascii().unwrap();

    // optionally load canvas to print
    let canvas = Canvas::new(210, 53);
    canvas.render(frame, artscii::core::canvas::Padding::Center);
}
