use crate::application::Application;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use ratatui::backend::Backend;
use std::io;
use std::io::stdout;
use std::panic::{set_hook, take_hook};
mod application;

fn main() -> io::Result<()> {
    main_impl()
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
pub fn restore_tui() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
