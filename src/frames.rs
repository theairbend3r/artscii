use crate::frame::Frame;
use std::{thread, time::Duration};

#[derive(Debug)]
pub struct Frames {
    pub data: Vec<Frame>,
}

impl Frames {
    pub fn render(self) {
        for frame in self.data {
            frame.render();
            print!("\x1b[{}D", 28);
            print!("\x1b[{}A", 14);

            thread::sleep(Duration::from_secs(1));
        }
    }
}
