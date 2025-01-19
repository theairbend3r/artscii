use image::{imageops::FilterType, open, Luma};

const ASCII: [&str; 14] = [
    "@", "&", "#", "$", "*", "+", "|", "^", "-", ";", ":", "'", ",", ".",
];

fn map_pixel_to_ascii(pixel: &Luma<u8>) -> String {
    let Luma([luma_value]) = *pixel;
    let px: usize = luma_value as usize;
    let idx: usize = (px * (ASCII.len() - 1)) / 255;
    ASCII[idx].to_string()
}

fn main() {
    let rgb = open("./../spaceship.png").unwrap();

    let new_height: usize = 32;
    let new_width: usize = 32;
    let rgb_resized = rgb.resize(
        new_width.try_into().unwrap(),
        new_height.try_into().unwrap(),
        FilterType::Nearest,
    );
    let gray = rgb_resized.into_luma8();
    println!("{:?}", gray);
    println!("{:?}", gray.dimensions());

    let mut ascii_img = vec![];
    for (_, _, px) in gray.enumerate_pixels() {
        let ascii_val = map_pixel_to_ascii(px);
        ascii_img.push(ascii_val);
    }

    for chunk in ascii_img.chunks(new_width) {
        println!("{:?}", chunk.join(" "));
    }
}
