// 参考 Helix 实现的 UI 调度器
use crate::editor::EditorView;
use crate::job::Jobs;
use crate::searchbox::SearchBox;
use crate::Component;
use crossterm::event::{Event, KeyEvent};
use ratatui::{
    prelude::*,
    style::{Modifier, Style},
    widgets::*,
};

/// 回调
pub type Callback = Box<dyn FnOnce(&mut Compositor, &mut CompositorContext)>;

pub enum EventResult {
    Ignored(Option<Callback>),
    Consumed(Option<Callback>),
}
/// UI 组合器
pub struct Compositor {
    layers: Vec<Box<dyn Component>>,
}

/// 全局状态管理
pub struct CompositorContext {
    // pub jobs: Jobs
}
impl CompositorContext {
    pub fn new() -> Self {
        Self {}
    }
}
impl Compositor {
    pub fn new() -> Self {
        let editor: Box<dyn Component> = Box::new(EditorView::new());
        let layers = vec![editor];
        Self { layers }
    }
    pub fn render(&mut self, frame: &mut Frame, surface: Rect) {
        for layer in &mut self.layers {
            layer.render(frame, surface);
        }
    }

    pub fn handle_event(&mut self, event: KeyEvent, cx: &mut CompositorContext) -> bool {

        let mut callbacks = Vec::new();
        for layer in self.layers.iter_mut().rev() {
            match layer.handle_event(event, cx) {
                EventResult::Consumed(Some(callback)) => {
                    callbacks.push(callback);
                }
                EventResult::Consumed(None) => {}
                _ => {}
            }
        }
        //由于 Rust 的
        for callback in callbacks {
            callback(self, cx)
        }
        false
    }

    pub fn cursor_position(&self, area: Rect) -> Option<(u16, u16)> {
        for layer in self.layers.iter().rev() {
            if let Some(pos) = layer.cursor_position(area) {
                return Some(pos);
            }
        }
        None
    }

    pub fn push(&mut self, mut layer: Box<dyn Component>) {
        self.layers.push(layer);
    }

    pub fn pop(&mut self) -> Option<Box<dyn Component>> {
        self.layers.pop()
    }
}
