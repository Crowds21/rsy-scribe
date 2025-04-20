use super::*;
use crate::compositor::{Compositor, CompositorContext, EventResult};
use crossterm::event::KeyEvent;
use ratatui::{
    prelude::*,
    style::{Modifier, Style},
    widgets::*,
};
use syservice;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use unicode_width::UnicodeWidthStr;

pub const ID: &str = "search-box";
/// 可搜索的文本框组件
#[derive(Debug)]
pub struct SearchBox<'a> {
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
    title: &'a str,
    block: Block<'a>,

    pub width: u16,
    pub height: u16,
    last_input_time: Instant,
    search_tx: Option<mpsc::Sender<String>>, // 用于取消之前的搜索
    current_search_id: u64,                  // 用于标识当前搜索请求
    is_searching: bool,
}
impl<'a> Default for SearchBox<'a> {
    fn default() -> Self {
        Self {
            cursor_position: 0,
            input: String::new(),
            items: vec!["[输入搜索内容]".to_string()],
            results: vec![],
            list_state: ListState::default(),
            selected_result: None,
            active: false,
            title: "Search",
            block: Block::default(),
            width: 0,
            height: 0,
            last_input_time: Instant::now(),
            search_tx: None,
            current_search_id: 0,
            is_searching: false,
        }
    }
}
impl Component for SearchBox<'_> {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
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
            .title(self.title)
            .borders(Borders::ALL)
            .border_style(if self.active {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        // 计算光标在屏幕上的位置
        let cursor_x = self.input[..self.cursor_position].width() as u16 + 1;

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

    fn handle_event(&mut self, event: KeyEvent, context: &mut CompositorContext) -> EventResult {
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
            crossterm::event::KeyCode::Enter => self.perform_search(),
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
            crossterm::event::KeyCode::Esc => {
                let callback: crate::compositor::Callback = Box::new(
                    move |compositor: &mut Compositor, cx: &mut CompositorContext| {
                        compositor.pop();
                    },
                );
                return EventResult::Consumed(Some(callback));
            }
            _ => return EventResult::Ignored(None),
        };
        EventResult::Consumed(None)
    }

    fn cursor_position(&self, area: Rect) -> Option<(u16, u16)> {
        todo!()
    }

    fn id(&self) -> Option<&'static str> {
        Some(ID)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        todo!()
    }
}
impl<'a> SearchBox<'a> {
    /// 创建新的SearchBox
    pub fn new(title: &'a str, results_title: &'a str) -> Self {
        Self {
            title,
            items: vec!["[输入搜索内容]".to_string()],
            last_input_time: Instant::now(),
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
    /// 处理输入变化（在handle_event的Char/Backspace/Delete等分支调用）
    fn on_input_change(&mut self, cx: &mut CompositorContext) {
        self.last_input_time = Instant::now();
        self.current_search_id += 1;
        self.is_searching = true;

        
        // 设置延迟搜索
        let (new_tx, mut rx) = mpsc::channel(1);
        self.search_tx = Some(new_tx);
        let search_id = self.current_search_id;
        let input = self.input.clone();

        // 启动延迟搜索任务
        tokio::spawn(async move {
            // 阶段1：等待延迟
            let delay_future = tokio::time::sleep(Duration::from_millis(1500));
            tokio::pin!(delay_future);

            tokio::select! {
                _ = delay_future => {
                    // 阶段2：执行API请求
                    let result = syservice::document::search_doc_with_title(input).await;
                    let mut resp = vec![];
                    if let Ok(vec_result) = result {
                        resp = vec_result.data.iter()
                            .map(|it| it.content)
                            .collect::<Vec<_>>();
                    }
                    // 回调到UI线程
                    cx.callback().invoke(move |comp| {
                        if let Some(search_box) = comp.find_mut::<SearchBox>() {
                            if search_box.current_search_id == search_id {
                                search_box.update_results(resp);
                                search_box.is_searching = false;
                            }
                        }
                    });
                }
                _ = cancel_rx.recv() => {
                    // 收到取消信号，直接退出
                }
            }
        });
    }

    /// 执行实际异步搜索
    fn do_async_search(&mut self, query: String, cx: &mut CompositorContext) {
        self.is_searching = true;
        let search_id = self.current_search_id;
        let callback = cx.callback();

        tokio::spawn(async move {
            // 实际API调用
            let result = syservice::document::search_doc_with_title(query.clone()).await;

            // 回调到UI线程
            callback.invoke(move |comp| {
                if let Some(search_box) = comp.find_mut::<SearchBox>() {
                    search_box.is_searching = false;

                    // 只处理最新请求的结果
                    if search_box.current_search_id == search_id {
                        match result {
                            Ok(res) => search_box.update_results(
                                res.data.iter().map(|d| d.content.clone()).collect(),
                            ),
                            Err(e) => search_box.update_results(vec![format!("搜索失败: {}", e)]),
                        }
                    }
                }
            });
        });
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
