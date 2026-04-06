#![cfg(feature = "cli")]

mod frame;
mod utils;

use anyhow::Result;
use clap_verbosity_flag::Verbosity;
use frame::Frame;

use clap::Parser;
use log::info;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
    #[command(flatten)]
    verbose: Verbosity,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // init logger
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("Starting up.");

    let frame = Frame::from_path(&args.path)?;

    frame.render();

    info!("Shutting down.");
    Ok(())
}
