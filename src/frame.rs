use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

use crate::utils::get_terminal_size;

const ASCII_CHARS: [char; 10] = ['@', '#', 'S', '%', '?', '*', '+', ';', ':', '.'];

pub struct Frame {
    image: DynamicImage,
    width: u32,
    height: u32,
}

impl Frame {
    pub fn from_path(path: &PathBuf) -> Self {
        let img = Self::load(path);
        let img = Self::resize(img);
        let img = Self::colorise(img);

        let (width, height) = img.dimensions();

        Self {
            image: img,
            width,
            height,
        }
    }

    fn load(path: &PathBuf) -> DynamicImage {
        image::open(path).expect("Failed to open image.")
    }
    fn resize(img: DynamicImage) -> DynamicImage {
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
        img.grayscale()
    }

    pub fn to_ascii(&self) -> Vec<char> {
        // store the ascii image in a single list periodically
        // separated by newlines.
        let (ascii_w, ascii_h) = (self.width, self.height);
        let (term_w, term_h) = get_terminal_size();

        let pad_top = ((term_h as i32 - ascii_h as i32) / 2).max(0) as u32;
        let pad_left = ((term_w as i32 - ascii_w as i32) / 2).max(0) as u32;

        // include padding in new width on one side
        let new_width = self.width + pad_left;

        // init finall ascii 2d matrix as a 1d vector
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

                // multiply first and divide later because in rust
                // dividing two integers results in an integer which throws away the decimal.
                let ascii_idx = brightness * (ASCII_CHARS.len() - 1) / 255;
                let ascii_char = ASCII_CHARS[ascii_idx];

                ascii_image.push(ascii_char);
            }
            ascii_image.push('\n');
        }
        ascii_image
    }
}
