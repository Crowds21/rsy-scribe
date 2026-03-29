//! 样式嵌套可视化测试工具
//! 
//! 运行方式：cargo run --bin test_styles
//! 
//! 这会在真实终端中展示各种样式组合的效果

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 设置终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 运行主循环
    let result = run_app(&mut terminal);

    // 恢复终端
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;

    result
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| {
            let area = f.size();
            
            // 分割布局
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![
                    Constraint::Length(3),  // 标题
                    Constraint::Length(3),  // 单一粗体
                    Constraint::Length(3),  // 单一斜体
                    Constraint::Length(3),  // 粗体 + 斜体
                    Constraint::Length(3),  // 粗体 + 下划线
                    Constraint::Length(3),  // 三样式组合
                    Constraint::Length(3),  // 删除线
                    Constraint::Length(3),  // 高亮
                    Constraint::Length(3),  // 行内代码
                    Constraint::Min(1),     // 底部提示
                ])
                .split(area);

            // 标题
            let title = Paragraph::new("Ratatui 样式嵌套测试 - 按 'q' 退出")
                .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // 1. 单一粗体
            let bold = Paragraph::new(Line::from(
                Span::styled("这是粗体文本 (BOLD)", Style::default().add_modifier(Modifier::BOLD))
            ))
            .block(Block::default().title("粗体").borders(Borders::ALL));
            f.render_widget(bold, chunks[1]);

            // 2. 单一斜体
            let italic = Paragraph::new(Line::from(
                Span::styled("这是斜体文本 (ITALIC)", Style::default().add_modifier(Modifier::ITALIC))
            ))
            .block(Block::default().title("斜体").borders(Borders::ALL));
            f.render_widget(italic, chunks[2]);

            // 3. 粗体 + 斜体
            let bold_italic = Paragraph::new(Line::from(
                Span::styled("粗体 + 斜体 (BOLD + ITALIC)", 
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC))
            ))
            .block(Block::default().title("粗体 + 斜体").borders(Borders::ALL));
            f.render_widget(bold_italic, chunks[3]);

            // 4. 粗体 + 下划线
            let bold_underline = Paragraph::new(Line::from(
                Span::styled("粗体 + 下划线 (BOLD + UNDERLINE)", 
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::UNDERLINED))
            ))
            .block(Block::default().title("粗体 + 下划线").borders(Borders::ALL));
            f.render_widget(bold_underline, chunks[4]);

            // 5. 三样式组合
            let triple = Paragraph::new(Line::from(
                Span::styled("粗体 + 斜体 + 下划线 (BOLD + ITALIC + UNDERLINE)", 
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC)
                        .add_modifier(Modifier::UNDERLINED))
            ))
            .block(Block::default().title("三样式组合").borders(Borders::ALL));
            f.render_widget(triple, chunks[5]);

            // 6. 删除线
            let strike = Paragraph::new(Line::from(
                Span::styled("这是删除线文本 (CROSSED_OUT)", 
                    Style::default().add_modifier(Modifier::CROSSED_OUT))
            ))
            .block(Block::default().title("删除线").borders(Borders::ALL));
            f.render_widget(strike, chunks[6]);

            // 7. 高亮
            let mark = Paragraph::new(Line::from(
                Span::styled("这是高亮文本 (MARK)", 
                    Style::default().bg(Color::Yellow).fg(Color::Black))
            ))
            .block(Block::default().title("高亮").borders(Borders::ALL));
            f.render_widget(mark, chunks[7]);

            // 8. 行内代码
            let code = Paragraph::new(Line::from(
                Span::styled("let x = 10; // 行内代码", 
                    Style::default().fg(Color::Green).bg(Color::DarkGray))
            ))
            .block(Block::default().title("行内代码").borders(Borders::ALL));
            f.render_widget(code, chunks[8]);

            // 底部提示
            let help = Paragraph::new("按 'q' 退出 | 测试 ratatui 是否支持多种样式同时显示")
                .style(Style::default().fg(Color::Gray))
                .block(Block::default().borders(Borders::TOP));
            f.render_widget(help, chunks[9]);
        })?;

        // 处理事件
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
