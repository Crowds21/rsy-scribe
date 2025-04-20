pub mod component_search_box;
pub mod component_editor;
pub mod gutter;

use std::any::Any;
use std::time::Instant;
use ratatui::{layout::Rect, Frame};
use crossterm::event::KeyEvent;
use ratatui::widgets::Paragraph;
use crate::compositor::{Callback, CompositorContext, EventResult};

pub trait Component: Any + 'static{
    fn render(&mut self, f: &mut Frame, area: Rect){
    }
    fn handle_event(&mut self, event: KeyEvent, context: &mut CompositorContext) -> EventResult{
        EventResult::Ignored(None),
    }
    fn cursor_position(&self, area: Rect) -> Option<(u16, u16)> {
        None
    }
    fn render_loading(&self,frame: &mut Frame) {
        let spinner = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let frame_num = (Instant::now().elapsed().as_millis() / 100) % spinner.len() as u128;
        frame.render_widget(Paragraph::new(spinner[frame_num as usize]), frame.size());
    }

    fn id(&self) -> Option<&'static str> {
        None
    }
    /// 用于向下转型
    fn as_any_mut(&mut self) -> &mut dyn Any;

}
impl<T: Any + 'static> Component for T {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}