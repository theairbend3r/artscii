use image::{ImageBuffer, Luma};

const ASCII: [&str; 14] = [
    "@", "&", "#", "$", "*", "+", "|", "^", "-", ";", ":", "'", ",", ".",
];

pub struct Ascii {
    pub width: u32,
    pub height: u32,
    pub art: Vec<Vec<String>>,
}

impl Ascii {
    fn pixel_to_ascii(px: &Luma<u8>) -> String {
        let Luma([luma_value]) = *px;
        let px: usize = luma_value.into();
        let idx: usize = (px * (ASCII.len() - 1)) / 255;
        ASCII[idx].to_string()
    }

    pub fn img_to_ascii(img: ImageBuffer<Luma<u8>, Vec<u8>>) -> Ascii {
        let width = img.width() as usize;
        let height = img.height() as usize;
        let mut art = vec![vec![" ".to_string(); width]; height];

        for (y, x, p) in img.enumerate_pixels() {
            let ascii_char = Ascii::pixel_to_ascii(p);
            art[x as usize][y as usize] = ascii_char;
        }

        Ascii {
            width: img.width(),
            height: img.height(),
            art,
        }
    }

    pub fn display(ascii: Ascii) {
        for row in ascii.art {
            println!("{}", row.join(" "));
        }
    }
}
