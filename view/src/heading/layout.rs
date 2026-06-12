use crate::document::DocumentLine;
use crate::heading::{content_style_key, decor_style_key};

/// 按级别添加装饰线：
/// - H1: 上下双线 `═`
/// - H2: 下划线 `─`
/// - H3: 无装饰（仅靠颜色/字重区分）
/// - H4: 波浪下划线 `~`
/// - H5/H6: 短虚线感 `-`
pub fn decorate_heading(level: u8, content_width: u16, lines: &mut Vec<DocumentLine>) {
    if lines.is_empty() {
        return;
    }
    match level {
        1 => {
            lines.insert(0, DocumentLine::decoration_line(repeat_char('═', content_width)));
            lines.push(DocumentLine::decoration_line(repeat_char('═', content_width)));
        }
        2 => {
            lines.push(DocumentLine::decoration_line(repeat_char('─', content_width)));
        }
        3 => {}
        4 => {
            lines.push(DocumentLine::decoration_line(repeat_char('~', content_width)));
        }
        5 | 6 => {
            lines.push(DocumentLine::decoration_line(repeat_char('─', content_width)));
        }
        _ => {}
    }
}

/// 为标题各行写入 theme 样式键（装饰线与正文分级）
pub fn apply_heading_styles(lines: &mut [DocumentLine], level: u8) {
    let content_key = content_style_key(level).to_string();
    let decor_key = decor_style_key(level).to_string();

    for line in lines.iter_mut() {
        let base = if is_decoration_line(line) {
            decor_key.as_str()
        } else {
            content_key.as_str()
        };
        for item in &mut line.content {
            let inline_styles = std::mem::take(&mut item.styles);
            item.styles = vec![base.to_string()];
            item.styles.extend(inline_styles);
        }
    }
}

fn repeat_char(ch: char, width: u16) -> String {
    ch.to_string().repeat(width as usize)
}

fn is_decoration_line(line: &DocumentLine) -> bool {
    if line.content.len() != 1 {
        return false;
    }
    let text = &line.content[0].display_content;
    !text.is_empty()
        && text
            .chars()
            .all(|c| matches!(c, '═' | '─' | '~' | '-' | '·'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::document::InLineItem;
    use crate::heading::heading_level_from_node;
    use syservice::lute::node::Node;

    #[test]
    fn h1_gets_double_rule() {
        let mut lines = vec![DocumentLine::default_with_items(vec![InLineItem {
            marks: Default::default(),
            content: "Title".into(),
            display_content: "Title".into(),
            link: None,
            styles: vec![],
            line_break: false,
        }])];
        decorate_heading(1, 5, &mut lines);
        assert_eq!(lines.len(), 3);
        assert!(lines[0].content[0].display_content.chars().all(|c| c == '═'));
        assert!(lines[2].content[0].display_content.chars().all(|c| c == '═'));
    }

    #[test]
    fn apply_styles_uses_level_keys() {
        let mut lines = vec![DocumentLine::default_with_items(vec![InLineItem {
            marks: Default::default(),
            content: "H2".into(),
            display_content: "H2".into(),
            link: None,
            styles: vec![],
            line_break: false,
        }])];
        decorate_heading(2, 2, &mut lines);
        apply_heading_styles(&mut lines, 2);
        assert_eq!(lines[0].content[0].styles, vec!["node.heading.h2"]);
        assert_eq!(lines[1].content[0].styles, vec!["node.heading.h2.decor"]);
    }

    #[test]
    fn heading_level_defaults_to_one() {
        let node = Node::default();
        assert_eq!(heading_level_from_node(&node), 1);
    }
}
