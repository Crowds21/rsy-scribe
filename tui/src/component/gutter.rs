use ratatui::layout::Rect;
use ratatui::prelude::{Line, Style};
use ratatui::text::Span;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use view::document::DocumentLine;

use crate::uiconfig::Icons;

/// 编辑器侧边栏
pub struct GutterConfig {
    /// 侧边栏展示顺序(从左到右)
    pub layout: Vec<GutterType>,
}

pub enum GutterType {
    /// 行号
    LineNumber,
    /// 块类型 nerd 图标（仅块首行）
    Icon,
    /// 空白列
    Spacer,
}

impl Default for GutterConfig {
    fn default() -> Self {
        Self {
            layout: vec![GutterType::LineNumber, GutterType::Icon],
        }
    }
}

fn line_number_width(total_lines: usize) -> u16 {
    let digits = (total_lines.max(1) as f64).log10().floor() as u16 + 1;
    digits.max(3) + 1
}

pub fn gutter_total_width(total_lines: usize, config: &GutterConfig) -> u16 {
    config
        .layout
        .iter()
        .map(|t| t.column_width(total_lines))
        .sum()
}

pub fn render_gutter(
    frame: &mut Frame,
    area: Rect,
    config: &GutterConfig,
    doc_lines: &[DocumentLine],
    scroll_offset: u16,
    icons: &Icons,
    gutter_style: Style,
    line_number_style: Style,
    icon_style: Style,
) {
    use ratatui::widgets::Block;

    // Gutter 背景，与正文区域区分
    frame.render_widget(Block::default().style(gutter_style), area);

    let height = area.height as usize;
    let total_lines = doc_lines.len();
    let mut x_offset = area.x;

    for gutter_type in &config.layout {
        let width = gutter_type.column_width(total_lines);
        let gutter_area = Rect {
            x: x_offset,
            y: area.y,
            width,
            height: area.height,
        };

        let paragraph = match gutter_type {
            GutterType::LineNumber => {
                let num_width = line_number_width(total_lines) - 1;
                let mut lines = Vec::with_capacity(height);
                for visual_line in 0..height {
                    let doc_idx = scroll_offset as usize + visual_line;
                    if doc_idx < total_lines {
                        let num = doc_idx + 1;
                        let text = format!("{:>width$} ", num, width = num_width as usize);
                        lines.push(Line::from(Span::styled(text, line_number_style)));
                    } else {
                        lines.push(Line::from(Span::styled("", line_number_style)));
                    }
                }
                Paragraph::new(lines)
            }
            GutterType::Icon => {
                let mut lines = Vec::with_capacity(height);
                for visual_line in 0..height {
                    let doc_idx = scroll_offset as usize + visual_line;
                    let icon = doc_lines
                        .get(doc_idx)
                        .and_then(|line| icons.for_document_line(line))
                        .unwrap_or(" ");
                    lines.push(Line::from(Span::styled(format!("{icon} "), icon_style)));
                }
                Paragraph::new(lines)
            }
            GutterType::Spacer => {
                let text = " \n".repeat(height.saturating_sub(1));
                Paragraph::new(text).style(gutter_style)
            }
        };

        frame.render_widget(paragraph, gutter_area);
        x_offset += width;
    }
}

impl GutterType {
    fn column_width(&self, total_lines: usize) -> u16 {
        match self {
            GutterType::LineNumber => line_number_width(total_lines),
            GutterType::Icon => 2,
            GutterType::Spacer => 1,
        }
    }
}
