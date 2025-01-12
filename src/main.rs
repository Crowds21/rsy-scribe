use crate::application::Application;
use crossterm::{
    event::{self},
    ExecutableCommand,
};
use std::io::{self, Read, Write};
pub mod application;
pub mod commands;
mod compositor;
pub mod editor;
pub mod keymap;
pub mod syapi;

/// enter_editor_mode: Init Terminal
/// get_windwos_size
/// editor_open
fn main() -> io::Result<()> {
    let mut application = Application::new();
    // application
    // editor_config.enter_editor_mode()?;
    // editor_conadflk;;ig.get_window_size();

    // execute!(cursor::Hide).unwrap();
    // 这里可以添加一些屏幕绘制的操作
    // execute!(cursor::Show).unwrap();
    // let mut buffer = [0; 1]; // 单字节的缓冲区
    // let stdin = io::stdin();
    // let mut handle = stdin.lock(); // 锁定标准输入以提高效率
    loop {
        application.event_loop(event::read()?)
    }
}
// Ok(())
