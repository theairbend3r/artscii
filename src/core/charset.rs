use anyhow::{Result, bail};

pub enum Charset {
    Ascii,
    Braille,
    Custom(Vec<char>),
}

impl Charset {
    pub fn new(chars: Vec<char>) -> Result<Self> {
        if chars.is_empty() {
            bail!("Custom charset cannot be empty.")
        } else {
            Ok(Self::Custom(chars))
        }
    }
    pub fn chars(&self) -> Result<&[char]> {
        match self {
            Self::Ascii => Ok(&['@', '#', 'S', '%', '?', '*', '+', ';', ':', '.']),
            Self::Braille => Ok(&['⠀', '⠁', '⠃', '⠇', '⠏', '⠟', '⠿', '⡿', '⣿']),
            Self::Custom(custom_chars) => Ok(custom_chars),
            // Charset::Custom(custom_chars) => {
            //     if custom_chars.is_empty() {
            //         bail!("Custom charset cannot be empty.")
            //     } else {
            //         Ok(custom_chars)
            //     }
            // }
        }
    }
}
