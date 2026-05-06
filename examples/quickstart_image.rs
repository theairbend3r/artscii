use std::path::PathBuf;

use artscii::core::{
    canvas::{Canvas, Padding},
    charset::Charset,
    reader::image::ReaderImage,
};

fn rgb_to_ansi(r: u8, g: u8, b: u8, threshold: u8) -> u8 {
    let r_bit = if r > threshold { 1 } else { 0 };
    let g_bit = if g > threshold { 1 } else { 0 };
    let b_bit = if b > threshold { 1 } else { 0 };

    let rgb_value = (r_bit << 2) | (g_bit << 1) | b_bit;

    let ansi_index = match rgb_value {
        0 => 0, // black
        1 => 4, // blue
        2 => 2, // green
        3 => 6, // cyan
        4 => 1, // red
        5 => 5, // magenta
        6 => 3, // yellow
        7 => 7, // white
        _ => 0,
    };

    30 + ansi_index
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load image from disk into a Frame
    let path = PathBuf::from("./../test-images/cuddlyferris.png");

    // init canvas, reader, and charset.
    let reader = ReaderImage::new(path);
    let canvas = Canvas::new(210, 53);
    let charset = Charset::new(vec!['⠀', '⠁', '⠃', ':', 'S', '⠏', '#', '⠿', '⡿', '⣿'])?;

    // read and process frame
    let frame = reader.read()?;
    let frame = frame.resize(40, 20)?.create_ascii(&charset)?;

    // render frame
    canvas.render(frame, Padding::Center);

    Ok(())
}
