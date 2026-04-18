//! 代码块渲染器
//!
//! 将 CodeBlockModel 渲染为带样式的行。

use super::model::CodeBlockModel;
use super::highlight::{SyntaxHighlighter, NoOpHighlighter, StyledLine};
use crate::document::InLineItem;
use ratatui::style::Style;

/// 代码块样式配置
///
/// 用于定义代码块的容器样式（边框、背景、头部等）。
#[derive(Clone, Debug)]
pub struct CodeBlockStyle {
    /// 边框样式
    pub border_style: Style,
    /// 头部样式（语言标识）
    pub header_style: Style,
    /// 背景样式
    pub background: Style,
    /// 代码内容样式
    pub content_style: Style,
}

impl Default for CodeBlockStyle {
    fn default() -> Self {
        Self {
            border_style: Style::default(),
            header_style: Style::default(),
            background: Style::default(),
            content_style: Style::default(),
        }
    }
}

/// 渲染代码块
///
/// # Arguments
///
/// * `code_block` - 代码块模型
/// * `available_width` - 可用显示宽度
/// * `style` - 样式配置
/// * `highlighter` - 语法高亮器（可选）
///
/// # Returns
///
/// 返回渲染后的 InLineItem 向量
pub fn render_code_block(
    code_block: &CodeBlockModel,
    available_width: u16,
    style: &CodeBlockStyle,
    highlighter: Option<&dyn SyntaxHighlighter>,
) -> Vec<InLineItem> {
    let mut items = Vec::new();

    // 1. 顶部边框（带语言标识）
    items.push(create_header_item(code_block, available_width, style));

    // 2. 代码内容
    let noop = NoOpHighlighter::new();
    let hl = highlighter.unwrap_or(&noop);
    let highlighted_lines = hl.highlight(&code_block.language, &code_block.content);

    for styled_line in highlighted_lines {
        items.extend(render_styled_line(styled_line, available_width, style));
    }

    // 3. 底部边框
    items.push(create_footer_item(available_width, style));

    items
}

/// 创建头部项（带语言标识的边框）
fn create_header_item(
    code_block: &CodeBlockModel,
    width: u16,
    style: &CodeBlockStyle,
) -> InLineItem {
    // 格式：┌─[rust]─────────────────
    let language = code_block.language_display();
    let prefix = format!("┌─[{}]", language);
    let suffix_len = width.saturating_sub(prefix.len() as u16);
    let suffix = "─".repeat(suffix_len as usize);

    InLineItem {
        marks: crate::styles::InlineMarks::default(),
        content: format!("{}{}", prefix, suffix),
        display_content: format!("{}{}", prefix, suffix),
        link: None,
        styles: vec!["code.block.header".to_string()],
        line_break: false,
    }
}

/// 创建底部项（边框）
fn create_footer_item(width: u16, style: &CodeBlockStyle) -> InLineItem {
    let line = "└".to_string() + &"─".repeat(width as usize);

    InLineItem {
        marks: crate::styles::InlineMarks::default(),
        content: line.clone(),
        display_content: line,
        link: None,
        styles: vec!["code.block.border".to_string()],
        line_break: false,
    }
}

/// 渲染带样式的行
fn render_styled_line(
    styled_line: StyledLine,
    max_width: u16,
    style: &CodeBlockStyle,
) -> Vec<InLineItem> {
    let mut items = Vec::new();

    // 添加左侧边框
    let left_border = InLineItem {
        marks: crate::styles::InlineMarks::default(),
        content: "│ ".to_string(),
        display_content: "│ ".to_string(),
        link: None,
        styles: vec!["code.block.border".to_string()],
        line_break: false,
    };
    items.push(left_border);

    // 渲染代码内容
    let content_width = max_width.saturating_sub(4); // 减去边框和边距

    for span in styled_line.spans {
        // 处理长行换行
        let split_lines = super::parser::split_code_line(&span.content, content_width);

        for (i, line) in split_lines.iter().enumerate() {
            if i > 0 {
                // 换行后需要添加左侧边框
                items.push(InLineItem {
                    marks: crate::styles::InlineMarks::default(),
                    content: "│ ".to_string(),
                    display_content: "│ ".to_string(),
                    link: None,
                    styles: vec!["code.block.border".to_string()],
                    line_break: true,
                });
            }

            items.push(InLineItem {
                marks: crate::styles::InlineMarks::default(),
                content: line.clone(),
                display_content: line.clone(),
                link: None,
                styles: vec!["code.block.content".to_string()], // 实际样式由高亮器决定
                line_break: false,
            });
        }
    }

    // 添加右侧边框
    items.push(InLineItem {
        marks: crate::styles::InlineMarks::default(),
        content: " │".to_string(),
        display_content: " │".to_string(),
        link: None,
        styles: vec!["code.block.border".to_string()],
        line_break: true,
    });

    items
}

/// 从主题获取代码块样式
pub fn get_code_block_style(theme: &std::collections::HashMap<String, Style>) -> CodeBlockStyle {
    CodeBlockStyle {
        border_style: theme.get("code.block.border").copied().unwrap_or_default(),
        header_style: theme.get("code.block.header").copied().unwrap_or_default(),
        background: theme.get("code.block").copied().unwrap_or_default(),
        content_style: theme.get("code.block.content").copied().unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_header_item() {
        let code_block = CodeBlockModel {
            language: "rust".to_string(),
            content: String::new(),
            lines: Vec::new(),
            is_fenced: true,
            id: String::new(),
        };

        let style = CodeBlockStyle::default();
        let item = create_header_item(&code_block, 40, &style);

        assert!(item.display_content.starts_with("┌─[rust]"));
        assert!(item.display_content.contains("─"));
    }

    #[test]
    fn test_create_footer_item() {
        let style = CodeBlockStyle::default();
        let item = create_footer_item(40, &style);

        assert!(item.display_content.starts_with("└"));
        assert!(item.display_content.contains("─"));
    }
}
