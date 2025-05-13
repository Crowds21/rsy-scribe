pub mod search_box;
pub mod editor;
pub mod gutter;
mod block;

use std::any::Any;
use std::time::Instant;
use ratatui::{layout::Rect, Frame};
use crossterm::event::KeyEvent;
use ratatui::widgets::Paragraph;
use crate::compositor::{Callback, CompositorContext, EventResult};

pub trait Component: Any + AnyComponent{
    fn render(&mut self, f: &mut Frame, area: Rect, cx: &mut CompositorContext);
    fn handle_event(&mut self, event: KeyEvent, context: &mut CompositorContext) -> EventResult{
        EventResult::Ignored(None)
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
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// A view that can be downcasted to its concrete type.
///
/// This trait is automatically implemented for any `T: Component`.
pub trait AnyComponent {
    /// Downcast self to a `Any`.
    fn as_any(&self) -> &dyn Any;

    /// Downcast self to a mutable `Any`.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Returns a boxed any from a boxed self.
    ///
    /// Can be used before `Box::downcast()`.
    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any>;
}

impl<T: Component> AnyComponent for T {
    /// Downcast self to a `Any`.
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Downcast self to a mutable `Any`.
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_boxed_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
impl dyn AnyComponent {
    /// Attempts to downcast `self` to a concrete type.
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    /// Attempts to downcast `self` to a concrete type.
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }

    /// Attempts to downcast `Box<Self>` to a concrete type.
    pub fn downcast<T: Any>(self: Box<Self>) -> Result<Box<T>, Box<Self>> {
        // Do the check here + unwrap, so the error
        // value is `Self` and not `dyn Any`.
        if self.as_any().is::<T>() {
            Ok(self.as_boxed_any().downcast().unwrap())
        } else {
            Err(self)
        }
    }

    /// Checks if this view is of type `T`.
    pub fn is<T: Any>(&mut self) -> bool {
        self.as_any().is::<T>()
    }
}
