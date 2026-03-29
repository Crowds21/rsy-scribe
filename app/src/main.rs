use crate::application::Application;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use std::io;
use std::io::stdout;
use std::panic::{set_hook, take_hook};
mod application;

fn main() -> io::Result<()> {
    let result = main_impl();
    // 确保退出时恢复终端
    let _ = restore_tui();
    result
}

#[tokio::main]
async fn main_impl() -> io::Result<()> {
    init_panic_hook();
    let mut app = Application::new();
    app.run().await;
    Ok(())
}

pub fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

/// 恢复终端到正常状态
/// 在应用退出时必须调用此函数
pub fn restore_tui() -> io::Result<()> {
    let result = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);
    result
}
