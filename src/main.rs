mod frame;
mod utils;

use anyhow::Result;
use frame::Frame;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let frame = Frame::from_path(&args.path)?;

    frame.render();

    Ok(())
}
