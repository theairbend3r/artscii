use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let ascii_chars = "@%#*+=-:. ";

    let img = image::open(&args.path).expect("Failed to open image.");

    //  terminal characters are approx twice as high as they are wide
    let img_resized = img.resize(
        img.width(),
        img.width() / 2,
        image::imageops::FilterType::Nearest,
    );

    // let img_gray = img_resized.grayscale();
    let img_gray = img_resized.to_luma8();

    println!("Path provided: {:?}", args.path);
    println!("Ascii chars: {:?}", ascii_chars);
    println!("Loaded image: {}x{}", img.width(), img.height());
    println!(
        "Resized image: {}x{}",
        img_resized.width(),
        img_resized.height()
    );
    println!(
        "Grayscale image: {}x{}",
        img_gray.width(),
        img_gray.height()
    );

    for y in 0..img_gray.height() {
        for x in 0..img_gray.width() {
            let pixel = img_gray.get_pixel(x, y);
            let brightness = pixel[0] as usize;

            // multiply first and divide later because in rust
            // dividing two integers results in an integer which throws away the decimal.
            let ascii_idx = brightness * (ascii_chars.len() - 1) / 255;
            let ascii_char = ascii_chars.chars().nth(ascii_idx).unwrap();

            print!("{}", ascii_char);
        }
        println!();
    }
}
