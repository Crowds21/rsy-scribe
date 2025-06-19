// 参考 Helix 实现的 UI 调度器
use crate::component::editor::EditorView;
use crate::component::Component;
use crate::model::editor_model::EditorModel;
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
    area: Rect,
}
/// 全局状态管理
pub struct CompositorContext {
    pub theme: Theme,
    pub editor_model: EditorModel,
    pub scroll: Option<usize>,
}

impl<'a> Compositor {
    pub fn new(area: Rect) -> Compositor {
        let editor: Box<dyn Component> = Box::new(EditorView::new());
        let layers = vec![editor];
        Self { layers, area }
    }
    /// UI 组合器从下往上逐层绘制组件
    /// TODO 如果事件被顶层 Layer 消费,
    ///     并且不涉及异步更新 UI 的操作,就可以不重绘下层 UI
    pub fn render(&mut self, frame: &mut Frame, surface: Rect, cx: &mut CompositorContext) {
        for layer in &mut self.layers {
            layer.render(frame, self.area, cx);
        }
    }

    /// 传递事件. 顶层组件未处理的事件会传向下一层.
    pub fn handle_event(
        &mut self,
        event: &Event,
        cx: &mut CompositorContext,
    ) -> bool {
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
    /// 从顶层开始逐层尝试获取光标位置
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

    pub fn resize(&mut self, area: Rect) {
        self.area = area;
    }
}

impl CompositorContext {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            editor_model: EditorModel::default(),
            scroll: None,
        }
    }
}
