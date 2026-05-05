use anyhow::{Result, bail};

use crate::core::charset::Charset;

pub fn brightness_to_ascii_char(brightness: u8, charset: &Charset) -> Result<char> {
    let charset = charset.chars()?;

    // multiply first and divide later because in rust
    // dividing two integers results in an integer which throws away the decimal.
    let ascii_idx = brightness as usize * (charset.len() - 1) / 255;

    Ok(charset[ascii_idx])
}

#[derive(Debug)]
pub enum ColourStyle {
    Rgba,
    Rgb,
    Gray,
}

#[derive(Debug)]
pub struct Frame {
    pub pixels: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub colourstyle: ColourStyle,
}

#[derive(Debug)]
pub struct Ascii {
    pub pixels: Vec<char>,
    pub width: u32,
    pub height: u32,
}

impl Frame {
    pub fn new(pixels: Vec<u8>, width: u32, height: u32, colourstyle: ColourStyle) -> Self {
        Self {
            pixels,
            width,
            height,
            colourstyle,
        }
    }

    pub fn resize(self, target_width: u32, target_height: u32) -> Result<Self> {
        if target_width == 0 || target_height == 0 {
            bail!("target_width or target_height cannot be 0.")
        }

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

        Ok(Self {
            pixels: resized_frame,
            width: target_width,
            height: target_height,
            colourstyle: self.colourstyle,
        })
    }

    pub fn gray(self) -> Result<Self> {
        let mut gray = Vec::new();

        match self.colourstyle {
            ColourStyle::Rgba => {
                for chunk in self.pixels.chunks_exact(4) {
                    let [r, g, b] = [chunk[0], chunk[1], chunk[2]];
                    let y = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                    gray.push(y)
                }
            }
            ColourStyle::Rgb => {
                for chunk in self.pixels.chunks_exact(3) {
                    let [r, g, b] = [chunk[0], chunk[1], chunk[2]];
                    let y = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                    gray.push(y)
                }
            }
            _ => {
                gray = self.pixels;
            }
        }

        let frame = Frame::new(gray, self.width, self.height, ColourStyle::Gray);

        Ok(frame)
    }

    pub fn to_charset(self, charset: &Charset) -> Result<Ascii> {
        let mut ascii_frame: Vec<char> = Vec::with_capacity((self.width * self.height) as usize);

        for i in 0..self.pixels.len() {
            let pixel = self.pixels[i];
            let ascii = brightness_to_ascii_char(pixel, charset)?;
            ascii_frame.push(ascii);
        }

        Ok(Ascii {
            pixels: ascii_frame,
            width: self.width,
            height: self.height,
        })
    }
}
