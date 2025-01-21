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
    // init structs
    let args = Args::parse();
    let canvas = Canvas::new();
    let img = Img::new(args.path);

    // calculate target size for the ascii art
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

    // resize image and convert to grayscale
    let processed_img = img.process_img(
        target_width,
        target_height,
        preserve_aspect_ratio,
        FilterType::Lanczos3,
    );

    // convert image pixels to ascii chars
    let ascii = Ascii::img_to_ascii(processed_img);
    Ascii::display(ascii);
}
