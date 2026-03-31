use super::*;
use crate::adaptor::rect::URect;
use crate::component::gutter::{render_gutter, GutterConfig, GutterType};
use crate::component::search_box::SearchBox;
use crate::compositor::{Compositor, CompositorContext, EventResult};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Position, Size};
use ratatui::text::{Line, Span};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use ratatui::widgets::Clear;

pub const ID: &str = "editor-view";
pub struct EditorView {
    pub cursor_position: Position,
    pub gutter_area: Rect,
    pub content_area: Rect,
    status_msg: Option<String>, // 状态消息
    count: Option<u32>,         // 模拟按键计数
    /// 侧边栏
    gutter: GutterConfig,
    pub need_redraw: bool,
}
impl<'a> Default for EditorView {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> EditorView {
    pub fn new() -> Self {
        let status_msg = Some("status".to_string());
        let count = None;
        Self {
            cursor_position: Position::default(),
            gutter_area: Rect::default(),
            content_area: Rect::default(),
            status_msg,
            count,
            gutter: GutterConfig::default(),
            need_redraw: false,
        }
    }

    fn cursor_move(&mut self, code: KeyCode,cx: &mut CompositorContext) -> EventResult {
        let offset = cx.scroll.unwrap_or_default();
        // 当光标向下移动的位置超出屏幕展示边界,offset+1
        // 当光标向上超出屏幕展示边界, offset -1 
        // TODO 但是需判断文档的总长度
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

    fn render_document(
        &mut self,
        frame: &mut Frame,
        cx: &mut CompositorContext,
    ) {
        let content_area = self.content_area;
        if cx.editor_model.documents.is_empty() {
            return;
        }
        let mut remaining_height = content_area.height;
        let result = cx.editor_model.get_current_mutable_document();
        if let Some(document) = result {
            if self.need_redraw {
                // 如果宽度发生变化,需要重新计算并生成树结构
                document.parse_ast_root_node(self.content_area.width);
                self.need_redraw = false;
            }
            let offset = cx.scroll.unwrap_or_default();
            let mut remaining_height = content_area.height;
            let mut current_y = content_area.y;

            // 计算要渲染的起始行和结束行
            let start_line = offset;
            let end_line = (start_line + remaining_height).min(document.lines.len() as u16);
            // 逐行渲染可见部分
            for line in document.lines
                .iter()
                .skip(start_line as usize)
                .take((end_line - start_line) as usize)
            {
                if remaining_height == 0 {
                    break;
                }
                // 只能是逐行渲染,或者按照元素类型,按块渲染,因为需要设置gutter
                let mut rendered_line = Line::default();
                for item in line.content.iter() {
                    // TODO Add style
                    let style = match &item.style_name {
                        None => Style::default(),
                        Some(it) => cx.theme.get(&it.clone()),
                    };
                    let span = Span::from(item.display_content.clone()).style(style);
                    rendered_line.push_span(span)
                }

                let render_area = Rect {
                    x: content_area.x,
                    y: current_y,
                    width: content_area.width,
                    height: 1,
                };

                frame.render_widget(rendered_line, render_area);
                current_y += 1;
                remaining_height -= 1;
            }
        }
    }

    fn handle_key_event(&mut self, event: &KeyEvent, cx: &mut CompositorContext) -> EventResult {
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
                self.cursor_move(event.code,cx)
            }
            _ => EventResult::Ignored(None), // 其他按键不处理
        }
    }

    fn handle_resize_event(
        &mut self,
        width: u16,
        height: u16,
        cx: &mut CompositorContext,
    ) -> EventResult {
        self.need_redraw = true;
        EventResult::Ignored(None)
    }
}

impl Component for EditorView {
    fn render(&mut self, frame: &mut Frame, area: Rect, cx: &mut CompositorContext) {
        let area = frame.size();

        // 清空背景

        frame.render_widget(Clear, area);
        let editor_bg = cx.theme.styles.get("editor.bg").unwrap();
        frame.render_widget(Block::default().style(*editor_bg), area);

        // 计算编辑器区域（减去状态栏和可能的 BufferLine）
        // TODO BufferLine 需要动态渲染
        let editor_area = Layout::default()
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
        let buffer_line_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1)])
            .split(area)[0];
        let buffer_line = Paragraph::new("Buffer 1 | Buffer 2")
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(buffer_line, buffer_line_area);

        // Editor
        self.render_document(frame, cx);

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
            Event::Key(e) => self.handle_key_event(e, cx),
            Event::Resize(w, h) => self.handle_resize_event(*w, *h, cx),
            _ => EventResult::Consumed(None),
        }
    }

    fn cursor_position(&self, area: Rect) -> Option<(u16, u16)> {
        todo!()
    }

    fn id(&self) -> Option<&'static str> {
        Some(ID)
    }
    
}
