#![cfg(feature = "cli")]

mod utils;

use anyhow::Result;
use artscii::core::frame::Frame;
use clap_verbosity_flag::Verbosity;

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

    let (term_w, term_h) = utils::get_terminal_size();
    let frame = Frame::from_path(&args.path)?
        .resize(term_w, term_h)?
        .colorise()?;
    let ascii = frame.to_ascii().unwrap();
    Frame::render(ascii);

    info!("Shutting down.");
    Ok(())
}
