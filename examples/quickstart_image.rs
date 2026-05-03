use std::path::PathBuf;

use artscii::core::{
    canvas::{Canvas, Padding},
    charset::Charset,
    reader::image::ReaderImage,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load image from disk into a Frame
    let path = PathBuf::from("./../test-images/cuddlyferris.png");

    // init canvas, reader, and charset.
    let reader = ReaderImage::new(path);
    let canvas = Canvas::new(210, 53);
    let charset = Charset::new(vec!['⠀', '⠁', '⠃', ':', 'S', '⠏', '#', '⠿', '⡿', '⣿'])?;

    // read and process frame
    let frame = reader.read()?;
    let frame = frame.resize(40, 20)?.to_charset(&charset)?;

    // render frame
    canvas.render(frame, Padding::Center);

    Ok(())
}
