// 参考 Helix 实现的 UI 调度器
use crate::component::editor::EditorView;
use crate::component::Component;
use crate::uiconfig::theme::Theme;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use ratatui::prelude::*;
use view::document::DocumentId;
use view::editor::EditorModel;

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
    pub layers: Vec<Box<dyn Component>>,
    // 其他元素的 Area 应该都是通过这个 Rect
    // 以固定的方式计算得出的.
    pub area: Rect,
}
/// 全局状态管理
pub struct CompositorContext<'a> {
    pub editor_model: &'a mut EditorModel,
    pub theme: Theme,
    /// 偏移量. 即从第几行开始展示原文档
    /// TODO Helix command::scroll
    pub scroll: Option<u16>,
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
    pub fn handle_event(&mut self, event: &Event, cx: &mut CompositorContext) -> bool {
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
impl CompositorContext<'_> {
    /// 基于光标的移动大小,来计算是否需要向上滑动屏幕. 
    /// # 参数列表
    /// - movement: 光标向上移动的行数（正数）
    pub fn scroller_backward(&mut self, movement: u16) {
        let current_scroll = self.scroll.unwrap_or_default();

        match self.editor_model.current_id {
            None => return,
            Some(id) => {
                // 获取文档总高度
                let doc_height = self.editor_model.get_current_doc_height();

                // 计算新的滚动位置（确保不会滚动到负值）
                let new_scroll = current_scroll.saturating_sub(movement);

                // 更新滚动位置
                self.scroll = Some(new_scroll);
            }
        }
    }

    /// 基于光标的移动位置,向下滑动屏幕
    /// # 参数列表
    /// - movement: 光标向下移动的行数（正数）
    pub fn scroller_forward(&mut self, movement: u16, area_height: u16) {
        let current_scroll = self.scroll.unwrap_or_default();

        match self.editor_model.current_id {
            None => return,
            Some(id) => {
                // 获取文档总高度
                let doc_height = self.editor_model.get_current_doc_height();

                // 计算最大可滚动位置
                let max_scroll = if doc_height > area_height {
                    doc_height - area_height
                } else {
                    0
                };

                // 计算新的滚动位置（确保不会超过最大滚动位置）
                let new_scroll = current_scroll.saturating_add(movement);
                let clamped_scroll = new_scroll.min(max_scroll);

                // 更新滚动位置
                self.scroll = Some(clamped_scroll);
            }
        }
    }

    /// 调用该函数时,光标应保持在屏幕中的位置不动,
    /// 而移动整个展示窗口
    /// # 参数列表
    /// - lines: 要滚动的行数，正数表示向下滚动，负数表示向上滚动
    pub fn scroller_screen(&mut self, lines: i16, area_height: u16) {
        let current_scroll = self.scroll.unwrap_or_default();
        match self.editor_model.current_id {
            None => (),
            Some(id) => {
                let doc_height = self.editor_model.get_current_doc_height();
                // 计算最大可滚动位置
                let max_scroll = doc_height.saturating_sub(area_height);

                // 根据移动方向计算新的滚动位置
                let new_scroll = if lines >= 0 {
                    // 向下滚动
                    current_scroll.saturating_add(lines as u16)
                } else {
                    // 向上滚动（lines 为负值）
                    current_scroll.saturating_sub(lines.unsigned_abs())
                };
                // 确保滚动位置在有效范围内
                let clamped_scroll = new_scroll.clamp(0, max_scroll);
                self.scroll = Some(clamped_scroll);
            }
        }
    }

    /// 向上滚动一页
    pub fn scroll_page_up(&mut self, area_height: u16) {
        self.scroller_screen(-(area_height as i16), area_height);
    }

    /// 向下滚动一页
    pub fn scroll_page_down(&mut self, area_height: u16) {
        self.scroller_screen(area_height as i16, area_height);
    }

    /// 确保光标在可视区域内，必要时调整滚动
    pub fn ensure_cursor_in_viewport(
        &mut self,
        cursor_line: u16,
        cursor_column: u16,
        viewport_height: u16,
        viewport_width: u16
    ) -> bool {
        let current_scroll = self.scroll.unwrap_or_default();
        let mut scrolled = false;

        match self.editor_model.current_id {
            None => return false,
            Some(id) => {
                // 获取文档总高度
                let doc_height = self.editor_model.get_current_doc_height();

                // 垂直滚动检查
                if cursor_line < current_scroll {
                    // 光标在视口上方，向上滚动
                    self.scroll = Some(cursor_line);
                    scrolled = true;
                } else if cursor_line >= current_scroll + viewport_height {
                    // 光标在视口下方，向下滚动
                    let new_scroll = cursor_line - viewport_height + 1;
                    let max_scroll = if doc_height > viewport_height {
                        doc_height - viewport_height
                    } else {
                        0
                    };
                    self.scroll = Some(new_scroll.min(max_scroll));
                    scrolled = true;
                }

                // TODO: 水平滚动检查（如果需要）
                // 可以添加类似的水平滚动逻辑

                scrolled
            }
        }
    }

    /// 获取当前滚动位置
    pub fn get_scroll(&self) -> u16 {
        self.scroll.unwrap_or_default()
    }

    /// 设置滚动位置
    pub fn set_scroll(&mut self, scroll: u16) {
        self.scroll = Some(scroll);
    }

    /// 重置滚动位置到顶部
    pub fn scroll_to_top(&mut self) {
        self.scroll = Some(0);
    }

    /// 滚动到底部
    pub fn scroll_to_bottom(&mut self, area_height: u16) {
        match self.editor_model.current_id {
            None => return,
            Some(id) => {
                let doc_height = self.editor_model.get_current_doc_height();
                if doc_height > area_height {
                    self.scroll = Some(doc_height - area_height);
                } else {
                    self.scroll = Some(0);
                }
            }
        }
    }

    /// 计算光标在视口中的相对位置
    pub fn get_relative_cursor_position(&self, cursor_line: u16, cursor_column: u16) -> (u16, u16) {
        let scroll = self.scroll.unwrap_or_default();
        let relative_line = cursor_line.saturating_sub(scroll);
        (cursor_column, relative_line)
    }

    /// 检查是否可以向下滚动
    pub fn can_scroll_down(&self, area_height: u16) -> bool {
        match self.editor_model.current_id {
            None => false,
            Some(id) => {
                let doc_height = self.editor_model.get_current_doc_height();
                let current_scroll = self.scroll.unwrap_or_default();
                current_scroll + area_height < doc_height
            }
        }
    }

    /// 检查是否可以向上滚动
    pub fn can_scroll_up(&self) -> bool {
        self.scroll.unwrap_or_default() > 0
    }
}
