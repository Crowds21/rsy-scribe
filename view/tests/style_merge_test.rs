//! 样式合并测试 - 验证多个样式列表的合并逻辑
//!
//! 测试目标：
//! 1. 装饰样式 + 基础样式的合并（如 bold + highlight）
//! 2. 多个修饰符是否正确累加
//! 3. 颜色是否正确覆盖

use ratatui::{
    backend::TestBackend,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Terminal,
};

/// 模拟主题样式表
fn create_test_theme() -> std::collections::HashMap<String, Style> {
    let mut theme = std::collections::HashMap::new();
    
    // 基础样式
    theme.insert("node.text.strong".to_string(), 
        Style::default().add_modifier(Modifier::BOLD));
    theme.insert("node.text.italic".to_string(), 
        Style::default().add_modifier(Modifier::ITALIC));
    theme.insert("node.text.underline".to_string(), 
        Style::default().add_modifier(Modifier::UNDERLINED));
    theme.insert("node.text.mark".to_string(), 
        Style::default().bg(Color::Yellow));
    theme.insert("node.text.code".to_string(), 
        Style::default().fg(Color::Green).bg(Color::Rgb(38, 41, 44)));
    
    theme
}

/// 合并多个样式（模拟 editor.rs 中的逻辑）
fn merge_styles(style_names: &[String], theme: &std::collections::HashMap<String, Style>) -> Style {
    let mut style = Style::default();
    for name in style_names {
        if let Some(theme_style) = theme.get(name) {
            style = style
                .add_modifier(theme_style.add_modifier)
                .remove_modifier(theme_style.sub_modifier);
            if theme_style.fg.is_some() {
                style = style.fg(theme_style.fg.unwrap());
            }
            if theme_style.bg.is_some() {
                style = style.bg(theme_style.bg.unwrap());
            }
        }
    }
    style
}

/// 测试：bold + highlight（修饰符 + 背景色）
#[test]
fn test_bold_and_highlight() {
    let theme = create_test_theme();
    let styles = vec![
        "node.text.mark".to_string(),    // 黄色背景
        "node.text.strong".to_string(),  // 粗体
    ];
    
    let merged = merge_styles(&styles, &theme);
    
    // 验证：应该有粗体修饰符和黄色背景
    assert!(merged.add_modifier.contains(Modifier::BOLD), 
            "应该包含粗体修饰符");
    assert_eq!(merged.bg, Some(Color::Yellow), 
            "背景色应该是黄色");
    
    println!("✓ bold + highlight: {:?}", merged);
}

/// 测试：highlight + underline（背景色 + 下划线）
#[test]
fn test_highlight_and_underline() {
    let theme = create_test_theme();
    let styles = vec![
        "node.text.mark".to_string(),     // 黄色背景
        "node.text.underline".to_string(), // 下划线
    ];
    
    let merged = merge_styles(&styles, &theme);
    
    // 验证：应该有下划线修饰符和黄色背景
    assert!(merged.add_modifier.contains(Modifier::UNDERLINED), 
            "应该包含下划线修饰符");
    assert_eq!(merged.bg, Some(Color::Yellow), 
            "背景色应该是黄色");
    
    println!("✓ highlight + underline: {:?}", merged);
}

/// 测试：italic + bold（两个修饰符）
#[test]
fn test_italic_and_bold() {
    let theme = create_test_theme();
    let styles = vec![
        "node.text.strong".to_string(),  // 粗体
        "node.text.italic".to_string(),  // 斜体
    ];
    
    let merged = merge_styles(&styles, &theme);
    
    // 验证：应该同时有粗体和斜体
    assert!(merged.add_modifier.contains(Modifier::BOLD), 
            "应该包含粗体修饰符");
    assert!(merged.add_modifier.contains(Modifier::ITALIC), 
            "应该包含斜体修饰符");
    
    println!("✓ italic + bold: {:?}", merged);
}

