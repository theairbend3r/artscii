#![cfg(feature = "cli")]

mod utils;

use anyhow::Result;
use artscii::core::canvas::{Canvas, Padding};
use artscii::core::charset::Charset;
use artscii::core::reader::gif::ReaderGif;
use artscii::core::reader::image::ReaderImage;
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
    let canvas = Canvas::new(term_w, term_h);

    let file_extension = args.path.extension().and_then(|e| e.to_str());

    match file_extension {
        Some("gif") => {
            let charset = Charset::Braille;
            let gif_iter = ReaderGif::new(args.path);

            for frame in gif_iter {
                let frame = frame.resize(term_w, term_h)?.to_charset(&charset)?;

                canvas.render_clear_delay(frame, Padding::Center, 20);
            }
        }
        Some("png") | Some("jpg") | Some("jpeg") => {
            let charset = Charset::Ascii;
            let img = ReaderImage::new(args.path).read()?;

            let frame = img.resize(term_w, term_h)?.to_charset(&charset)?;

            canvas.render(frame, Padding::Center);
        }
        Some(_) => {
            println!("Unsupported file type.");
        }
        None => {
            println!("Invalid file type: No extension found.");
        }
    }

    info!("Shutting down.");
    Ok(())
}
