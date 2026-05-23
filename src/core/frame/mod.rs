use crate::core::colourstyle::ColourStyle;

pub mod ascii;
pub mod gray;
pub mod resize;

#[derive(Debug)]
pub struct Frame {
    pub pixels: Vec<u8>,
    pub ascii: Vec<String>,
    pub width: u32,
    pub height: u32,
    pub colourstyle: ColourStyle,
}
impl Frame {
    pub fn new(pixels: Vec<u8>, width: u32, height: u32, colourstyle: ColourStyle) -> Self {
        Self {
            pixels,
            ascii: Vec::new(),
            width,
            height,
            colourstyle,
        }
    }
}
