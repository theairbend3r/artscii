use image::{imageops::FilterType, DynamicImage, ImageBuffer, Luma};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Img {
    path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    img: DynamicImage,
}

impl Img {
    pub fn new(path: PathBuf) -> Self {
        let img = image::open(&path).expect("Image not found.");
        let aspect_ratio = img.width() as f32 / img.height() as f32;

        Img {
            path,
            width: img.width(),
            height: img.height(),
            aspect_ratio,
            img,
        }
    }

    pub fn process_img(
        &self,
        new_width: u32,
        new_height: u32,
        preserve_aspect_ratio: bool,
        resize_filter: FilterType,
    ) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        if preserve_aspect_ratio {
            self.img
                .resize(new_width, new_height, resize_filter)
                .into_luma8()
        } else {
            self.img
                .resize_exact(new_width, new_height, resize_filter)
                .into_luma8()
        }
    }
}
