pub enum Charset {
    Ascii,
    Braille,
    Custom(Vec<char>),
}

impl Charset {
    pub fn chars(&self) -> &[char] {
        match self {
            Charset::Ascii => &['@', '#', 'S', '%', '?', '*', '+', ';', ':', '.'],
            Charset::Braille => &['⠀', '⠁', '⠃', '⠇', '⠏', '⠟', '⠿', '⡿', '⣿'],
            Charset::Custom(custom_chars) => custom_chars,
        }
    }
}
