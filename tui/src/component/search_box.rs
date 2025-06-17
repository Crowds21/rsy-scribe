mod search_box_debounce;

use super::*;
use crate::component::editor::EditorView;
use crate::component::search_box::search_box_debounce::SearchBoxDebounce;
use crate::compositor::{Compositor, CompositorContext, EventResult};
use crate::debounce::{send_blocking, AsyncHook};
use crate::job::dispatch;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    style::{Modifier, Style},
    widgets::*,
};
use syservice;
use tokio::sync::mpsc::Sender;
use unicode_width::UnicodeWidthStr;

pub const ID: &str = "search-box";
/// 可搜索的文本框组件
#[derive(Debug)]
pub struct SearchBox {
    cursor_position: usize,
    /// 当前输入内容
    input: String,
    /// 搜索结果显示列表
    results: Vec<SearchResultItem>,
    /// Ratatui ui 状态
    list_state: ListState,
    selected_result: Option<usize>,
    /// 输入框标题
    title: String,

    pub width: u16,
    pub height: u16,

    /// 延时搜索
    async_sender: Sender<String>,
}
/// 摘取SiYuan数据库字段
#[derive(Debug)]
struct SearchResultItem {
    pub id: String,
    pub box_id: String,
    pub content: String,
    pub path: String,
    pub hpath: String,
}
impl<'a> Default for SearchBox {
    fn default() -> Self {
        let search_debounce = SearchBoxDebounce::new();
        // TODO 在这里手动设置 debounce 中的异步逻辑
        let sender = search_debounce.spawn();
        Self {
            cursor_position: 0,
            input: String::new(),
            results: vec![],
            list_state: ListState::default(),
            selected_result: None,
            title: "Search".to_string(),
            width: 0,
            height: 0,
            async_sender: sender,
        }
    }
}
impl Component for SearchBox {
    fn render(&mut self, frame: &mut Frame, area: Rect, cx: &mut CompositorContext) {
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
            .border_style(Style::default());

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
            .map(|r| ListItem::new(r.hpath.as_str()))
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

    fn handle_event(&mut self, event: &Event, cx: &mut CompositorContext) -> EventResult {
        match event {
            Event::Key(e) => self.handle_key_event(e, cx),
            _ => EventResult::Consumed(None),
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
            ..Default::default()
        }
    }
    /// 获取当前输入内容
    pub fn input(&self) -> &str {
        &self.input
    }

    /// 获取当前选中的结果
    pub fn selected_result(&self) -> Option<&String> {
        self.selected_result
            .and_then(|i| self.results.get(i))
            .map(|item| &item.hpath)
    }
    /// 清除选中状态
    pub fn clear_selection(&mut self) {
        self.selected_result = None;
    }
    /// 处理用户字符输入.
    fn handle_search_input(&mut self, c: char) -> EventResult {
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

    fn load_document(&mut self) -> EventResult {
        let doc_path = match self.selected_result.and_then(|idx| self.results.get(idx)) {
            Some(doc_info) => format!("{}{}", doc_info.box_id, doc_info.path),
            None => return EventResult::Consumed(None), // 提前返回避免无效spawn
        };
        tokio::spawn(async move {
            let sy_nodes = syservice::file::load_json_node(&doc_path);
            let open_document = move |compositor: &mut Compositor| {
                let component = compositor.find::<EditorView>();
                if let Some(editorView) = component {
                    if let Ok(node) = sy_nodes {
                        editorView.document = Some(node);
                        compositor.pop();
                        // TODO 这里还需要进行计算操作
                        //  每个元素组件占据多少 offset.
                        //  以便处理窗口滑动
                    }
                }
            };
            dispatch(open_document).await
        });
        EventResult::Consumed(None)
    }
    fn handle_key_event(&mut self, event: &KeyEvent, cx: &mut CompositorContext) -> EventResult {
        match event.code {
            KeyCode::Char(c) => self.handle_search_input(c),
            KeyCode::Backspace => self.handle_delete_char(),
            KeyCode::Delete => self.handle_delete_char(),
            KeyCode::Left => {
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
            KeyCode::Right => {
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
            KeyCode::End => {
                self.cursor_position = self.input.len();
                EventResult::Consumed(None)
            }
            KeyCode::Enter => self.load_document(),
            KeyCode::Down => {
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
            _ => EventResult::Ignored(None),
        }
    }
}
