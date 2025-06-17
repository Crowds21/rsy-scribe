use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};
use crossterm::{
    event::Event,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};
use ratatui::layout::Rect;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, panic};
use tui::compositor::{Compositor, CompositorContext};
use tui::job::JobQueue;
use tui::model::editor_model::EditorModel;

/// 应用后台
pub struct Application {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    compositor: Compositor,
    compositor_context: CompositorContext,
    editor_model: EditorModel<'static>,
    pub jobs: JobQueue, // 引用全局 JobQueue
}
impl Application {
    pub fn new() -> Self {
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).expect("Enter alternate screen error");

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("terminal initialization failed");
        let mut compositor = Compositor::new(terminal.size().unwrap());
        enable_raw_mode().expect("Enter raw mode error");
        let mut cx = CompositorContext::new();
        Self {
            terminal,
            compositor,
            compositor_context: cx,
            jobs: JobQueue::new(),
            editor_model: EditorModel::default(),
        }
    }

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

    async fn handle_terminal_events(&mut self, event: Event) {
        match event {
            Event::Resize(width, height) => {
                self.terminal
                    .resize(Rect::new(0, 0, width, height))
                    .expect("Unable to resize terminal");
                let area = self.terminal.size().expect("couldn't get terminal size");

                self.compositor.resize(area);

                self.compositor
                    .handle_event(&Event::Resize(width, height), &mut self.compositor_context);
            }
            Event::Key(key) => {
                if key.modifiers == KeyModifiers::CONTROL && key.code == KeyCode::Char('c') {
                    self.exit_app();
                }
                self.compositor
                    .handle_event(&event, &mut self.compositor_context);
            }
            _ => {}
        }
    }
    /// 进行绘制
    pub async fn render(&mut self) {
        // self.terminal.draw(pos).unwrap();
        self.terminal
            .draw(|f| {
                self.compositor
                    .render(f, f.size(), &mut self.compositor_context);
            })
            .expect("rendering error");
    }

    pub fn exit_app(&mut self) {
        disable_raw_mode().expect("Disable raw mode before exit");
        execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
    }
}
