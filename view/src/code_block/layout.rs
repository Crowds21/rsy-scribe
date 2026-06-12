//! 代码块布局：基于终端显示宽度计算外框与换行。

use super::glyphs::box_glyphs;
use unicode_width::UnicodeWidthStr;

/// 外框可完整展示所需的最小宽度（含左右边框）
pub const MIN_BOX_WIDTH: u16 = 16;

/// 列表项前缀占位（bullet + 空格），渲染前需从可用宽度中扣除
pub const LIST_ITEM_PREFIX_WIDTH: u16 = 2;

/// 顶部边框分段（便于分别着色）
#[derive(Clone, Debug)]
pub struct HeaderParts {
    pub left: String,
    pub language: String,
    pub right: String,
}

/// 内容行分段：左右竖线为边框，中间为代码
#[derive(Clone, Debug)]
pub struct ContentRowParts {
    pub left: String,
    pub content: String,
    pub padding_and_right: String,
}

/// 根据可用宽度计算外框总宽度（不超过终端宽度）
pub fn box_width(available_width: u16) -> u16 {
    if available_width >= MIN_BOX_WIDTH {
        available_width
    } else {
        available_width.max(8)
    }
}

/// 代码区内侧宽度（去掉左右 `│`）
pub fn inner_content_width(box_width: u16) -> u16 {
    let g = box_glyphs();
    let overhead = (g.vertical.width() * 2) as u16;
    box_width.saturating_sub(overhead).max(1)
}

/// 顶部边框：`╭─[lang]──…──╮`
pub fn format_header(language: &str, box_width: u16) -> HeaderParts {
    let g = box_glyphs();
    let left = format!("{}{}[", g.top_left, g.horizontal);
    let close_bracket = "]".to_string();
    let left_width = left.width() as u16;
    let lang_width = language.width() as u16;
    let bracket_width = close_bracket.width() as u16;
    let right_corner_width = g.top_right.width() as u16;
    let fill = box_width
        .saturating_sub(left_width)
        .saturating_sub(lang_width)
        .saturating_sub(bracket_width)
        .saturating_sub(right_corner_width);
    let right = format!(
        "{}{}{}",
        close_bracket,
        g.horizontal.repeat(fill as usize),
        g.top_right
    );
    HeaderParts {
        left,
        language: language.to_string(),
        right,
    }
}

/// 底部边框：`╰──…──╯`
pub fn format_footer(box_width: u16) -> String {
    let g = box_glyphs();
    let side = (g.bottom_left.width() + g.bottom_right.width()) as u16;
    let fill = box_width.saturating_sub(side);
    format!(
        "{}{}{}",
        g.bottom_left,
        g.horizontal.repeat(fill as usize),
        g.bottom_right
    )
}

/// 内容行：`│{content}{pad}│`
pub fn format_content_row(content: &str, inner_width: u16) -> ContentRowParts {
    let g = box_glyphs();
    let content_width = content.width() as u16;
    let padding = inner_width.saturating_sub(content_width);
    ContentRowParts {
        left: g.vertical.clone(),
        content: content.to_string(),
        padding_and_right: format!("{}{}", " ".repeat(padding as usize), g.vertical),
    }
}

pub fn header_display_width(parts: &HeaderParts) -> usize {
    parts.left.width() + parts.language.width() + parts.right.width()
}

pub fn content_row_display_width(parts: &ContentRowParts) -> usize {
    parts.left.width() + parts.content.width() + parts.padding_and_right.width()
}

/// 将一行代码按显示宽度拆分为多段
pub fn split_by_display_width(line: &str, max_width: u16) -> Vec<String> {
    if max_width == 0 {
        return vec![String::new()];
    }

    let mut result = Vec::new();
    let mut current = String::new();
    let mut current_width = 0u16;

    for c in line.chars() {
        let char_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(1) as u16;

        if current_width + char_width > max_width && !current.is_empty() {
            result.push(current);
            current = String::new();
            current_width = 0;
        }

        current.push(c);
        current_width += char_width;
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_display_width_matches_box() {
        let box_w = 40;
        let inner = inner_content_width(box_w);
        let row = format_content_row("let x = 1;", inner);
        assert_eq!(content_row_display_width(&row), box_w as usize);

        let header = format_header("rust", box_w);
        assert_eq!(header_display_width(&header), box_w as usize);

        let footer = format_footer(box_w);
        assert_eq!(footer.width(), box_w as usize);
    }

    #[test]
    fn test_header_has_top_right_corner() {
        let g = super::super::glyphs::box_glyphs();
        let header = format_header("rust", 30);
        assert!(header.right.ends_with(g.top_right.as_str()));
    }

    #[test]
    fn test_split_long_url() {
        let url = "https://example.com/very/long/path/segment";
        let inner = inner_content_width(30);
        let parts = split_by_display_width(url, inner);
        assert!(parts.len() > 1);
        for part in &parts {
            assert!(part.width() <= inner as usize);
        }
    }
}
