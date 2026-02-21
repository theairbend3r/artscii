use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

pub struct Frame {
    image: DynamicImage,
    width: u32,
    height: u32,
    ascii_chars: String,
}

impl Frame {
    pub fn from_path(path: &PathBuf) -> Self {
        let img = image::open(path).expect("Failed to open image.");
        let ascii_chars = "@%#*+=-:. ".to_string();

        //  terminal characters are approx twice as high as they are wide
        let img_resized = img.resize(
            img.width(),
            img.width() / 2,
            image::imageops::FilterType::Nearest,
        );

        let img_gray = img_resized.grayscale();
        let (width, height) = img_gray.dimensions();
        // let img_gray = img_resized.to_luma8();

        Self {
            image: img_gray,
            width,
            height,
            ascii_chars,
        }
    }

    pub fn render(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.image.get_pixel(x, y);
                let brightness = pixel[0] as usize;

                // multiply first and divide later because in rust
                // dividing two integers results in an integer which throws away the decimal.
                let ascii_idx = brightness * (self.ascii_chars.len() - 1) / 255;
                let ascii_char = self.ascii_chars.chars().nth(ascii_idx).unwrap();

                print!("{}", ascii_char);
            }
            println!();
        }
    }
}
