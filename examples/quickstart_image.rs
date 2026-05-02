use std::path::PathBuf;

use artscii::core::{
    canvas::{Canvas, Padding},
    charset::DefaultCharset,
    reader::image::ReaderImage,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load image from disk into a Frame
    let path = PathBuf::from("./../test-images/cuddlyferris.png");
    let reader = ReaderImage::new(path);
    let canvas = Canvas::new(210, 53);

    let frame = reader.read()?;

    // let charset = Charset {
    //     chars: vec!['@', '#', 'S', '%', '?', '*', '+', ';', ':', '.'],
    // };
    let charset = DefaultCharset::Braille.chars();
    let frame = frame.resize(40, 20).to_ascii(&charset)?;

    canvas.render(frame, Padding::Center);

    Ok(())
}
