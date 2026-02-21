use image::{DynamicImage, GenericImageView};

const ASCII_CHARS: &[char] = &['@', '#', '8', '&', 'o', ':', '*', '.', ' '];

#[derive(Debug)]
pub struct Frame {
    pub data: Vec<Vec<u8>>,
}

impl Frame {
    pub fn from_image(img: DynamicImage) -> Self {
        let (width, height) = img.dimensions();

        let img = img
            .resize_exact(width, height / 2, image::imageops::FilterType::Nearest)
            .to_luma8();

        let (width, height) = img.dimensions();

        let mut arr: Vec<Vec<u8>> = vec![vec![0; width as usize]; height as usize];

        for y in 0..img.height() {
            for x in 0..img.width() {
                arr[y as usize][x as usize] = img.get_pixel(x, y).0[0];
            }
        }

        Self { data: arr }
    }

    fn pixel_to_ascii(pixel: u8) -> char {
        let ascii_idx = pixel as usize * (ASCII_CHARS.len() - 1) / 255;
        ASCII_CHARS[ascii_idx]
    }

    pub fn render(self) {
        for row in self.data {
            for col in row {
                print!("{}", Self::pixel_to_ascii(col));
                // print!("{:4?}", col);
            }
            println!()
        }
    }
}
