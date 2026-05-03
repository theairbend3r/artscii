use std::str::FromStr;

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
        }
    }
}

impl FromStr for Charset {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ascii" => Ok(Charset::Ascii),
            "braille" => Ok(Charset::Braille),
            _ => {
                if s.is_empty() {
                    Err("Custom charset string cannot be empty.".to_string())
                } else {
                    let custom_chars = s.chars().collect::<Vec<_>>();
                    match Charset::new(custom_chars) {
                        Ok(c) => Ok(c),
                        Err(e) => Err(e.to_string()),
                    }
                }
            }
        }
    }
}
