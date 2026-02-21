mod frame;
mod frames;

use crate::frame::Frame;
use crate::frames::Frames;

use std::{
    fs::read_dir,
    path::{Path, PathBuf},
};

use clap::Parser;
use image::{DynamicImage, ImageReader};

fn read_image(file_path: &PathBuf) -> DynamicImage {
    let img_reader = ImageReader::open(file_path).expect("No file found.");
    img_reader.decode().expect("Could not open image.")
}
fn is_image_file(path: &Path) -> bool {
    match path.extension() {
        Some(ext) => ["png", "jpg", "jpeg"].contains(&ext.to_str().unwrap()),
        None => false,
    }
}

/// path to image to display or directory of images to animate
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
