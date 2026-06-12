use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::{
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::layout::Rect;
use ratatui::{backend::CrosstermBackend, Terminal};
use syservice::config::Config;
use std::io;
use std::panic;
use tui::compositor::{Compositor, CompositorContext};
use tui::job::JobQueue;
use view::editor::EditorModel;

use crate::restore_tui;

/// 应用后台
pub struct Application {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    compositor: Compositor,
    editor_model: EditorModel,
    pub jobs: JobQueue, // 引用全局 JobQueue
}
impl Application {
    pub fn new() -> Self {
        let _ = Config::init_global();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).expect("Enter alternate screen error");

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).expect("terminal initialization failed");
        let compositor = Compositor::new(terminal.size().unwrap());
        enable_raw_mode().expect("Enter raw mode error");
        let editor_model = EditorModel::default();
        Self {
            terminal,
            compositor,
            jobs: JobQueue::new(),
            editor_model,
        }
    }

    /// 主线程
    pub(crate) async fn run(&mut self) {
        let mut input_stream = crossterm::event::EventStream::new();
        use futures_util::StreamExt;
        // 主循环
        loop {
            // 重新渲染接收新的事件前,重新渲染上一次的处理结果
            self.render().await;
            tokio::select! {
                biased;
                Some(Ok(event)) = input_stream.next() => {
                    self.handle_terminal_events(event).await;
                }
                Some(callback) = self.jobs.callbacks.recv() => {
                    self.jobs.handle_callback(&mut self.editor_model,&mut self.compositor, Ok(Some(callback)));
                    // self.render().await;
                }
            }
        }
    }

    /// 事件处理
    async fn handle_terminal_events(&mut self, event: Event) {
        let compositor = &mut self.compositor;
        let editor_model = &mut self.editor_model;
        let mut cx = CompositorContext {
            editor_model,
            scroll: Some(compositor.scroll),
            theme: tui::uiconfig::theme::Theme::default(),
            icons: tui::uiconfig::Icons::default(),
        };
        match event {
            Event::Resize(width, height) => {
                self.terminal
                    .resize(Rect::new(0, 0, width, height))
                    .expect("Unable to resize terminal");
                let area = self.terminal.size().expect("couldn't get terminal size");
                compositor.resize(area);
                compositor.handle_event(&Event::Resize(width, height), &mut cx);
            }
            Event::Key(key) => {
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    compositor.handle_event(&event, &mut cx);
                    self.exit_app();
                    return;
                }
                compositor.handle_event(&event, &mut cx);
            }
            _ => {}
        }
        if let Some(scroll) = cx.scroll {
            compositor.scroll = scroll;
        }
    }

    /// 进行绘制
    pub async fn render(&mut self) {
        let terminal = &mut self.terminal;
        let compositor = &mut self.compositor;
        let editor_model = &mut self.editor_model;
        terminal
            .draw(|f| {
                let mut cx = CompositorContext {
                    editor_model,
                    scroll: Some(compositor.scroll),
                    theme: tui::uiconfig::theme::Theme::default(),
                    icons: tui::uiconfig::Icons::default(),
                };
                compositor.render(f, f.size(), &mut cx);
                if let Some(scroll) = cx.scroll {
                    compositor.scroll = scroll;
                }
            })
            .expect("rendering error");
    }

    pub fn exit_app(&mut self) {
        let _ = restore_tui();
        std::process::exit(0);
    }
}