/// 测试：code + bold（前景色 + 背景色 + 修饰符）
#[test]
fn test_code_and_bold() {
    let theme = create_test_theme();
    let styles = vec![
        "node.text.code".to_string(),    // 绿色前景 + 深色背景
        "node.text.strong".to_string(),  // 粗体
    ];
    
    let merged = merge_styles(&styles, &theme);
    
    // 验证：应该有粗体、绿色前景、深色背景
    assert!(merged.add_modifier.contains(Modifier::BOLD), 
            "应该包含粗体修饰符");
    assert_eq!(merged.fg, Some(Color::Green), 
            "前景色应该是绿色");
    assert_eq!(merged.bg, Some(Color::Rgb(38, 41, 44)), 
            "背景色应该是深色");
    
    println!("✓ code + bold: {:?}", merged);
}

/// 测试：顺序影响颜色覆盖
#[test]
fn test_order_affects_color_override() {
    let theme = create_test_theme();
    
    // code 在前，mark 在后 → mark 的背景色覆盖 code
    let styles1 = vec![
        "node.text.code".to_string(),
        "node.text.mark".to_string(),
    ];
    let merged1 = merge_styles(&styles1, &theme);
    
    // mark 在前，code 在后 → code 的背景色覆盖 mark
    let styles2 = vec![
        "node.text.mark".to_string(),
        "node.text.code".to_string(),
    ];
    let merged2 = merge_styles(&styles2, &theme);
    
    // 验证顺序影响
    assert_eq!(merged1.bg, Some(Color::Yellow), 
            "mark 在后，背景色应该是黄色");
    assert_eq!(merged2.bg, Some(Color::Rgb(38, 41, 44)), 
            "code 在后，背景色应该是深色");
    
    // 但修饰符应该累加（如果有的话）
    println!("✓ Order test - code+mark: {:?}, mark+code: {:?}", merged1, merged2);
}

/// 测试：三种样式组合
#[test]
fn test_three_styles_combination() {
    let theme = create_test_theme();
    let styles = vec![
        "node.text.code".to_string(),    // 绿色前景 + 深色背景
        "node.text.strong".to_string(),  // 粗体
        "node.text.underline".to_string(), // 下划线
    ];
    
    let merged = merge_styles(&styles, &theme);
    
    // 验证：应该有粗体、下划线、绿色前景、深色背景
    assert!(merged.add_modifier.contains(Modifier::BOLD), 
            "应该包含粗体修饰符");
    assert!(merged.add_modifier.contains(Modifier::UNDERLINED), 
            "应该包含下划线修饰符");
    assert_eq!(merged.fg, Some(Color::Green), 
            "前景色应该是绿色");
    assert_eq!(merged.bg, Some(Color::Rgb(38, 41, 44)), 
            "背景色应该是深色");
    
    println!("✓ code + bold + underline: {:?}", merged);
}

/// 测试：渲染到终端验证
#[test]
fn test_render_with_merged_styles() {
    let backend = TestBackend::new(50, 10);
    let mut terminal = Terminal::new(backend).unwrap();
    
    let theme = create_test_theme();
    let styles = vec![
        "node.text.code".to_string(),
        "node.text.strong".to_string(),
    ];
    let merged = merge_styles(&styles, &theme);
    
    let span = Span::styled("Test", merged);
    let paragraph = Paragraph::new(Line::from(span));
    
    terminal.draw(|f| f.render_widget(paragraph, f.size())).unwrap();
    
    let buffer = terminal.backend().buffer();
    let cell = buffer.get(0, 0);
    
    // 验证渲染后的样式
    assert!(cell.style().add_modifier.contains(Modifier::BOLD), 
            "渲染后应该包含粗体");
    assert_eq!(cell.style().fg, Some(Color::Green), 
            "渲染后前景色应该是绿色");
    
    println!("✓ Render test passed: {:?}", cell.style());
}
