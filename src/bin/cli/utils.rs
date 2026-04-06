use crossterm::terminal;
use log::{info, warn};

pub fn get_terminal_size() -> (u32, u32) {
    match terminal::size() {
        Ok((w, h)) => {
            info!("Detected terminal size: {}x{}", w, h);
            (w as u32, h as u32)
        }
        Err(e) => {
            warn!(
                "Unable to get terminal size ({}). Using the default size of (80,24)",
                e
            );
            (80, 24)
        } // fallback
    }
}
