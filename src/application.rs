use std::io::stdout;

use anyhow::Error;
use futures_util::Stream;
use ratatui::prelude::CrosstermBackend;

pub mod editor;
use editor::*;

type TerminalBackend = CrosstermBackend<std::io::Stdout>;
type MyTerminal = ratatui::terminal::Terminal<TerminalBackend>;
/// State of the entire Application
pub struct Application {
    /// Ratatui 终端
    terminal: MyTerminal,
    /// State of the Editor
    pub editor: Editor,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal = 0,
    Select = 1,
    Insert = 2,
    Command = 3,
}

impl Application {
    fn new() -> Self {
        let backend = CrosstermBackend::new(stdout());
        let terminal = MyTerminal::new(backend).unwrap();
        Self {
            terminal,
            editor: Editor::new(),
        }
    }
    fn key_envent_handler<S>(&mut self, input_stream: &mut S) -> bool
    where
        S: Stream<Item = std::io::Result<crossterm::event::Event>> + Unpin,
    {
        true
    }
}
