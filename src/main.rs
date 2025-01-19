use std::path::PathBuf;

use image::{imageops::FilterType, open, Luma};

const ASCII: [&str; 14] = [
    "@", "&", "#", "$", "*", "+", "|", "^", "-", ";", ":", "'", ",", ".",
];

#[derive(Debug)]
struct Canvas {
    width: usize,
    height: usize,
    aspect_ratio: f32,
}

impl Canvas {
    fn new() -> Self {
        let terminal_size = crossterm::terminal::size();

        match terminal_size {
            Ok((width, height)) => {
                let width: usize = width as usize;
                let height: usize = height as usize;
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

// struct Ascii {
//     width: usize,
//     height: usize,
// }
//
// struct Image {
//     path: PathBuf,
//     width: usize,
//     height: usize,
// }
//
// fn map_pixel_to_ascii(pixel: &Luma<u8>) -> String {
//     let Luma([luma_value]) = *pixel;
//     let px: usize = luma_value as usize;
//     let idx: usize = (px * (ASCII.len() - 1)) / 255;
//     ASCII[idx].to_string()
// }

fn main() {
    let canvas = Canvas::new();
    println!("{:?}", canvas);

    // let rgb = open("./../spaceship.png").unwrap();
    //
    // let new_height: usize = 32;
    // let new_width: usize = 32;
    // let rgb_resized = rgb.resize(
    //     new_width.try_into().unwrap(),
    //     new_height.try_into().unwrap(),
    //     FilterType::Nearest,
    // );
    // let gray = rgb_resized.into_luma8();
    // println!("{:?}", gray);
    // println!("{:?}", gray.dimensions());
    //
    // let mut ascii_img = vec![];
    // for (_, _, px) in gray.enumerate_pixels() {
    //     let ascii_val = map_pixel_to_ascii(px);
    //     ascii_img.push(ascii_val);
    // }
    //
    // for chunk in ascii_img.chunks(new_width) {
    //     println!("{:?}", chunk.join(" "));
    // }
}
