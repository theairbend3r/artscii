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
}

fn main() {
    let args = Args::parse();

    let canvas = Canvas::new();
    println!("{:?}", canvas);

    let img = Img::new(args.path);
    let processed_img = img.process_img(16, 16, FilterType::Nearest);
    println!("{:?}", processed_img);

    let art = Ascii::img_to_ascii(processed_img);
    println!("{:?}", art);

    Ascii::display(&art);
}
