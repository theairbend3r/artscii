use artscii::core::frame::Frame;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./../test-images/cuddlyferris.png");

    let frame = Frame::from_path(&path).unwrap();
    println!("{}x{}", frame.width, frame.height);
    let resized_frame = frame.resize(64, 64).unwrap();
    println!("{}x{}", resized_frame.width, resized_frame.height);
    let colourised_frame = resized_frame.colorise().unwrap();
    println!("{}x{}", colourised_frame.width, colourised_frame.height);

    let ascii = colourised_frame.to_ascii().unwrap();
    Frame::render(ascii);
}
