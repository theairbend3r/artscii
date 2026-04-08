#![cfg(feature = "cli")]

mod utils;

use anyhow::Result;
use artscii::core::canvas::{Canvas, Padding};
use artscii::core::decoder::decoder::Decoder;
use artscii::core::decoder::image::ImageDecoder;
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

    let img_decoder = ImageDecoder::new(args.path);
    let frame = img_decoder.decode().unwrap();
    let frame = frame.resize(term_w, term_h);
    let frame = frame.to_ascii().unwrap();

    let canvas = Canvas::new(term_w, term_h);
    canvas.render(frame, Padding::Center);

    info!("Shutting down.");
    Ok(())
}
