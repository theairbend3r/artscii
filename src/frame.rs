use anyhow::{Context, Result};
use image::{DynamicImage, GenericImageView};
use log::info;
use std::path::PathBuf;

use crate::utils::get_terminal_size;

const ASCII_CHARS: [char; 10] = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', '.'];

pub struct Frame {
    image: DynamicImage,
    width: u32,
    height: u32,
}

impl Frame {
    #[cfg(test)]
    fn from_image(img: DynamicImage) -> Self {
        let (width, height) = img.dimensions();
        Self {
            image: img,
            width,
            height,
        }
    }

    pub fn from_path(path: &PathBuf) -> Result<Self> {
        let img = Self::load(path)?;

        let img = Self::resize(img);
        let img = Self::colorise(img);

        let (width, height) = img.dimensions();

        Ok(Self {
            image: img,
            width,
            height,
        })
    }

    fn load(path: &PathBuf) -> Result<DynamicImage> {
        info!("Loading image from disk as a frame.");
        image::open(path).with_context(|| format!("Failed to read file: `{}`", path.display()))
    }

    fn resize(img: DynamicImage) -> DynamicImage {
        info!("Resizing frame.");
        let (img_w, img_h) = img.dimensions();
        let (term_w, term_h) = get_terminal_size();

        // terminal characters are approx twice as high as they are wide
        // original aspect => w/h = a
        // we want aspect => w/(h/2) = 2 * (w/h)
        let char_aspect_ratio = 2.0;
        let scale_w = term_w as f32 / img_w as f32;
        let scale_h = term_h as f32 / (img_h as f32 / char_aspect_ratio);

        // find which direction to expand in to prevent image from getting cut-off
        let scale = scale_w.min(scale_h);

        let new_img_w = (img_w as f32 * scale).round() as u32;
        let new_img_h = (img_h as f32 / char_aspect_ratio * scale).round() as u32;

        img.resize_exact(new_img_w, new_img_h, image::imageops::FilterType::Nearest)
    }
    fn colorise(img: DynamicImage) -> DynamicImage {
        info!("Converting image to grayscale (luma8).");
        img.grayscale()
    }

    fn brightness_to_ascii_char(brightness: usize) -> char {
        // multiply first and divide later because in rust
        // dividing two integers results in an integer which throws away the decimal.
        let ascii_idx = brightness * (ASCII_CHARS.len() - 1) / 255;
        ASCII_CHARS[ascii_idx]
    }

    fn calculate_padding(ascii_width: u32, ascii_height: u32) -> (u32, u32) {
        let (term_w, term_h) = get_terminal_size();

        let pad_left = ((term_w as i32 - ascii_width as i32) / 2).max(0) as u32;
        let pad_top = ((term_h as i32 - ascii_height as i32) / 2).max(0) as u32;

        (pad_left, pad_top)
    }

    pub fn to_ascii(&self) -> Vec<char> {
        info!("Converting frame to a vector of ascii chars.");
        // store the ascii image in a single list periodically
        // separated by newlines.
        let (ascii_w, ascii_h) = (self.width, self.height);

        // calculate required padding based on terminal size
        let (pad_left, pad_top) = Self::calculate_padding(ascii_w, ascii_h);

        // init finall ascii 2d matrix as a 1d vector
        // include padding in new width on one side
        let new_width = self.width + pad_left;
        let mut ascii_image: Vec<char> =
            Vec::with_capacity((new_width * self.height) as usize + self.height as usize);

        // pad top
        for _ in 0..pad_top {
            println!();
        }

        for y in 0..self.height {
            // fill with padding
            ascii_image.extend(std::iter::repeat_n(' ', pad_left as usize));

            // fill with ascii chars
            for x in 0..self.width {
                let pixel = self.image.get_pixel(x, y);
                let brightness = pixel[0] as usize;

                let ascii_char = Self::brightness_to_ascii_char(brightness);

                ascii_image.push(ascii_char);
            }
            ascii_image.push('\n');
        }
        ascii_image
    }

    pub fn render(&self) {
        let artscii = self.to_ascii();
        info!("Printing the computed vector of ascii characters.");
        for c in artscii {
            print!("{}", c)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use image::{ImageBuffer, Luma};

    fn create_test_img(width: u32, height: u32, default_pixel: u8) -> DynamicImage {
        let img = ImageBuffer::from_pixel(width, height, Luma([default_pixel]));
        DynamicImage::ImageLuma8(img)
    }

    #[test]
    fn convert_img_to_frame() {
        let img = create_test_img(8, 8, 0);
        let frame = Frame::from_image(img);
        let (w, h) = (frame.width, frame.height);

        // frame has correct dimensions
        assert_eq!(w, 8);
        assert_eq!(h, 8);
    }

    #[test]
    fn check_ascii_height() {
        let img = create_test_img(8, 8, 0);
        let frame = Frame::from_image(img);
        let ascii = frame.to_ascii();
    }
}
