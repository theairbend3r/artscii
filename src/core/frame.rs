use std::str::FromStr;

use anyhow::{Result, bail};

use crate::core::charset::Charset;

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

#[derive(Debug)]
pub enum ColourStyle {
    Rgba,
    Rgb,
    Gray,
}
impl FromStr for ColourStyle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rgb" => Ok(ColourStyle::Rgba),
            "rgba" => Ok(ColourStyle::Rgb),
            "gray" => Ok(ColourStyle::Gray),
            _ => Err("ColourStyle can be rgb, rgba, or gray.".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    pub pixels: Vec<u8>,
    pub ascii: Vec<String>,
    pub width: u32,
    pub height: u32,
    pub colourstyle: ColourStyle,
}

impl Frame {
    pub fn new(pixels: Vec<u8>, width: u32, height: u32, colourstyle: ColourStyle) -> Self {
        Self {
            pixels,
            ascii: Vec::new(),
            width,
            height,
            colourstyle,
        }
    }

    pub fn resize(self, target_width: u32, target_height: u32) -> Result<Self> {
        if target_width == 0 || target_height == 0 {
            bail!("target_width or target_height cannot be 0.")
        }

        // aspect aware resizing
        let scale_w = target_width as f32 / self.width as f32;
        let scale_h = target_height as f32 / self.height as f32;
        let scale = scale_w.min(scale_h);

        let target_width = (self.width as f32 * scale).round() as u32 * 2;
        let target_height = (self.height as f32 * scale).round() as u32;

        let bytes_per_pixel: u32 = match self.colourstyle {
            ColourStyle::Gray => 1,
            ColourStyle::Rgb => 3,
            ColourStyle::Rgba => 4,
        };

        let mut resized_frame: Vec<u8> =
            Vec::with_capacity((target_width * target_height * bytes_per_pixel) as usize);

        let x_ratio = self.width as f32 / target_width as f32;
        let y_ratio = self.height as f32 / target_height as f32;

        for y_new in 0..target_height {
            for x_new in 0..target_width {
                let x_old = (x_new as f32 * x_ratio).floor() as u32;
                let y_old = (y_new as f32 * y_ratio).floor() as u32;

                let idx_old = ((y_old * self.width + x_old) * bytes_per_pixel) as usize;

                resized_frame
                    .extend_from_slice(&self.pixels[idx_old..idx_old + bytes_per_pixel as usize]);
            }
        }

        Ok(Self {
            pixels: resized_frame,
            ascii: self.ascii,
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
                    let [r, g, b, _a] = [chunk[0], chunk[1], chunk[2], chunk[3]];
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
            ColourStyle::Gray => {
                gray = self.pixels;
            }
        }

        Ok(Self {
            pixels: gray,
            ascii: self.ascii,
            width: self.width,
            height: self.height,
            colourstyle: ColourStyle::Gray,
        })
    }

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
