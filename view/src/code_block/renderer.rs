//! 代码块渲染器
//!
//! 将 CodeBlockModel 渲染为带完整外框、按终端宽度换行的行。

use super::layout::{
    box_width, format_content_row, format_footer, format_header, inner_content_width,
    split_by_display_width, ContentRowParts, HeaderParts,
};
use super::model::CodeBlockModel;
use super::highlight::{SyntaxHighlighter, NoOpHighlighter, StyledLine};
use crate::document::InLineItem;
use ratatui::style::Style;
use unicode_width::UnicodeWidthStr;

const STYLE_BORDER: &str = "code.block.border";
const STYLE_HEADER: &str = "code.block.header";
const STYLE_INFO: &str = "code.block.info";
const STYLE_CONTENT: &str = "code.block.content";

/// 代码块样式配置
#[derive(Clone, Debug)]
pub struct CodeBlockStyle {
    pub border_style: Style,
    pub header_style: Style,
    pub background: Style,
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

/// 渲染代码块；每行宽度不超过 `available_width`。
pub fn render_code_block(
    code_block: &CodeBlockModel,
    available_width: u16,
    _style: &CodeBlockStyle,
    highlighter: Option<&dyn SyntaxHighlighter>,
) -> Vec<InLineItem> {
    let box_w = box_width(available_width);
    let inner_w = inner_content_width(box_w);
    let mut items = Vec::new();

    items.extend(header_items(&format_header(code_block.language_display(), box_w)));

    let noop = NoOpHighlighter::new();
    let hl = highlighter.unwrap_or(&noop);
    let highlighted_lines = hl.highlight(&code_block.language, &code_block.content);

    if highlighted_lines.is_empty() && code_block.content.is_empty() {
        items.extend(content_row_items(&format_content_row("", inner_w)));
    } else {
        for styled_line in highlighted_lines {
            items.extend(render_styled_line(styled_line, inner_w));
        }
    }

    items.push(border_item(
        format_footer(box_w),
        true,
    ));

    items
}

fn header_items(parts: &HeaderParts) -> Vec<InLineItem> {
    vec![
        border_item(parts.left.clone(), false),
        styled_item(parts.language.clone(), STYLE_INFO, false),
        border_item(parts.right.clone(), true),
    ]
}

fn content_row_items(parts: &ContentRowParts) -> Vec<InLineItem> {
    vec![
        border_item(parts.left.clone(), false),
        styled_item(parts.content.clone(), STYLE_CONTENT, false),
        border_item(parts.padding_and_right.clone(), true),
    ]
}

fn border_item(text: String, line_break: bool) -> InLineItem {
    styled_item(text, STYLE_BORDER, line_break)
}

fn styled_item(text: String, style: &str, line_break: bool) -> InLineItem {
    InLineItem {
        marks: crate::styles::InlineMarks::default(),
        content: text.clone(),
        display_content: text,
        link: None,
        styles: vec![style.to_string()],
        line_break,
    }
}

fn render_styled_line(styled_line: StyledLine, inner_width: u16) -> Vec<InLineItem> {
    let mut items = Vec::new();

    for span in styled_line.spans {
        let segments = split_by_display_width(&span.content, inner_width);
        if segments.is_empty() {
            items.extend(content_row_items(&format_content_row("", inner_width)));
            continue;
        }
        for segment in segments {
            items.extend(content_row_items(&format_content_row(&segment, inner_width)));
        }
    }

    items
}

/// 从主题获取代码块样式
pub fn get_code_block_style(theme: &std::collections::HashMap<String, Style>) -> CodeBlockStyle {
    CodeBlockStyle {
        border_style: theme.get(STYLE_BORDER).copied().unwrap_or_default(),
        header_style: theme.get(STYLE_HEADER).copied().unwrap_or_default(),
        background: theme.get("code.block").copied().unwrap_or_default(),
        content_style: theme.get(STYLE_CONTENT).copied().unwrap_or_default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::layout::MIN_BOX_WIDTH;

    fn items_to_lines(items: Vec<InLineItem>) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        let mut current = String::new();
        for item in items {
            current.push_str(&item.display_content);
            if item.line_break {
                lines.push(current);
                current = String::new();
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
        lines
    }

    fn border_styles_only(styles: &[String]) -> bool {
        styles.iter().all(|s| s == STYLE_BORDER || s == STYLE_HEADER)
    }

    #[test]
    fn test_vertical_borders_use_border_style() {
        let code_block = CodeBlockModel {
            language: "rust".to_string(),
            content: "let x = 1;".to_string(),
            lines: vec![],
            is_fenced: true,
            id: String::new(),
        };
        let items = render_code_block(&code_block, 40, &CodeBlockStyle::default(), None);
        let vertical_items: Vec<_> = items
            .iter()
            .filter(|i| i.display_content == "│" || i.display_content.ends_with('│'))
            .collect();
        assert!(!vertical_items.is_empty());
        for item in vertical_items {
            assert!(
                border_styles_only(&item.styles),
                "vertical border should not use content color: {:?}",
                item.styles
            );
        }
    }

    #[test]
    fn test_each_row_has_uniform_width() {
        let code_block = CodeBlockModel {
            language: "rust".to_string(),
            content: "let rust = \"Hello world!\"\nhttps://example.com/long-url-path".to_string(),
            lines: vec![],
            is_fenced: true,
            id: String::new(),
        };
        let width = 50;
        let items = render_code_block(&code_block, width, &CodeBlockStyle::default(), None);
        let lines = items_to_lines(items);

        for line in &lines {
            assert_eq!(line.width(), width as usize, "line: {line}");
        }
    }

    #[test]
    fn test_code_block_visual_structure() {
        let code_block = CodeBlockModel {
            language: "rust".to_string(),
            content: "let rust = \"Hello world!\"\nhttps://example.com/long-url".to_string(),
            lines: vec![],
            is_fenced: true,
            id: String::new(),
        };
        let width = 40;
        let lines = items_to_lines(render_code_block(
            &code_block,
            width,
            &CodeBlockStyle::default(),
            None,
        ));

        assert!(lines.len() >= 4);
        assert!(lines[0].starts_with('╭'));
        assert!(lines[0].ends_with('╮'));
        assert!(lines[1].starts_with('│'));
        assert!(lines[1].ends_with('│'));
        assert!(lines.iter().any(|l| l.contains("let rust")));
        assert!(lines.last().unwrap().starts_with('╰'));
        assert!(lines.last().unwrap().ends_with('╯'));
    }

    #[test]
    fn test_respects_minimum_width() {
        let code_block = CodeBlockModel {
            language: "rust".to_string(),
            content: "x".to_string(),
            lines: vec![],
            is_fenced: true,
            id: String::new(),
        };
        let lines = items_to_lines(render_code_block(
            &code_block,
            MIN_BOX_WIDTH,
            &CodeBlockStyle::default(),
            None,
        ));
        assert!(lines.iter().all(|l| l.width() == MIN_BOX_WIDTH as usize));
    }
}
