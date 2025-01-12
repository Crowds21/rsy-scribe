use std::io;
use std::io::stdout;

use crossterm::event::Event;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::{cursor, event, execute, terminal};
use futures_util::Stream;
use ratatui::prelude::{CrosstermBackend, Rect};

use crate::compositor::Compositor;
use crate::editor::*;

type TerminalBackend = CrosstermBackend<std::io::Stdout>;
type MyTerminal = ratatui::terminal::Terminal<TerminalBackend>;
/// State of the entire Application
pub struct Application {
    /// Ratatui 终端
    pub terminal: MyTerminal,
    /// State of the Editor
    pub editor: Editor,
    pub compositor: Compositor,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal = 0,
    Select = 1,
    Insert = 2,
    Command = 3,
}

impl Application {
    pub fn new() -> Self {
        let backend = CrosstermBackend::new(stdout());
        let terminal = MyTerminal::new(backend).unwrap();
        let area = terminal.size().expect("couldn't get terminal size");
        let mut mutompositor = Compositor::new(area);
        Self {
            terminal,
            editor: Editor::new(),
            compositor: mutompositor,
        }
    }

    pub fn event_loop(&mut self, input: event::Event) {
        // 创建上下文
        let mut cx = crate::compositor::Context {
            editor: &mut self.editor,
        };
        // 渲染基础的 UI
        self.render();

        // 循环中进行事件处理,以及基于事件的触发来更新 UI
        loop {
            let should_redraw: bool = match input {
                Event::Resize(width, height) => {
                    self.terminal
                        .resize(Rect::new(0, 0, width, height))
                        .expect("Unable to resize terminal");

                    let area = self.terminal.size().expect("couldn't get terminal size");
                    self.compositor.resize(area);

                    self.compositor
                        .handle_event(&Event::Resize(width, height), &mut cx)
                }
                Event::Key(event::KeyEvent {
                    kind: event::KeyEventKind::Release,
                    ..
                }) => false,
                // 这里是直接传入的 函数参数 event. 而在 helix 中,他是获取函数参数中的一个局部变量
                ref input => self.compositor.handle_event(input, &mut cx),
            };
            if should_redraw {
                self.render();
            }
        }
    }

    /// 调用 Compositor 来进行 UI 渲染,这里还需要手动更改一下
    fn render(&mut self) {
        if self.compositor.full_redraw {
            self.terminal.clear().expect("Cannot clear the terminal");
            self.compositor.full_redraw = false;
        }

        let mut cx = crate::compositor::Context {
            editor: &mut self.editor,
        };

        cx.editor.needs_redraw = false;

        self.terminal
            .autoresize()
            .expect("Unable to determine terminal size");
        let area = self.terminal.size().expect("couldn't get terminal size");

        let surface = self.terminal.current_buffer_mut();

        self.compositor.render(area, surface, &mut cx);
        let (col,row) = self.compositor.cursor_position(area, &self.editor).unwrap();
        // TODO
        // reset cursor cache
        // self.editor.cursor_cache.set(None);

        // let pos = pos.map(|pos| (pos.col as u16, pos.row as u16));
        // self.terminal.draw(pos, kind).unwrap();
    }

    /// Enters editor mode, usually involving disabling line buffering
    /// and echo
    fn enter_editor_mode(&self) -> io::Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        Ok(())
    }

    /// Get and save windows size
    fn get_window_size(&mut self) {
        match crossterm::terminal::size() {
            Ok((cols, rows)) => {
                // self.screen_rows = rows;
                // self.screen_cols = cols;
            }
            Err(e) => {
                eprintln!("Failed to get windows size: {}", e);
            }
        }
        // let (cols, rows) = crossterm::terminal::size().unwrap();
    }

    ///
    fn get_cursor_position(&self) -> anyhow::Result<(u16, u16)> {
        let (x, y) = cursor::position()?;
        Ok((x, y))
    }
}
