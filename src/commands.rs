use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveUp},
    execute,
    terminal::{disable_raw_mode, LeaveAlternateScreen},
    ExecutableCommand,
};
use std::io::stdout;

pub fn reset_terminal_and_exit() {
    let _ = stdout().execute(LeaveAlternateScreen);
    let _ = disable_raw_mode();
    std::process::exit(0)
}
/// 光标左移len个位置
/// TODO: 换行判断
pub fn move_curosr_left() {
    execute!(stdout(), MoveLeft(1));
}

pub fn move_curosr_right() {
    execute!(stdout(), MoveRight(1));
}

pub fn move_curosr_up() {
    execute!(stdout(), MoveUp(1));
}

pub fn move_curosr_down() {
    execute!(stdout(), MoveDown(1));
}
