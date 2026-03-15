mod frame;
mod utils;

use frame::Frame;

use clap::Parser;
use std::{error::Error, path::PathBuf};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let frame = Frame::from_path(&args.path)?;
    let artscii = frame.to_ascii();
    for c in artscii {
        print!("{}", c)
    }

    Ok(())
}
