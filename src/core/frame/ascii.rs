use anyhow::Result;

use crate::core::{charset::Charset, colourstyle::ColourStyle, frame::Frame};

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

pub fn brightness_to_ascii_char(brightness: u8, charset: &Charset) -> Result<char> {
    let charset = charset.chars()?;

    // multiply first and divide later because in rust
    // dividing two integers results in an integer which throws away the decimal.
    let ascii_idx = brightness as usize * (charset.len() - 1) / 255;

    Ok(charset[ascii_idx])
}

impl Frame {
    pub fn create_ascii(self, charset: &Charset) -> Result<Frame> {
        let mut ascii_frame: Vec<String> = Vec::with_capacity((self.width * self.height) as usize);

        match self.colourstyle {
            ColourStyle::Rgba => {
                for chunk in self.pixels.chunks_exact(4) {
                    let [r, g, b, _a] = [chunk[0], chunk[1], chunk[2], chunk[3]];

                    let clr = rgb_to_ansi(r, g, b, 128);
                    let px = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                    let ascii = brightness_to_ascii_char(px, charset)?;
                    let clr_ascii = format!("\x1b[{}m{}\x1b[0m", clr, ascii);

                    ascii_frame.push(clr_ascii);
                }
            }
            ColourStyle::Rgb => {
                for chunk in self.pixels.chunks_exact(3) {
                    let [r, g, b] = [chunk[0], chunk[1], chunk[2]];

                    let clr = rgb_to_ansi(r, g, b, 128);
                    let px = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                    let ascii = brightness_to_ascii_char(px, charset)?;
                    let clr_ascii = format!("\x1b[{}m{}\x1b[0m", clr, ascii);

                    ascii_frame.push(clr_ascii);
                }
            }
            ColourStyle::Gray => {
                for i in 0..self.pixels.len() {
                    let pixel = self.pixels[i];
                    let ascii = brightness_to_ascii_char(pixel, charset)?;

                    ascii_frame.push(ascii.into());
                }
            }
        }

        Ok(Frame {
            pixels: self.pixels,
            ascii: ascii_frame,
            width: self.width,
            height: self.height,
            colourstyle: self.colourstyle,
        })
    }
}
