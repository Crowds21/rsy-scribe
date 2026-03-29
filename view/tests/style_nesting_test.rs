//! 样式嵌套测试 - 验证 ratatui 是否支持多种样式组合
//!
//! 测试目标：
//! 1. 粗体 + 斜体是否可以同时应用
//! 2. 粗体 + 下划线是否可以同时应用
//! 3. 多种样式组合的渲染效果

use ratatui::{
    backend::TestBackend,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Terminal,
};

/// 测试粗体 + 斜体嵌套
#[test]
fn test_bold_and_italic_nesting() {
    let backend = TestBackend::new(50, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    // 创建同时具有粗体和斜体的文本
    let styled_span = Span::styled(
        "Bold and Italic Text",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC),
    );

    let paragraph = Paragraph::new(Line::from(styled_span));

    terminal
        .draw(|f| {
            f.render_widget(paragraph, f.size());
        })
        .unwrap();

    // 获取渲染后的缓冲区
    let buffer = terminal.backend().buffer();
    
    // 验证第一个字符的样式
    let cell = buffer.get(0, 0);
    let cell_style = cell.style();
    
    // 通过比较样式来验证
    let expected_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);
    
    println!("Buffer cell style: {:?}", cell_style);
    println!("Expected style: {:?}", expected_style);
    
    // 断言：ratatui 应该支持同时应用多种样式
    // 检查修饰符是否正确设置
    assert_eq!(cell_style.add_modifier, expected_style.add_modifier, 
               "修饰符应该匹配");
}

/// 测试粗体 + 下划线嵌套
#[test]
fn test_bold_and_underline_nesting() {
    let backend = TestBackend::new(50, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let styled_span = Span::styled(
        "Bold and Underlined",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::UNDERLINED),
    );

    let paragraph = Paragraph::new(Line::from(styled_span));
    terminal.draw(|f| f.render_widget(paragraph, f.size())).unwrap();

    let buffer = terminal.backend().buffer();
    let cell = buffer.get(0, 0);
    
    let expected_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::UNDERLINED);
    
    println!("Bold+Underline - Cell style: {:?}", cell.style());
    assert_eq!(cell.style().add_modifier, expected_style.add_modifier);
}

/// 测试三种样式嵌套：粗体 + 斜体 + 下划线
#[test]
fn test_three_modifiers_nesting() {
    let backend = TestBackend::new(50, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let styled_span = Span::styled(
        "Bold + Italic + Underline",
        Style::default()
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC)
            .add_modifier(Modifier::UNDERLINED),
    );

    let paragraph = Paragraph::new(Line::from(styled_span));
    terminal.draw(|f| f.render_widget(paragraph, f.size())).unwrap();

    let buffer = terminal.backend().buffer();
    let cell = buffer.get(0, 0);
    
    let expected_style = Style::default()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC)
        .add_modifier(Modifier::UNDERLINED);
    
    println!("Three modifiers - Cell style: {:?}", cell.style());
    assert_eq!(cell.style().add_modifier, expected_style.add_modifier);
}

