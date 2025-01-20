use image::{ImageBuffer, Luma};

const ASCII: [&str; 14] = [
    "@", "&", "#", "$", "*", "+", "|", "^", "-", ";", ":", "'", ",", ".",
];

pub struct Ascii {
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

    pub fn img_to_ascii(img: ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<String> {
        let mut art = vec![];
        for (_, _, px) in img.enumerate_pixels() {
            let ascii_char = Ascii::pixel_to_ascii(px);
            art.push(ascii_char);
        }

        art
    }

    pub fn display(ascii_art: &[String]) {
        for chunk in ascii_art.chunks(16) {
            println!("{:?}", chunk.join(" "));
        }
    }
}
