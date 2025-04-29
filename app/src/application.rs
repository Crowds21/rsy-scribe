use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::{
    event::Event,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, panic};
use tui::compositor::{Compositor, CompositorContext};
use tui::job::JobQueue;

/// 应用后台
pub struct Application {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    compositor: Compositor,
    compositor_context: CompositorContext,
    pub jobs: JobQueue, // 引用全局 JobQueue
}
pub struct ApplicationState {
    pub search_result: Vec<String>,
    pub document: Vec<String>,
}
impl Application {
    pub fn new() -> Self {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).expect("Enter alternate screen error");

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("terminal initialization failed");
        let mut compositor = Compositor::new();
        enable_raw_mode().expect("Enter raw mode error");
        let mut cx = CompositorContext::new();
        Self {
            terminal,
            compositor,
            compositor_context: cx,
            jobs: JobQueue::new(),
        }
    }

    pub(crate) async fn run(&mut self) {
        let mut input_stream = crossterm::event::EventStream::new();
        use futures_util::StreamExt;
        // 主循环
        loop {
            self.render().await;

            tokio::select! {
                biased;
                Some(Ok(event)) = input_stream.next() => {
                    if let Event::Key(key) = event {
                        if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                            self.exit_app();
                            break;
                        }
                        self.compositor.handle_event(key.clone(), &mut self.compositor_context);
                    }
                }
                Some(callback) = self.jobs.callbacks.recv() => {
                    self.jobs.handle_callback(&mut self.compositor, Ok(Some(callback)));
                    // self.render().await;
                }
            }
        }
    }

    /// 进行绘制
    pub async fn render(&mut self) {
        // self.terminal.draw(pos).unwrap();
        self.terminal
            .draw(|f| {
                self.compositor.render(f, f.size());
            })
            .expect("rendering error");
    }

    pub fn exit_app(&mut self) {
        disable_raw_mode().expect("Disable raw mode before exit");
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    }
}
