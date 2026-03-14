mod frame;
mod utils;

use frame::Frame;

use clap::Parser;
use std::path::PathBuf;

use crate::utils::get_terminal_size;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("Path provided: {:?}", args.path);

    let frame = Frame::from_path(&args.path);
    let artscii = frame.to_ascii();
    for c in artscii {
        print!("{}", c)
    }
}
