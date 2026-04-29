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
    pub fn new(pixels: Vec<u8>, width: u32, height: u32) -> Self {
        Self {
            pixels,
            width,
            height,
        }
    }

    pub fn resize(self, target_width: u32, target_height: u32) -> Self {
        let mut resized_frame: Vec<u8> =
            Vec::with_capacity((target_width * target_height) as usize);

        let x_ratio = self.width as f32 / target_width as f32;
        let y_ratio = self.height as f32 / target_height as f32;

        for y_new in 0..target_height {
            for x_new in 0..target_width {
                let x_old = (x_new as f32 * x_ratio).floor() as u32;
                let y_old = (y_new as f32 * y_ratio).floor() as u32;

                let idx_old = (y_old * self.width + x_old) as usize;

                resized_frame.push(self.pixels[idx_old]);
            }
        }

        Self {
            pixels: resized_frame,
            width: target_width,
            height: target_height,
        }
    }

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
