//! 代码块解析器

use super::model::CodeBlockModel;
use syservice::lute::node::Node;

/// 从 Node 解析代码块
///
/// # Arguments
///
/// * `node` - 思源笔记的 NodeCodeBlock 节点
///
/// # Returns
///
/// 返回解析后的 CodeBlockModel
///
/// # Example
///
/// ```rust,ignore
/// let code_block = parse_code_block(&node);
/// println!("Language: {}, Lines: {}", code_block.language, code_block.line_count());
/// ```
pub fn parse_code_block(node: &Node) -> CodeBlockModel {
    CodeBlockModel::from_node(node)
}

/// 分割长代码行以适应显示宽度（按 Unicode 显示宽度）
pub fn split_code_line(line: &str, max_width: u16) -> Vec<String> {
    super::layout::split_by_display_width(line, max_width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_short_line() {
        let line = "let x = 1;";
        let result = split_code_line(line, 20);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "let x = 1;");
    }

    #[test]
    fn test_split_long_line() {
        use unicode_width::UnicodeWidthStr;
        let line = "let very_long_variable_name = 12345;";
        let max = 20;
        let result = split_code_line(line, max);
        assert!(result.len() > 1);
        assert!(result.iter().all(|l| l.width() <= max as usize));
    }

    #[test]
    fn test_split_empty_line() {
        let result = split_code_line("", 20);
        // 空行返回空向量（没有字符需要分割）
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_split_unicode() {
        let line = "你好，世界！";
        let result = split_code_line(line, 10);
        // 中文字符宽度为 2
        assert!(result.len() > 1);
    }
}
