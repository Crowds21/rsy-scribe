use ratatui::{
    text::{Line, Span},
    widgets::{Paragraph, Borders, Block},
    style::{Modifier, Style},
};

fn render_header(text: &str, level: u8) -> Paragraph {
    let content = Span::styled(text, Style::default().add_modifier(Modifier::BOLD));
    match level {
        1 => {
            // H1: 双实线包围 + 上下双横线
            let line = "═".repeat(text.len() + 2);
            Paragraph::new(vec![
                Line::from(line.clone()),
                Line::from(vec![content]),
                Line::from(line),
            ])
                .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
        },
        2..=3 => {
            // H2-H3: 单实线下划线
            Paragraph::new(vec![
                Line::from(content),
                Line::from("─".repeat(text.len())), // 实线
            ])
        },
        _ => {
            // H4-H6: 虚线下划线
            Paragraph::new(vec![
                Line::from(content),
                Line::from("﹏".repeat(text.len())), // 中文虚线符号
            ])
        }
    }
}

