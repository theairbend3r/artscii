use clap::Parser;
use image::{imageops::FilterType, DynamicImage, ImageBuffer, Luma};
use std::path::PathBuf;

const ASCII: [&str; 14] = [
    "@", "&", "#", "$", "*", "+", "|", "^", "-", ";", ":", "'", ",", ".",
];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

#[derive(Debug)]
struct Canvas {
    width: u32,
    height: u32,
    aspect_ratio: f32,
}

impl Canvas {
    fn new() -> Self {
        let terminal_size = crossterm::terminal::size();

        match terminal_size {
            Ok((width, height)) => {
                let width = width as u32;
                let height = height as u32;
                let aspect_ratio: f32 = width as f32 / height as f32;

                Canvas {
                    width,
                    height,
                    aspect_ratio,
                }
            }
            Err(_) => {
                eprintln!("Creating with default height=80 and width=24.");
                Canvas {
                    width: 80,
                    height: 24,
                    aspect_ratio: (80 / 24) as f32,
                }
            }
        }
    }
}

#[derive(Debug)]
struct Image {
    path: PathBuf,
    width: u32,
    height: u32,
    img: DynamicImage,
}

impl Image {
    fn new(path: PathBuf) -> Self {
        let img = image::open(&path).expect("Image not found.");

        Image {
            path,
            width: img.width(),
            height: img.height(),
            img,
        }
    }

    fn process_img(
        &self,
        new_width: u32,
        new_height: u32,
        resize_filter: FilterType,
    ) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        self.img
            .resize(new_width, new_height, resize_filter)
            .into_luma8()
    }
}

struct Ascii {
    width: usize,
    height: usize,
    art: Vec<String>,
}

impl Ascii {
    fn pixel_to_ascii(px: &Luma<u8>) -> String {
        let Luma([luma_value]) = *px;
        let px: usize = luma_value.into();
        let idx: usize = (px * (ASCII.len() - 1)) / 255;
        ASCII[idx].to_string()
    }

    fn img_to_ascii(img: ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<String> {
        let mut art = vec![];
        for (_, _, px) in img.enumerate_pixels() {
            let ascii_char = Ascii::pixel_to_ascii(px);
            art.push(ascii_char);
        }

        art
    }

    fn display(ascii_art: &[String]) {
        for chunk in ascii_art.chunks(16) {
            println!("{:?}", chunk.join(" "));
        }
    }
}

fn main() {
    let args = Args::parse();

    let canvas = Canvas::new();
    println!("{:?}", canvas);

    let img = Image::new(args.path);
    let processed_img = img.process_img(16, 16, FilterType::Nearest);
    println!("{:?}", processed_img);
    println!("{:?}", processed_img.height());
    println!("{:?}", processed_img.width());

    let art = Ascii::img_to_ascii(processed_img);
    println!("{:?}", art);

    Ascii::display(&art);
}
