use std::future::Future;
use crate::Component;
use syservice;
use crossterm::event::KeyEvent;
use ratatui::{
    prelude::*,
    style::{Modifier, Style},
    widgets::*,
};
use tokio::spawn;
use unicode_width::UnicodeWidthStr;
use crate::compositor::{CompositorContext, EventResult};

/// 可搜索的文本框组件
#[derive(Debug, Default)]
pub struct SearchBox<'a> {
    cursor_position: usize,
    /// 当前输入内容
    input: String,
    /// item[0]为输入内容, 其余为搜索结果
    items: Vec<String>,
    /// 搜索结果显示列表
    results: Vec<String>,
    list_state: ListState,
    selected_result: Option<usize>,
    /// 是否处于活跃状态(接收输入)
    active: bool,
    /// 输入框标题
    title: &'a str,
    block: Block<'a>,

    pub width: u16,
    pub height: u16,
}

impl Component for SearchBox<'_> {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        // 分割上下区域
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(1)])
            .split(area);

        // 渲染输入框
        let input_block = Block::default()
            .title(self.title)
            .borders(Borders::ALL)
            .border_style(if self.active {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        // 计算光标在屏幕上的位置
        let cursor_x =
            self.input[..self.cursor_position].width() as u16 + 1 ;

        let input = Paragraph::new(self.input.as_str())
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
            .highlight_symbol(">> ");

        if let Some(selected) = self.selected_result {
            self.list_state.select(Some(selected));
        }

        frame.render_stateful_widget(list, chunks[1], &mut self.list_state);
    }

    fn handle_event(&mut self, event: KeyEvent, context: &mut CompositorContext) -> EventResult{
        match event.code {
            crossterm::event::KeyCode::Char(c) => {
                self.input.insert(self.cursor_position, c);
                self.cursor_position += c.len_utf8();
                // self.update_input_display();
            }
            crossterm::event::KeyCode::Backspace => {
                if self.cursor_position > 0 {
                    let prev_char_len = self.input[..self.cursor_position]
                        .chars()
                        .last()
                        .unwrap()
                        .len_utf8();
                    self.input.remove(self.cursor_position - prev_char_len);
                    self.cursor_position -= prev_char_len;
                }
            }
            crossterm::event::KeyCode::Delete => {
                if self.cursor_position < self.input.len() {
                    let next_char_len = self.input[self.cursor_position..]
                        .chars()
                        .next()
                        .unwrap()
                        .len_utf8();
                    self.input.remove(self.cursor_position);
                }
            }
            crossterm::event::KeyCode::Left => {
                if self.cursor_position > 0 {
                    let prev_char_len = self.input[..self.cursor_position]
                        .chars()
                        .last()
                        .unwrap()
                        .len_utf8();
                    self.cursor_position -= prev_char_len;
                }
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
            }
            crossterm::event::KeyCode::Home => {
                self.cursor_position = 0;
            }
            crossterm::event::KeyCode::End => {
                self.cursor_position = self.input.len();
            }
            crossterm::event::KeyCode::Enter => {
                self.perform_search()
            }
            crossterm::event::KeyCode::Down => {
                if !self.results.is_empty() {
                    self.selected_result = Some(match self.selected_result {
                        Some(i) if i < self.results.len() - 1 => i + 1,
                        None if !self.results.is_empty() => 0,
                        _ => self.results.len() - 1,
                    });
                }
            }
            crossterm::event::KeyCode::Up => {
                if !self.results.is_empty() {
                    self.selected_result = match self.selected_result {
                        Some(0) | None => None,
                        Some(i) => Some(i - 1),
                    };
                }
            }
            _ => return EventResult::Ignored(None),
        }
        EventResult::Consumed(None)
    }

    fn cursor_position(&self, area: Rect) -> Option<(u16, u16)> {
        todo!()
    }
}
impl<'a> SearchBox<'a> {
    /// 创建新的SearchBox
    pub fn new(title: &'a str, results_title: &'a str) -> Self {
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

    /// 执行搜索(模拟)
    fn perform_search(&mut self) {
        let input = self.input.clone();
        let mut items = &self.items;
        // 启动异步任务
        // tokio::spawn(async move {
        //     // 设置加载状态
        //     let result = syservice::document::search_doc_with_title(input).await;
        //
        //     // 处理结果
        //     self.items = match result {
        //         Ok(res) => res.data.iter().map(|it| it.content.clone()).collect(),
        //         Err(_) => vec!["搜索失败".to_string()],
        //     };
        // });
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent};
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_searchbox_initial_state() {
        let searchbox = SearchBox::new("搜索", "结果");
        assert_eq!(searchbox.input(), "");
        assert!(searchbox.results.is_empty());
        assert!(!searchbox.active);
    }

    #[test]
    fn test_searchbox_input_handling() {
        let mut searchbox = SearchBox::new("搜索", "结果");
        searchbox.set_active(true);

        // 测试字符输入
        searchbox.handle_event(KeyEvent::from(KeyCode::Char('a')), );
        searchbox.handle_event(KeyEvent::from(KeyCode::Char('b')), );
        assert_eq!(searchbox.input(), "ab");

        // 测试退格
        searchbox.handle_event(KeyEvent::from(KeyCode::Backspace), );
        assert_eq!(searchbox.input(), "a");

        // 测试回车执行搜索
        searchbox.handle_event(KeyEvent::from(KeyCode::Enter), );
        assert_eq!(searchbox.results.len(), 3);
        assert!(searchbox.results[0].contains("a - 结果 1"));
    }

    #[test]
    fn test_searchbox_rendering() {
        let mut searchbox = SearchBox::new("搜索", "结果");
        searchbox.set_active(true);
        searchbox.input = "test".to_string();
        searchbox.results = vec!["结果1".to_string(), "结果2".to_string()];

        // 使用测试后端
        let backend = TestBackend::new(40, 20);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                searchbox.render(f, f.size());
            })
            .unwrap();

        // 可以在这里添加对渲染输出的断言
        // 实际项目中可能需要更详细的渲染测试
    }
    #[test]
    fn test_searchbox_cursor_movement() {
        let mut searchbox = SearchBox::new("搜索", "结果");
        searchbox.set_active(true);
        searchbox.input = "测试".to_string();
        searchbox.cursor_position = 0;

        // 测试右移
        searchbox.handle_event(KeyEvent::from(KeyCode::Right), );
        assert_eq!(searchbox.cursor_position, "测".len());

        // 测试左移
        searchbox.handle_event(KeyEvent::from(KeyCode::Left), );
        assert_eq!(searchbox.cursor_position, 0);

        // 测试Home/End
        searchbox.handle_event(KeyEvent::from(KeyCode::End), );
        assert_eq!(searchbox.cursor_position, searchbox.input.len());
        searchbox.handle_event(KeyEvent::from(KeyCode::Home), );
        assert_eq!(searchbox.cursor_position, 0);
    }

    #[test]
    fn test_searchbox_cursor_with_editing() {
        let mut searchbox = SearchBox::new("搜索", "结果");
        searchbox.set_active(true);

        // 测试插入时光标移动
        searchbox.handle_event(KeyEvent::from(KeyCode::Char('a')), );
        assert_eq!(searchbox.cursor_position, 1);
        assert_eq!(searchbox.input, "a");

        // 测试删除时光标移动
        searchbox.handle_event(KeyEvent::from(KeyCode::Backspace), );
        assert_eq!(searchbox.cursor_position, 0);
        assert_eq!(searchbox.input, "");
    }
}
