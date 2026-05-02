pub struct Charset {
    pub chars: Vec<char>,
}

pub enum DefaultCharset {
    Ascii,
    Braille,
}

impl DefaultCharset {
    pub fn chars(&self) -> Charset {
        match self {
            DefaultCharset::Ascii => Charset {
                chars: vec!['@', '#', 'S', '%', '?', '*', '+', ';', ':', '.'],
            },
            DefaultCharset::Braille => Charset {
                chars: vec!['⠀', '⠁', '⠃', '⠇', '⠏', '⠟', '⠿', '⡿', '⣿'],
            },
        }
    }
}
