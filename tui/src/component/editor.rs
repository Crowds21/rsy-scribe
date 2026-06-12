use super::*;
use crate::component::buffer_line::render_buffer_line;
use crate::component::gutter::{gutter_total_width, render_gutter, GutterConfig};
use crate::component::search_box::SearchBox;
use crate::compositor::{Compositor, CompositorContext, EventResult};
use crossterm::event::{KeyCode, KeyEvent};
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
            gutter_area: Rect::default(),
            content_area: Rect::default(),
            status_msg,
            count,
            gutter: GutterConfig::default(),
            need_redraw: false,
        }
    }

    fn scroll_document(&mut self, code: KeyCode, cx: &mut CompositorContext) -> EventResult {
        let viewport_height = self.content_area.height;
        match code {
            KeyCode::Down => {
                if cx.can_scroll_down(viewport_height) {
                    cx.scroller_forward(1, viewport_height);
                }
            }
            KeyCode::Up => {
                if cx.can_scroll_up() {
                    cx.scroller_backward(1);
                }
            }
            KeyCode::PageDown => {
                if cx.can_scroll_down(viewport_height) {
                    cx.scroll_page_down(viewport_height);
                }
            }
            KeyCode::PageUp => {
                if cx.can_scroll_up() {
                    cx.scroll_page_up(viewport_height);
                }
            }
            KeyCode::Home => cx.scroll_to_top(),
            KeyCode::End => cx.scroll_to_bottom(viewport_height),
            _ => return EventResult::Consumed(None),
        }
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
                    // 合并多个样式：累加修饰符，后面的颜色覆盖前面的
                    let mut style = Style::default();
                    for style_name in &item.styles {
                        let theme_style = cx.theme.get(&style_name.clone());
                        // 累加修饰符（bold, italic, underline 等）
                        style = style
                            .add_modifier(theme_style.add_modifier)
                            .remove_modifier(theme_style.sub_modifier);
                        // 颜色使用最后一个非默认值（后面的覆盖前面的）
                        if theme_style.fg.is_some() {
                            style = style.fg(theme_style.fg.unwrap());
                        }
                        if theme_style.bg.is_some() {
                            style = style.bg(theme_style.bg.unwrap());
                        }
                    }
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
            KeyCode::Down | KeyCode::Up | KeyCode::PageDown | KeyCode::PageUp | KeyCode::Home | KeyCode::End => {
                self.scroll_document(event.code, cx)
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
        let editor_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(1), // Buffer line
                Constraint::Min(1),    // 主编辑器区域
                Constraint::Length(1), // 状态栏
            ])
            .split(area)[1]; // 主编辑器区域

        let scroll = cx.scroll.unwrap_or_default();
        let total_lines = cx
            .editor_model
            .get_current_doc_height()
            .max(1) as usize;
        let doc_lines = cx
            .editor_model
            .get_current_document()
            .map(|doc| doc.lines.as_slice())
            .unwrap_or(&[]);
        let gutter_width = gutter_total_width(total_lines, &self.gutter);

        // Gutter
        let (gutter_area, content_area) = {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![
                    Constraint::Length(gutter_width),
                    Constraint::Min(1), // 内容区
                ])
                .split(editor_area);
            (chunks[0], chunks[1])
        };
        self.content_area = content_area;
        self.gutter_area = gutter_area;

        let gutter_style = cx
            .theme
            .styles
            .get("editor.gutter")
            .or_else(|| cx.theme.styles.get("ui.gutter"))
            .copied()
            .unwrap_or(Style::default().fg(Color::DarkGray).bg(Color::Black));
        let line_number_style = cx
            .theme
            .styles
            .get("editor.gutter.line_number")
            .copied()
            .unwrap_or(gutter_style);
        let icon_style = cx
            .theme
            .styles
            .get("editor.gutter.icon")
            .copied()
            .unwrap_or(gutter_style);

        render_gutter(
            frame,
            gutter_area,
            &self.gutter,
            doc_lines,
            scroll,
            &cx.icons,
            gutter_style,
            line_number_style,
            icon_style,
        );

        // Buffer line — 展示已打开文档
        let buffer_line_area = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(1)])
            .split(area)[0];
        render_buffer_line(frame, buffer_line_area, cx.editor_model, &cx.theme);

        // Editor
        self.render_document(frame, cx);

        // 阅读模式：不显示块光标

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

    fn cursor_position(&self, _area: Rect) -> Option<(u16, u16)> {
        None
    }

    fn id(&self) -> Option<&'static str> {
        Some(ID)
    }
    
}
