use artscii::core::canvas::{Canvas, Padding};
use artscii::core::charset::Charset;
use artscii::core::reader::gif::ReaderGif;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load image from disk into a Frame
    let path = PathBuf::from("./../test-images/countdown.gif");

    // init canvas, reader, and charset.
    let gif_iter = ReaderGif::new(path);
    let canvas = Canvas::new(210, 53);
    let charset = Charset::Braille;

    // iterate through the gif frames, process, and render them
    for frame in gif_iter {
        let frame = frame.resize(80, 40)?.to_charset(&charset)?;

        canvas.render_clear_delay(frame, Padding::Center, 20);
    }

    Ok(())
}
