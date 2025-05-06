mod search_box_debounce;

use super::*;
use crate::compositor::{Compositor, CompositorContext, EventResult};
use crossterm::event::KeyEvent;
use ratatui::{
    prelude::*,
    style::{Modifier, Style},
    widgets::*,
};
use tokio::sync::mpsc::Sender;
use syservice;
use unicode_width::UnicodeWidthStr;
use crate::component::search_box::search_box_debounce::SearchBoxDebounce;
use crate::debounce::{send_blocking, AsyncHook};

pub const ID: &str = "search-box";
/// 可搜索的文本框组件
#[derive(Debug)]
pub struct SearchBox {
    cursor_position: usize,
    /// 当前输入内容
    input: String,
    /// item[0]为输入内容, 其余为搜索结果
    items: Vec<String>,
    /// 搜索结果显示列表
    results: Vec<String>,
    /// Ratatui ui 状态
    list_state: ListState,
    selected_result: Option<usize>,
    /// 是否处于活跃状态(接收输入)
    active: bool,
    /// 输入框标题
    title: String,

    pub width: u16,
    pub height: u16,

    /// 延时搜索
    async_sender:Sender<String>, 
}
impl<'a> Default for SearchBox {
    fn default() -> Self {
        
        let search_debounce = SearchBoxDebounce::new();
        let sender = search_debounce.spawn();
        Self {
            cursor_position: 0,
            input: String::new(),
            items: vec!["[输入搜索内容]".to_string()],
            results: vec![],
            list_state: ListState::default(),
            selected_result: None,
            active: false,
            title: "Search".to_string(),
            width: 0,
            height: 0,
            async_sender:sender
        }
    }
}
impl Component for SearchBox {
    fn render(&mut self, frame: &mut Frame, area: Rect,cx: &mut CompositorContext) {
        let inner_area = Rect {
            x: area.x + 5,
            y: area.y + 5,
            width: area.width.saturating_sub(10),   // 左右各减5
            height: area.height.saturating_sub(10), // 上下各减5
        };
        // 需要对传入的 area 进行计算再处理
        // 分割上下区域
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(inner_area);

        // 渲染输入框
        let input_block = Block::default()
            .title(self.title.clone())
            .borders(Borders::ALL)
            .border_style(if self.active {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        // 计算光标在屏幕上的位置
        let cursor_x = self.input[..self.cursor_position].width() as u16 + 4;
        let block_content = " > ".to_owned() + self.input.as_str();
        let input = Paragraph::new(block_content)
            .block(input_block)
            .style(Style::default().fg(Color::White))
            .scroll((0, 0)); // 添加滚动支持
        frame.render_widget(input, chunks[0]);

        // 渲染光标
        frame.set_cursor(
            chunks[0].x + cursor_x,
            chunks[0].y + 1, // 通常放在输入区域的中间行
        );

        // 渲染结果列表（保持不变）
        let results_block = Block::default().borders(Borders::ALL);

        let items: Vec<ListItem> = self
            .results
            .iter()
            .map(|r| ListItem::new(r.as_str()))
            .collect();

        let list = List::new(items)
            .block(results_block)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(" > ");

        match self.selected_result {
            Some(selected) => {
                self.list_state.select(Some(selected));
            }
            None => {
                self.selected_result = Some(0);
                self.list_state.select(Some(0)); 
            }
        }

        frame.render_stateful_widget(list, chunks[1], &mut self.list_state);
    }

    fn handle_event(&mut self, event: KeyEvent, context: &mut CompositorContext) -> EventResult {
        match event.code {
            crossterm::event::KeyCode::Char(c) => {
               self.handle_search_input(c)
            }
            crossterm::event::KeyCode::Backspace => {
                self.handle_delete_char()
            }
            crossterm::event::KeyCode::Delete => {
                self.handle_delete_char()
            }
            crossterm::event::KeyCode::Left => {
                if self.cursor_position > 2 {
                    let prev_char_len = self.input[..self.cursor_position]
                        .chars()
                        .last()
                        .unwrap()
                        .len_utf8();
                    self.cursor_position -= prev_char_len;
                }
                EventResult::Consumed(None)
            }
            crossterm::event::KeyCode::Right => {
                if self.cursor_position < self.input.len() {
                    let next_char_len = self.input[self.cursor_position..]
                        .chars()
                        .next()
                        .unwrap()
                        .len_utf8();
                    self.cursor_position += next_char_len;
                }
                EventResult::Consumed(None)
            }
            crossterm::event::KeyCode::End => {
                self.cursor_position = self.input.len();
                EventResult::Consumed(None)
            }
            crossterm::event::KeyCode::Enter => {
                EventResult::Consumed(None)
            }
            crossterm::event::KeyCode::Down => {
                if !self.results.is_empty() {
                    self.selected_result = Some(match self.selected_result {
                        Some(i) if i <= self.results.len() - 1 => i + 1,
                        None if !self.results.is_empty() => 0,
                        _ => self.results.len() - 1,
                    });
                }

                EventResult::Consumed(None)
            }
            crossterm::event::KeyCode::Up => {
                if !self.results.is_empty() {
                    self.selected_result = match self.selected_result {
                        Some(0) | None => Some(0),
                        Some(i) => Some(i - 1),
                    };
                }
                EventResult::Consumed(None)
            }
            crossterm::event::KeyCode::Esc => {
                let callback: crate::compositor::Callback = Box::new(
                    move |compositor: &mut Compositor, cx: &mut CompositorContext| {
                        compositor.pop();
                    },
                );
                EventResult::Consumed(Some(callback))
            }
            _ =>  EventResult::Ignored(None),
        }
    }
    fn id(&self) -> Option<&'static str> {
        Some(ID)
    }
}

impl<'a> SearchBox {
    /// 创建新的SearchBox
    pub fn new(title: &'a str, results_title: &'a str) -> Self {
        let title = title.to_string();
        Self {
            title,
            items: vec!["[输入搜索内容]".to_string()],
            ..Default::default()
        }
    }

    /// 设置组件活跃状态
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }

    /// 获取当前输入内容
    pub fn input(&self) -> &str {
        &self.input
    }

    /// 获取当前选中的结果
    pub fn selected_result(&self) -> Option<&String> {
        self.selected_result.and_then(|i| self.results.get(i))
    }
    pub fn update_results(&mut self, results: Vec<String>) {
        self.items.truncate(1); // 保留输入框
        self.items.extend(results);
    }

    /// 清除选中状态
    pub fn clear_selection(&mut self) {
        self.selected_result = None;
    }
    fn move_selection(&mut self, delta: i32) {
        let new_idx = self
            .list_state
            .selected()
            .map(|i| (i as i32 + delta).max(0) as usize)
            .unwrap_or(0);

        self.list_state
            .select(Some(new_idx % self.items.len().max(1)));
    }

    /// 处理用户字符输入.
    fn handle_search_input(&mut self, c:char) -> EventResult {
        self.input.insert(self.cursor_position, c);
        self.cursor_position += c.len_utf8();
        send_blocking(&self.async_sender, self.input.clone());
        
        EventResult::Consumed(None)
    }
    
    fn handle_delete_char(&mut self) -> EventResult {
        if self.cursor_position > 0 {
            let prev_char_len = self.input[..self.cursor_position]
                .chars()
                .last()
                .unwrap()
                .len_utf8();
            self.input.remove(self.cursor_position - prev_char_len);
            self.cursor_position -= prev_char_len;
            send_blocking(&self.async_sender, self.input.clone());
        }

        EventResult::Consumed(None)
    }

}