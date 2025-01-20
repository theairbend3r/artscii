use std::path::PathBuf;
use image::{imageops::FilterType, DynamicImage, ImageBuffer, Luma};

#[derive(Debug)]
pub struct Img {
    path: PathBuf,
    width: u32,
    height: u32,
    img: DynamicImage,
}

impl Img {
    pub fn new(path: PathBuf) -> Self {
        let img = image::open(&path).expect("Image not found.");

        Img {
            path,
            width: img.width(),
            height: img.height(),
            img,
        }
    }

    pub fn process_img(
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
