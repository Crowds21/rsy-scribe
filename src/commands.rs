use crossterm::{
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::stdout;

pub fn reset_terminal_and_exit() {
    let _ = stdout().execute(LeaveAlternateScreen);
    let _ = disable_raw_mode();
    std::process::exit(0)
}
fn move_char_left() {}
