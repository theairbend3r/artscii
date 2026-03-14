use std::process::Command;

pub fn get_terminal_size() -> (u32, u32) {
    // for linux/unix
    let terminal_width = Command::new("tput").arg("cols").output();
    let terminal_height = Command::new("tput").arg("lines").output();

    if let (Ok(w_output), Ok(h_output)) = (terminal_width, terminal_height) {
        // extract stdout
        let w_stdout = String::from_utf8(w_output.stdout);
        let h_stdout = String::from_utf8(h_output.stdout);

        if let (Ok(w_stdout), Ok(h_stdout)) = (w_stdout, h_stdout) {
            // convert string to number
            let w = w_stdout.trim().parse::<u32>();
            let h = h_stdout.trim().parse::<u32>();

            // return number
            if let (Ok(w), Ok(h)) = (w, h) {
                return (w, h);
            }
        }
    }
    // use default terminal size if above fails
    (80, 24)
}
