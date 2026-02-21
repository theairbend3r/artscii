mod frame;

use frame::Frame;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("Path provided: {:?}", args.path);

    let frame = Frame::from_path(&args.path);
    frame.render();
}
