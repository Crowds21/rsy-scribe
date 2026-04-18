//! 语法高亮接口
//!
//! 提供语法高亮的 Trait 定义和默认实现。
//! 未来可以集成 Tree-sitter、Syntect 等高亮引擎。

use ratatui::style::Style;

/// 带样式的行
#[derive(Clone, Debug)]
pub struct StyledLine {
    /// 样式片段
    pub spans: Vec<StyledSpan>,
}

/// 带样式的片段
#[derive(Clone, Debug)]
pub struct StyledSpan {
    /// 内容
    pub content: String,
    /// 样式
    pub style: Style,
}

/// 语法高亮器 Trait
///
/// 用于实现不同语言的语法高亮。
/// 可以通过实现此 Trait 来集成 Tree-sitter、Syntect 等高亮引擎。
///
/// # Example
///
/// ```rust,ignore
/// struct MyHighlighter;
///
/// impl SyntaxHighlighter for MyHighlighter {
///     fn highlight(&self, language: &str, code: &str) -> Vec<StyledLine> {
///         // 实现高亮逻辑
///     }
/// }
/// ```
pub trait SyntaxHighlighter: Send + Sync {
    /// 高亮代码，返回带样式的行
    ///
    /// # Arguments
    ///
    /// * `language` - 编程语言标识（如 "rust", "python"）
    /// * `code` - 源代码
    fn highlight(&self, language: &str, code: &str) -> Vec<StyledLine>;

    /// 检查是否支持该语言
    ///
    /// # Arguments
    ///
    /// * `language` - 编程语言标识
    fn supports_language(&self, language: &str) -> bool;
}

/// 默认实现（无高亮）
///
/// 当没有可用的语法高亮器时，使用此实现。
/// 所有代码以单一样式显示。
pub struct NoOpHighlighter {
    /// 默认样式
    pub default_style: Style,
}

impl Default for NoOpHighlighter {
    fn default() -> Self {
        Self {
            default_style: Style::default(),
        }
    }
}

impl NoOpHighlighter {
    /// 创建新的无高亮器
    pub fn new() -> Self {
        Self::default()
    }

    /// 创建带指定样式的无高亮器
    pub fn with_style(style: Style) -> Self {
        Self {
            default_style: style,
        }
    }
}

impl SyntaxHighlighter for NoOpHighlighter {
    fn highlight(&self, _language: &str, code: &str) -> Vec<StyledLine> {
        // 无高亮：每行作为一个单一风格的片段
        code.lines()
            .map(|line| StyledLine {
                spans: vec![StyledSpan {
                    content: line.to_string(),
                    style: self.default_style,
                }],
            })
            .collect()
    }

    fn supports_language(&self, _language: &str) -> bool {
        false
    }
}

/// 语法高亮器管理器
///
/// 用于管理多个高亮器，根据语言自动选择合适的高亮器。
pub struct HighlighterManager {
    highlighters: Vec<Box<dyn SyntaxHighlighter>>,
    fallback: Box<dyn SyntaxHighlighter>,
}

impl HighlighterManager {
    /// 创建新的管理器
    pub fn new() -> Self {
        Self {
            highlighters: Vec::new(),
            fallback: Box::new(NoOpHighlighter::new()),
        }
    }

    /// 添加高亮器
    pub fn add_highlighter(&mut self, highlighter: Box<dyn SyntaxHighlighter>) {
        self.highlighters.push(highlighter);
    }

    /// 设置回退高亮器
    pub fn set_fallback(&mut self, fallback: Box<dyn SyntaxHighlighter>) {
        self.fallback = fallback;
    }

    /// 获取适合指定语言的高亮器
    pub fn get_highlighter(&self, language: &str) -> &dyn SyntaxHighlighter {
        for highlighter in &self.highlighters {
            if highlighter.supports_language(language) {
                return highlighter.as_ref();
            }
        }
        self.fallback.as_ref()
    }

    /// 高亮代码
    pub fn highlight(&self, language: &str, code: &str) -> Vec<StyledLine> {
        self.get_highlighter(language).highlight(language, code)
    }
}

impl Default for HighlighterManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_highlighter() {
        let highlighter = NoOpHighlighter::new();
        let code = "fn main() {\n    println!(\"Hello\");\n}";
        let result = highlighter.highlight("rust", code);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].spans.len(), 1);
        assert_eq!(result[0].spans[0].content, "fn main() {");
    }

    #[test]
    fn test_noop_highlighter_empty() {
        let highlighter = NoOpHighlighter::new();
        let result = highlighter.highlight("rust", "");

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_highlighter_manager() {
        let mut manager = HighlighterManager::new();
        manager.add_highlighter(Box::new(NoOpHighlighter::new()));

        let code = "let x = 1;";
        let result = manager.highlight("rust", code);

        assert_eq!(result.len(), 1);
    }
}
