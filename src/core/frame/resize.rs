use anyhow::{Result, bail};

use crate::core::{colourstyle::ColourStyle, frame::Frame};

impl Frame {
    pub fn resize(self, target_width: u32, target_height: u32) -> Result<Self> {
        if target_width == 0 || target_height == 0 {
            bail!("target_width or target_height cannot be 0.")
        }

        // aspect aware resizing
        let scale_w = target_width as f32 / self.width as f32;
        let scale_h = target_height as f32 / self.height as f32;
        let scale = scale_w.min(scale_h);

        let target_width = (self.width as f32 * scale).round() as u32 * 2;
        let target_height = (self.height as f32 * scale).round() as u32;

        let bytes_per_pixel: u32 = match self.colourstyle {
            ColourStyle::Gray => 1,
            ColourStyle::Rgb => 3,
            ColourStyle::Rgba => 4,
        };

        let mut resized_frame: Vec<u8> =
            Vec::with_capacity((target_width * target_height * bytes_per_pixel) as usize);

        let x_ratio = self.width as f32 / target_width as f32;
        let y_ratio = self.height as f32 / target_height as f32;

        for y_new in 0..target_height {
            for x_new in 0..target_width {
                let x_old = (x_new as f32 * x_ratio).floor() as u32;
                let y_old = (y_new as f32 * y_ratio).floor() as u32;

                let idx_old = ((y_old * self.width + x_old) * bytes_per_pixel) as usize;

                resized_frame
                    .extend_from_slice(&self.pixels[idx_old..idx_old + bytes_per_pixel as usize]);
            }
        }

        Ok(Self {
            pixels: resized_frame,
            ascii: self.ascii,
            width: target_width,
            height: target_height,
            colourstyle: self.colourstyle,
        })
    }
}
