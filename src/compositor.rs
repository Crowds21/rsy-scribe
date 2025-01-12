mod searchBox;

use crate::editor::Editor;
use crossterm::event::Event;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};
use ratatui::Frame;

#[derive(Debug)]
/// Info box used in editor. Rendering logic will be in other crate.
pub struct Info {
    /// Title shown at top.
    pub title: String,
    /// Text body, should contain newlines.
    pub text: String,
    /// Body width.
    pub width: u16,
    /// Body height.
    pub height: u16,
}

pub struct Context<'a> {
    pub editor: &'a mut Ed,
}

pub enum EventResult {
    Ignored,
    Consumed,
}

pub trait Component {
    /// 每个组件的事件处理
    fn component_event(&mut self, _event: &Event) {}

    /// 渲染组件
    fn render(&mut self, area: Rect, surface: &mut Buffer, cx: &mut Context);

    fn cursor_position(&self, area: Rect, ctx: &Editor) -> Option<(u16, u16)> {
        None
    }
}
pub struct Compositor {
    layers: Vec<Box<dyn Component>>,
    area: Rect,

    pub(crate) last_picker: Option<Box<dyn Component>>,
    pub(crate) full_redraw: bool,
}
impl Compositor {
    pub fn new(area: Rect) -> Self {
        Self {
            layers: Vec::new(),
            area,
            last_picker: None,
            full_redraw: false,
        }
    }
    pub fn render(&mut self, area: Rect, surface: &mut Buffer, cx: &mut Context) {
        for layer in &mut self.layers {
            layer.render(area, surface, cx);
        }
    }
    /// 组合器处理用户的各类输入
    pub fn handle_event(&mut self, event: &Event, cx: &mut Context) -> bool {
        for layer in self.layers.iter_mut().rev() {
            match layer.component_event(event) {
                //
                _ => {}
            }
        }
        false
    }

    /// return: row, col
    pub fn cursor_position(&self, area: Rect, editor: &Editor) -> Option<(u16, u16)> {
        for layer in self.layers.iter().rev() {
            if let Some(pos) = layer.cursor_position(area, editor) {
                return Some(pos);
            }
        }
        None
    }

    pub fn resize(&mut self, area: Rect) {
        self.area = area;
    }
}
