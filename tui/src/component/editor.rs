use super::*;
use crate::component::block::{doc, BlockComponent, RenderedBlock};
use crate::component::gutter::{render_gutter, GutterConfig, GutterType};
use crate::component::search_box::SearchBox;
use crate::compositor::{Compositor, CompositorContext, EventResult};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Position, Size};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use syservice::lute::node::Node;

pub const ID: &str = "editor-view";
pub struct EditorView<'a> {
    cursor_position: Position,
    pub gutter_area: Rect,
    pub content_area: Rect,
    pub document: Option<Node>,
    pub doc_blocks: Vec<BlockComponent<'a>>,
    status_msg: Option<String>, // 状态消息
    count: Option<u32>,         // 模拟按键计数
    /// 侧边栏
    gutter: GutterConfig,
}
impl<'a> EditorView<'a> {
    pub fn new() -> Self {
        let status_msg = Some("status".to_string());
        let count = None;
        Self {
            cursor_position: Position::default(),
            doc_blocks: Vec::new(),
            gutter_area: Rect::default(),
            content_area: Rect::default(),
            document: None,
            status_msg,
            count,
            gutter: GutterConfig::default(),
        }
    }
    
    fn cursor_move(&mut self, code: KeyCode) -> EventResult {
        let new_pos = match code {
            KeyCode::Down if self.cursor_position.y + 1 < self.content_area.height => Position {
                x: self.cursor_position.x,
                y: self.cursor_position.y.saturating_add(1),
            },
            KeyCode::Up => Position {
                x: self.cursor_position.x,
                y: self.cursor_position.y.saturating_sub(1),
            },
            KeyCode::Left => Position {
                x: self.cursor_position.x.saturating_sub(1),
                y: self.cursor_position.y,
            },
            KeyCode::Right if self.cursor_position.x + 1 < self.content_area.width => Position {
                x: self.cursor_position.x.saturating_add(1),
                y: self.cursor_position.y,
            },
            _ => return EventResult::Consumed(None),
        };
        self.cursor_position = new_pos;
        EventResult::Consumed(None)
    }

    pub fn render_document(
        // 使用不同的生命周期名称 'b
        & mut self,
        frame: & mut Frame,
        content_area: Rect,
        cx: & mut CompositorContext,
    ) {
        let mut vec: Vec<RenderedBlock> = Vec::new();
        if let Some(node) = &mut self.document {
            vec = doc::create_document_blocks(node, cx);
        }

        let mut current_y = content_area.y;
        let mut remaining_height = content_area.height;

        for item in vec {
            if remaining_height == 0 {
                break;
            }

            let render_height = item.rendered_height.min(remaining_height);
            let render_area = Rect {
                x: content_area.x,
                y: current_y,
                width: content_area.width,
                height: render_height,
            };
            item.render(frame, render_area);
            // 确保 RenderedBlock 实现了 Widget trait

            current_y += render_height;
            remaining_height -= render_height;
        }
    }

    fn handle_key_event(&mut self,event:&KeyEvent, cx: &mut CompositorContext) -> EventResult {
        match event.code {
            KeyCode::Char(' ') => {
                // 当按下空格键时，添加 SearchBox 组件
                let search_box = SearchBox::new("Search", "Result");
                // TODO:  由于 Rust 默认不允许"多重借用???" Helix通过
                //  单独的函数来将一个把 Compositor 作为参数的 fn 存入 callback
                //  参考 compositor 中的 handle_event 函数
                //  每一个组件实际上会返回一个 将 compositor 作为参数的函数.
                //  然后这个函数在 Compositor.handle_event 中被执行
                let callback: Callback = Box::new(
                    move |compositor: &mut Compositor, cx: &mut CompositorContext| {
                        compositor.push(Box::new(search_box));
                    },
                );
                //  返回给上一层的 callback
                EventResult::Consumed(Some(callback))
            }
            KeyCode::Down | KeyCode::Up | KeyCode::Left | KeyCode::Right => {
                self.cursor_move(event.code)
            }
            _ => EventResult::Ignored(None), // 其他按键不处理
        }
    }
}

impl Component for EditorView<'static> {
    fn render(&mut self, frame: &mut Frame, area: Rect, cx: &mut CompositorContext) {
        let area = frame.size();

        // 清空背景
        let editor_bg = cx.theme.styles.get("editor.bg").unwrap();
        frame.render_widget(Block::default().style(*editor_bg), area);

        // 计算编辑器区域（减去状态栏和可能的 BufferLine）
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
        self.content_area = content_area;
        self.gutter_area = gutter_area;

        // 计算文本行号,渲染Gutter
        let total_lines = 3;
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
        self.render_document(frame, content_area, cx);

        frame.set_cursor(
            content_area.x + self.cursor_position.x,
            content_area.y + self.cursor_position.y,
        );

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

    fn handle_event(&mut self, event: &Event, cx: &mut CompositorContext) -> EventResult {
        match event { 
            Event::Key(e) =>{
                self.handle_key_event(e, cx) 
            }
            _ => {
                EventResult::Consumed(None)
            }
        }
    }


    fn cursor_position(&self, area: Rect) -> Option<(u16, u16)> {
        todo!()
    }

    fn id(&self) -> Option<&'static str> {
        Some(ID)
    }
}
