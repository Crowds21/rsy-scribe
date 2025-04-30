use std::any::Any;
// 参考 Helix 实现的 UI 调度器
use crate::component::component_editor::EditorView;
use crate::component::Component;
use crate::uiconfig::theme::Theme;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use ratatui::prelude::*;

/// 回调
pub type Callback = Box<dyn FnOnce(&mut Compositor, &mut CompositorContext)>;
pub type EditorCompositorCallback = Box<dyn FnOnce(&mut Compositor) + Send>;
pub enum EventResult {
    /// 交由下一层 ui 处理
    Ignored(Option<Callback>),
    /// 表示上一层已经处理
    Consumed(Option<Callback>),
}
/// UI 组合器
pub struct Compositor {
    layers: Vec<Box<dyn Component>>,
}
/// 全局状态管理
pub struct CompositorContext {
    theme: Theme,
}

impl Compositor {
    pub fn new() -> Compositor {
        let editor: Box<dyn Component> = Box::new(EditorView::new());
        let layers = vec![editor];
        Self { layers }
    }
    /// UI 组合器从下往上逐层绘制组件
    pub fn render(&mut self, frame: &mut Frame, surface: Rect) {
        for layer in &mut self.layers {
            layer.render(frame, surface);
        }
    }

    /// 传递事件. 顶层组件未处理的事件会传向下一层.
    pub fn handle_event(&mut self, event: KeyEvent, cx: &mut CompositorContext) -> bool {
        let mut callbacks = Vec::new();
        for layer in self.layers.iter_mut().rev() {
            match layer.handle_event(event, cx) {
                EventResult::Consumed(Some(callback)) => {
                    callbacks.push(callback);
                    break;
                }
                EventResult::Consumed(None) => break,
                _ => {}
            }
        }
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

    pub fn find<T: 'static>(&mut self) -> Option<&mut T> {
        let type_name = std::any::type_name::<T>();
        self.layers
            .iter_mut()
            .find(|component| component.type_name() == type_name)
            .and_then(|component| component.as_any_mut().downcast_mut())
    }
}

impl CompositorContext {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
}