/// 测试删除线样式
#[test]
fn test_strikethrough_nesting() {
    let backend = TestBackend::new(50, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let styled_span = Span::styled(
        "Strikethrough Text",
        Style::default()
            .add_modifier(Modifier::CROSSED_OUT),
    );

    let paragraph = Paragraph::new(Line::from(styled_span));
    terminal.draw(|f| f.render_widget(paragraph, f.size())).unwrap();

    let buffer = terminal.backend().buffer();
    let cell = buffer.get(0, 0);
    
    let expected_style = Style::default()
        .add_modifier(Modifier::CROSSED_OUT);
    
    println!("Strikethrough - Cell style: {:?}", cell.style());
    assert_eq!(cell.style().add_modifier, expected_style.add_modifier);
}

/// 测试样式叠加（先设置粗体，再设置斜体）
#[test]
fn test_style_accumulation() {
    let backend = TestBackend::new(50, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    // 分步添加样式
    let mut style = Style::default();
    style = style.add_modifier(Modifier::BOLD);
    style = style.add_modifier(Modifier::ITALIC);

    let styled_span = Span::styled("Accumulated Style", style);
    let paragraph = Paragraph::new(Line::from(styled_span));
    terminal.draw(|f| f.render_widget(paragraph, f.size())).unwrap();

    let buffer = terminal.backend().buffer();
    let cell = buffer.get(0, 0);
    
    // 验证两种样式都存在
    let expected = Style::default()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);
    
    assert_eq!(cell.style().add_modifier, expected.add_modifier);
}

/// 测试多个 Span 的不同样式
#[test]
fn test_multiple_spans_different_styles() {
    let backend = TestBackend::new(50, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    let line = Line::from(vec![
        Span::styled("Normal", Style::default()),
        Span::styled("Bold", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled("Italic", Style::default().add_modifier(Modifier::ITALIC)),
        Span::styled("Bold+Italic", 
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC)),
    ]);

    let paragraph = Paragraph::new(line);
    terminal.draw(|f| f.render_widget(paragraph, f.size())).unwrap();

    let buffer = terminal.backend().buffer();
    
    // 验证不同位置的样式
    let normal_cell = buffer.get(0, 0);
    let bold_cell = buffer.get(6, 0);
    let italic_cell = buffer.get(10, 0);
    let both_cell = buffer.get(16, 0);
    
    println!("Normal style: {:?}", normal_cell.style());
    println!("Bold style: {:?}", bold_cell.style());
    println!("Italic style: {:?}", italic_cell.style());
    println!("Both style: {:?}", both_cell.style());
    
    // Bold 位置应该有 BOLD
    assert_eq!(bold_cell.style().add_modifier, Style::default().add_modifier(Modifier::BOLD).add_modifier);
    // Italic 位置应该有 ITALIC
    assert_eq!(italic_cell.style().add_modifier, Style::default().add_modifier(Modifier::ITALIC).add_modifier);
    // Both 位置应该同时有 BOLD 和 ITALIC
    let expected_both = Style::default()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);
    assert_eq!(both_cell.style().add_modifier, expected_both.add_modifier);
}

/// 测试从字符串解析样式（模拟 TextMarkType: "strong em"）
#[test]
fn test_parse_style_from_string() {
    // 模拟从 "strong em" 解析样式
    let style_str = "strong em";
    let mut style = Style::default();
    
    for part in style_str.split_whitespace() {
        match part {
            "strong" | "bold" => style = style.add_modifier(Modifier::BOLD),
            "em" | "italic" => style = style.add_modifier(Modifier::ITALIC),
            "u" | "underline" => style = style.add_modifier(Modifier::UNDERLINED),
            "s" | "strike" => style = style.add_modifier(Modifier::CROSSED_OUT),
            _ => {}
        }
    }
    
    let styled_span = Span::styled("Parsed Style", style);
    let paragraph = Paragraph::new(Line::from(styled_span));
    
    let backend = TestBackend::new(50, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|f| f.render_widget(paragraph, f.size())).unwrap();

    let buffer = terminal.backend().buffer();
    let cell = buffer.get(0, 0);
    
    // 验证解析后的样式正确应用
    let expected = Style::default()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);
    
    assert_eq!(cell.style().add_modifier, expected.add_modifier, 
               "应该同时包含粗体和斜体");
    
    println!("✓ Successfully parsed 'strong em' and applied both styles");
}

/// 测试样式冲突（BOLD 和 DIM 是互斥的）
#[test]
fn test_conflicting_modifiers() {
    let backend = TestBackend::new(50, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    // BOLD 和 DIM 是互斥的，后应用的会覆盖先应用的
    let style = Style::default()
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::DIM);

    let styled_span = Span::styled("Conflicting", style);
    let paragraph = Paragraph::new(Line::from(styled_span));
    terminal.draw(|f| f.render_widget(paragraph, f.size())).unwrap();

    let buffer = terminal.backend().buffer();
    let cell = buffer.get(0, 0);
    
    // 注意：BOLD 和 DIM 互斥，最终效果取决于终端实现
    println!("Conflicting modifiers test completed");
    println!("Cell style: {:?}", cell.style());
}
