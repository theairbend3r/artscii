use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crate::core::frame::Ascii;

pub struct Canvas {
    pub width: u32,
    pub height: u32,
}

pub enum Padding {
    None,
    Center,
    Custom(u32, u32),
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn calculate_center_padding(&self, frame_width: u32, frame_height: u32) -> (u32, u32) {
        let pad_left = ((self.width as i32 - frame_width as i32) / 2).max(0) as u32;
        let pad_top = ((self.height as i32 - frame_height as i32) / 2).max(0) as u32;

        (pad_left, pad_top)
    }

    pub fn render(&self, frame: Ascii, padding: Padding) {
        // calculate pad values
        let (pad_left, pad_top) = match padding {
            Padding::None => (0, 0),
            Padding::Center => self.calculate_center_padding(frame.width, frame.height),
            Padding::Custom(left, top) => (left, top),
        };

        // pad top
        for _ in 0..pad_top {
            println!();
        }

        // print on screen
        for row in 0..frame.height {
            // pad left before the start of width
            print!("{}", " ".repeat(pad_left as usize));

            // print pixels
            for col in 0..frame.width {
                let idx = (row * frame.width + col) as usize;
                let px = frame.pixels[idx];
                print!("{}", px);
            }

            // print newline at the end of width
            println!();
        }

        // pad bottom
        for _ in 0..pad_top {
            println!();
        }
    }

    pub fn render_with_delay(&self, frame: Ascii, padding: Padding, frames_per_second: u32) {
        print!("\x1b[H"); // move cursor to top-left
        io::stdout().flush().unwrap();

        self.render(frame, padding);

        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs_f32(1.0 / frames_per_second as f32));
    }
}
