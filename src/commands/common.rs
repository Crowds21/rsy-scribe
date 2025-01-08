use super::*;
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
pub fn move_cursor_left(cx: &mut Context) {
    execute!(stdout(), MoveLeft(1));
}

pub fn move_cursor_right(cx: &mut Context) {
    execute!(stdout(), MoveRight(1));
}

pub fn move_cursor_up(cx: &mut Context) {
    execute!(stdout(), MoveUp(1));
}

pub fn move_cursor_down(cx: &mut Context) {
    execute!(stdout(), MoveDown(1));
}
pub fn insert_mode(cx: &mut Context) {
    //
}

pub fn goto_file_start(cx: &mut Context) {
    //
}
pub fn goto_file_end(cx: &mut Context) {
    //
}
pub fn goto_word_end(cx: &mut Context) {
    //
}
pub fn select_mode(cx: &mut Context) {
    //
}
pub fn goto_line(cx: &mut Context) {
    //
}


pub fn search_in_current_file(cx:&mut Context) {
    //
}

pub fn search_globally(cx:&mut Context) {
    //
}
