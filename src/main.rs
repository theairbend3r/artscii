use std::{
    fs::read_dir,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};

use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageReader};

const ASCII_CHARS: &[char] = &['@', '#', '8', '&', 'o', ':', '*', '.', ' '];

#[derive(Debug)]
struct Frame {
    data: Vec<Vec<u8>>,
}

#[derive(Debug)]
struct Frames {
    data: Vec<Frame>,
}

fn read_image(file_path: &PathBuf) -> DynamicImage {
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

fn is_image_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => ["png", "jpg", "jpeg"].contains(&ext.to_str().unwrap()),
        None => false,
    }
}

/// path to image to display or directory of images to animate"
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, help = "path to file or dir.")]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    if args.path.is_dir() {
        let paths = read_dir(args.path).unwrap();
        let mut image_files: Vec<PathBuf> = vec![];

        for path in paths {
            image_files.push(path.unwrap().path());
        }
        let mut frames = Frames { data: vec![] };

        for fp in image_files {
            let img = read_image(&fp);
            let frame = Frame::from_image(img);
            frames.data.push(frame)
        }

        frames.render();
    } else if args.path.is_file() {
        if is_image_file(&args.path) {
            let img = read_image(&args.path);
            let frame = Frame::from_image(img);
            frame.render();
        } else {
            println!("Image file not supported.")
        }
    } else {
        println!("Could not find path.");
    }
}
