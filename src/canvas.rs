#[derive(Debug)]
pub struct Canvas {
    width: u32,
    height: u32,
    aspect_ratio: f32,
}

impl Canvas {
    pub fn new() -> Self {
        let terminal_size = crossterm::terminal::size();

        match terminal_size {
            Ok((width, height)) => {
                let width = width as u32;
                let height = height as u32;
                let aspect_ratio: f32 = width as f32 / height as f32;

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
