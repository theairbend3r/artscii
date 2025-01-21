mod ascii;
mod canvas;
mod img;

use crate::ascii::Ascii;
use crate::canvas::Canvas;
use crate::img::Img;

use clap::Parser;
use image::imageops::FilterType;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,

    #[arg(long)]
    width: Option<u32>,

    #[arg(long)]
    height: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let canvas = Canvas::new();
    let img = Img::new(args.path);

    println!(
        "Canvas: {}, {}, {}",
        canvas.width, canvas.height, canvas.aspect_ratio
    );
    println!("Image: {}, {}, {}", img.width, img.height, img.aspect_ratio);

    let mut preserve_aspect_ratio = true;

    let (target_width, target_height) = match (args.width, args.height) {
        (Some(width), Some(height)) => {
            preserve_aspect_ratio = false;
            (width, height)
        }
        (Some(width), None) => (width, img.height),
        (None, Some(height)) => (img.width, height),
        (None, None) => (canvas.width, canvas.height),
    };

    let processed_img = img.process_img(
        target_width,
        target_height,
        preserve_aspect_ratio,
        FilterType::Lanczos3,
    );
    println!(
        "Processed Img: {}, {}, {}",
        processed_img.width(),
        processed_img.height(),
        processed_img.width() / processed_img.height()
    );

    let ascii = Ascii::img_to_ascii(processed_img);
    println!(
        "{}, {}, {}",
        ascii.width,
        ascii.height,
        (ascii.width / ascii.height) as f32
    );

    Ascii::display(ascii);
}
