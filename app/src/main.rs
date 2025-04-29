use crossterm::{
    event::Event,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, panic};
use tui::compositor::{Compositor, CompositorContext};
use crate::application::Application;

mod application;

fn main() -> io::Result<()> {
    main_impl()
}

#[tokio::main]
async fn main_impl() -> io::Result<()> {
    let mut app = Application::new();
    app.run().await;
    Ok(())


}

