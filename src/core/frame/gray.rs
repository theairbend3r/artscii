use anyhow::Result;

use crate::core::{colourstyle::ColourStyle, frame::Frame};

impl Frame {
    pub fn gray(self) -> Result<Self> {
        let mut gray = Vec::new();

        match self.colourstyle {
            ColourStyle::Rgba => {
                for chunk in self.pixels.chunks_exact(4) {
                    let [r, g, b, _a] = [chunk[0], chunk[1], chunk[2], chunk[3]];
                    let y = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                    gray.push(y)
                }
            }
            ColourStyle::Rgb => {
                for chunk in self.pixels.chunks_exact(3) {
                    let [r, g, b] = [chunk[0], chunk[1], chunk[2]];
                    let y = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
                    gray.push(y)
                }
            }
            ColourStyle::Gray => {
                gray = self.pixels;
            }
        }

        Ok(Self {
            pixels: gray,
            ascii: self.ascii,
            width: self.width,
            height: self.height,
            colourstyle: ColourStyle::Gray,
        })
    }
}
