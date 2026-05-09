use std::str::FromStr;

#[derive(Debug)]
pub enum ColourStyle {
    Rgba,
    Rgb,
    Gray,
}

impl FromStr for ColourStyle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rgb" => Ok(ColourStyle::Rgba),
            "rgba" => Ok(ColourStyle::Rgb),
            "gray" => Ok(ColourStyle::Gray),
            _ => Err("ColourStyle can be rgb, rgba, or gray.".to_string()),
        }
    }
}
