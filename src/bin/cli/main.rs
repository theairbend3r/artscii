#![cfg(feature = "cli")]

mod utils;

use anyhow::Result;
use artscii::core::canvas::{Canvas, Padding};
use artscii::core::decoder::decoder::Decoder;
use artscii::core::decoder::gif::DecoderGif;
use artscii::core::decoder::image::ImageDecoder;
use clap_verbosity_flag::Verbosity;

use clap::Parser;
use log::info;
use std::path::PathBuf;

use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

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
            let gif = DecoderGif::new(args.path);

            for f in gif {
                let f = f.resize(term_w, term_h).to_ascii().unwrap();

                // clear screen + move cursor to top-left
                print!("\x1b[2J\x1b[H");
                canvas.render(f, Padding::Center);
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        }
        Some("png") | Some("jpg") | Some("jpg") | Some("jpeg") => {
            let img = ImageDecoder::new(args.path).decode().unwrap();

            let frame = img.resize(term_w, term_h).to_ascii().unwrap();

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
