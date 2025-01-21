#[derive(Debug)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f32,
}

impl Canvas {
    pub fn new() -> Self {
        let terminal_size = crossterm::terminal::size();

        match terminal_size {
            Ok((width, height)) => {
                let width = width as u32 - 2;
                let height = height as u32 - 2;
                let aspect_ratio = width as f32 / height as f32;

                Canvas {
                    width,
                    height,
                    aspect_ratio,
                }
            }
            Err(_) => {
                eprintln!("Creating with default height=80 and width=24.");
                Canvas {
                    width: 80,
                    height: 24,
                    aspect_ratio: (80 / 24) as f32,
                }
            }
        }
    }
}
