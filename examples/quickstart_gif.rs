use artscii::core::canvas::{Canvas, Padding};
use artscii::core::reader::gif::ReaderGif;
use std::path::PathBuf;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from("./../test-images/countdown.gif");
    let gif = ReaderGif::new(path);
    let canvas = Canvas::new(210, 53);

    for f in gif {
        let f = f.resize(80, 40).to_ascii().unwrap();

        // clear screen + move cursor to top-left
        print!("\x1b[2J\x1b[H");
        canvas.render(f, Padding::Center);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
