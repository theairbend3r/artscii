use std::path::PathBuf;

use artscii::core::{
    canvas::{Canvas, Padding},
    reader::image::ReaderImage,
};

fn main() {
    // load image from disk into a Frame
    let path = PathBuf::from("./../test-images/cuddlyferris.png");
    let img_decoder = ReaderImage::new(path);
    let frame = img_decoder.decode().unwrap();
    let frame = frame.resize(40, 20).to_ascii().unwrap();

    // optionally load canvas to print
    let canvas = Canvas::new(210, 53);
    canvas.render(frame, Padding::Center);
}
