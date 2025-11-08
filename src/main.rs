use image::{DynamicImage, GenericImageView, ImageReader};
use std::{thread, time::Duration};

const ASCII_CHARS: &[char] = &['@', '#', '8', '&', 'o', ':', '*', '.', ' '];

#[derive(Debug)]
struct Frame {
    data: Vec<Vec<u8>>,
}

#[derive(Debug)]
struct Frames {
    data: Vec<Frame>,
}

fn read_image(file_path: &str) -> DynamicImage {
    let img_reader = ImageReader::open(file_path).expect("No file found.");
    img_reader.decode().expect("Could not open image.")
}

impl Frame {
    fn from_image(img: DynamicImage) -> Self {
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

    fn render(self) {
        for row in self.data {
            for col in row {
                print!("{}", Self::pixel_to_ascii(col));
                // print!("{:4?}", col);
            }
            println!()
        }
    }
}

impl Frames {
    fn render(self) {
        for frame in self.data {
            frame.render();
            print!("\x1b[{}D", 28);
            print!("\x1b[{}A", 14);

            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn main() {
    let file_paths = vec![
        "./../1.png",
        "./../10006.png",
        "./../10009.png",
        "./../10002.png",
    ];

    let mut frames = Frames { data: vec![] };

    for fp in file_paths {
        let img = read_image(fp);
        let frame = Frame::from_image(img);
        frames.data.push(frame)
    }

    frames.render();
}
