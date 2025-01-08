use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph};
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

pub fn center_box(mut frame: Frame, input: String) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Length(3),
            Constraint::Percentage(50),
        ])
        .split(frame.size());

    let input_paragraph = Paragraph::new(Text::from(input.as_str()))
        .block(Block::default().borders(Borders::ALL).title("Input"));

    frame.render_widget(input_paragraph, chunks[1])
}
