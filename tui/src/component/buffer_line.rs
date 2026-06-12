use ratatui::layout::Rect;
use ratatui::prelude::{Line, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Paragraph};
use ratatui::Frame;
use view::editor::EditorModel;

use crate::uiconfig::theme::Theme;

pub fn render_buffer_line(frame: &mut Frame, area: Rect, editor: &EditorModel, theme: &Theme) {
    let bar_style = theme
        .styles
        .get("ui.bufferline")
        .copied()
        .unwrap_or_default();

    frame.render_widget(Block::default().style(bar_style), area);

    let active_style = theme
        .styles
        .get("ui.bufferline.active")
        .copied()
        .unwrap_or(
            bar_style
                .fg
                .map(|fg| Style::default().fg(fg).add_modifier(Modifier::BOLD))
                .unwrap_or_else(|| Style::default().add_modifier(Modifier::BOLD)),
        );
    let inactive_style = theme
        .styles
        .get("ui.bufferline.inactive")
        .copied()
        .unwrap_or(bar_style);
    let sep_style = theme
        .styles
        .get("ui.bufferline.separator")
        .copied()
        .unwrap_or(bar_style);
    let empty_style = theme
        .styles
        .get("ui.bufferline.empty")
        .copied()
        .unwrap_or(inactive_style);

    let labels = editor.open_document_labels();
    let current_id = editor.current_id;

    let mut spans = Vec::new();
    spans.push(Span::styled(" ", bar_style));

    if labels.is_empty() {
        spans.push(Span::styled(" No file open ", empty_style));
    } else {
        for (index, (id, label)) in labels.iter().enumerate() {
            if index > 0 {
                spans.push(Span::styled(" ", sep_style));
                spans.push(Span::styled("│", sep_style));
                spans.push(Span::styled(" ", sep_style));
            }
            let is_active = current_id == Some(*id);
            let tab_style = if is_active {
                active_style
            } else {
                inactive_style
            };
            spans.push(Span::styled(format!(" {label} "), tab_style));
        }
    }

    spans.push(Span::styled(" ", bar_style));
    frame.render_widget(Paragraph::new(Line::from(spans)), area);
}
