use crate::compositor::{Component, Compositor, Context};
use crossterm::event::{self, Event, KeyCode};
use ratatui::buffer::Buffer;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget},
    Terminal,
};
use std::io;

/// 自定义组件：包含输入框和搜索结果列表
struct SearchBox {
    input: String,               // 输入框内容
    search_results: Vec<String>, // 搜索结果列表
}
impl SearchBox {
    fn new() -> Self {
        Self {
            input: String::new(),
            search_results: Vec::new(),
        }
    }
}
impl Component for SearchBox {
    /// 处理键盘事件
    fn component_event(&mut self, event: Event) {

        let close_fn = Box::new(|compositor: &mut Compositor| {
            compositor.pop();
        });

        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char(c) => {
                    self.input.push(c); // 输入字符
                }
                KeyCode::Backspace => {
                    self.input.pop(); // 删除字符
                }
                KeyCode::Enter => {
                    // 模拟搜索逻辑
                    self.search_results = self
                        .input
                        .split_whitespace()
                        .map(|s| format!("Result: {}", s))
                        .collect();
                }
                KeyCode::Esc => {

                }
                _ => {}
            }
        }
    }

    /// 渲染组件
    fn render(&mut self, area: Rect, buf: &mut Buffer, cx: &mut Context) {

        // 将区域分为上下两部分
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(area);

        // 渲染输入框
        let input_block = Paragraph::new(self.input.as_ref())
            .block(Block::default().borders(Borders::ALL).title("Input"));
        input_block.render(chunks[0], buf);

        // 渲染搜索结果列表
        let items: Vec<ListItem> = self
            .search_results
            .iter()
            .map(|result| ListItem::new(result))
            .collect();
        let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Results"));
        list.render(chunks[1], buf);
    }
}
