use image::{imageops::FilterType, DynamicImage, ImageBuffer, Luma};
use std::path::PathBuf;

#[derive(Debug)]
enum MediaType {
    Image,
    Video,
}

#[derive(Debug)]
enum MediaColour {
    Gray,
    Colour,
}

#[derive(Debug)]
pub struct Media {
    path: PathBuf,
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
    media_type: MediaType,
    media_colour: MediaColour,
    frames: Vec<DynamicImage>, // vector will have only 1 item if it's an image
}

impl Media {
    pub fn new(path: PathBuf) -> Self {
        let img = image::open(&path).expect("Media not found.");

        Media {
            path,
            width: img.width(),
            height: img.height(),
            aspect_ratio: img.width() as f32 / img.height() as f32,
            media_type: MediaType::Image,
            media_colour: MediaColour::Colour,
            frames: vec![img],
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
            self.frames[0]
                .resize(new_width, new_height, resize_filter)
                .into_luma8()
        } else {
            self.frames[0]
                .resize_exact(new_width, new_height, resize_filter)
                .into_luma8()
        }
    }
}
