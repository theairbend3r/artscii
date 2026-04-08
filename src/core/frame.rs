use anyhow::Result;

const ASCII_CHARS: [char; 10] = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', '.'];

pub fn brightness_to_ascii_char(brightness: u8) -> char {
    // multiply first and divide later because in rust
    // dividing two integers results in an integer which throws away the decimal.
    let ascii_idx = brightness as usize * (ASCII_CHARS.len() - 1) / 255;

    ASCII_CHARS[ascii_idx]
}

#[derive(Debug)]
pub struct Frame {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct Ascii {
    pub pixels: Vec<char>,
    pub width: u32,
    pub height: u32,
}

impl Frame {
    pub fn to_ascii(self) -> Result<Ascii> {
        let mut ascii_frame: Vec<char> = Vec::with_capacity((self.width * self.height) as usize);

        for i in 0..self.pixels.len() {
            let pixel = self.pixels[i];
            let ascii = brightness_to_ascii_char(pixel);
            ascii_frame.push(ascii);
        }

        Ok(Ascii {
            pixels: ascii_frame,
            width: self.width,
            height: self.height,
        })
    }
}
