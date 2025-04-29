use crossterm::event::{KeyCode, KeyEvent};
use crate::compositor::{Compositor, CompositorContext, EventResult};
use crate::component::search_box::SearchBox;
use super::*;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::component::gutter::{render_gutter, GutterConfig, GutterType};


pub const ID: &str = "editor-view";
pub struct EditorView {
    documents: Vec<String>,      // 模拟文档列表
    status_msg: Option<String>, // 状态消息
    count: Option<u32>,          // 模拟按键计数
    gutter:GutterConfig
}
impl EditorView {
    pub fn new() -> Self {
        let documents = vec![String::from("Empty document")];
        let status_msg = Some("status".to_string());
        let count = None;
        Self{
            documents,
            status_msg,
            count,
            gutter: GutterConfig::default()
        }
    }



}

impl Component for EditorView {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let area = frame.size();

        // 1. 清空背景
        frame.render_widget(
            Block::default().style(Style::default().bg(Color::DarkGray)),
            area,
        );

        // 2. 计算编辑器区域（减去状态栏和可能的 BufferLine）
        let mut editor_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1), // Buffer line
                Constraint::Min(1),    // 主编辑器区域
                Constraint::Length(1), // 状态栏
            ])
            .split(area)[1]; // 主编辑器区域

        // Gutter
        let (gutter_area, content_area) = {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Length(8), // 固定宽度Gutter
                    Constraint::Min(1),    // 内容区
                ])
                .split(editor_area);
            (chunks[0], chunks[1])
        };
        // 渲染Gutter
        let total_lines = self.documents.first().map_or(1, |d| d.lines().count());
        // let current_line = self.cursor_position().line.saturating_add(1);
        render_gutter(frame, gutter_area, &self.gutter, total_lines);

        // Buffer line
        let bufferline_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1)])
            .split(area)[0];
        let bufferline = Paragraph::new("Buffer 1 | Buffer 2")
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(bufferline, bufferline_area);

        // Editor
        let temp_content = &String::new();
        let doc_content = self.documents.first().unwrap_or(temp_content);
        let editor =  Paragraph::new(doc_content.as_str())
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(editor, content_area, );

        // Status bar
        let status_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Min(0), Constraint::Length(1)])
            .split(area)[1];

        // 状态消息（左侧）
        let status_msg = self.status_msg.as_deref().unwrap_or("");
        let status =
            Paragraph::new(status_msg).style(Style::default().fg(Color::White).bg(Color::Blue));
        frame.render_widget(status, status_area);
    }

    fn handle_event(&mut self, event: KeyEvent, context: &mut CompositorContext) -> EventResult {
        match event.code {
            KeyCode::Char(' ')=> {
                // 当按下空格键时，添加 SearchBox 组件
                let search_box = SearchBox::new("Search", "Result");
                // TODO:  由于 Rust 默认不允许"多重借用???" Helix通过
                //  单独的函数来将一个把 Compositor 作为参数的 fn 存入 callback
                //  参考 compositor 中的 handle_event 函数
                //  每一个组件实际上会返回一个 将 compositor 作为参数的函数.
                //  然后这个函数在 Compositor.handle_event 中被执行
                let callback: crate::compositor::Callback = Box::new(
                    move |compositor:&mut Compositor, cx:&mut CompositorContext|{
                        compositor.push(Box::new(search_box));
                    }
                );
                //  返回给上一层的 callback
                EventResult::Consumed(Some(callback))
            }
            _ => EventResult::Ignored(None), // 其他按键不处理
        }
    }

    fn cursor_position(&self, area: Rect) -> Option<(u16, u16)> {
        todo!()
    }

    fn id(&self) -> Option<&'static str> {
        Some(ID)
    }
}
