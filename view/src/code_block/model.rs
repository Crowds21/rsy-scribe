//! 代码块数据模型

use syservice::lute::node::Node;

/// 代码块模型
///
/// 用于表示思源笔记中的代码块节点，包含语言标识和代码内容。
///
/// # Examples
///
/// ```rust,ignore
/// let code_block = CodeBlockModel::from_node(&node);
/// println!("Language: {}", code_block.language);
/// ```
#[derive(Clone, Debug)]
pub struct CodeBlockModel {
    /// 语言标识（如 "rust", "python", "markdown"）
    pub language: String,
    /// 原始代码内容
    pub content: String,
    /// 按行分割的代码（用于逐行渲染）
    pub lines: Vec<String>,
    /// 是否被围栏包围（```）
    pub is_fenced: bool,
    /// 代码块 ID（用于语法高亮缓存）
    pub id: String,
}

impl CodeBlockModel {
    /// 从 Node 解析代码块
    pub fn from_node(node: &Node) -> Self {
        let mut language = String::new();
        let mut content = String::new();
        let is_fenced = node.is_fenced_code_block.unwrap_or(true);

        // 遍历子节点
        for child in &node.children {
            match child.node_type {
                syservice::lute::node::NodeType::NodeCodeBlockFenceInfoMarker => {
                    // 解析语言标识（Base64 解码）
                    if let Some(ref info) = child.code_block_info {
                        language = decode_base64(&String::from_utf8_lossy(info)).unwrap_or_default();
                    }
                }
                syservice::lute::node::NodeType::NodeCodeBlockCode => {
                    content = child.data.clone().unwrap_or_default();
                }
                _ => {}
            }
        }

        // 按行分割
        let lines: Vec<String> = content.lines().map(String::from).collect();

        // 提取 ID
        let id = node.id.clone().unwrap_or_default();

        Self {
            language,
            content,
            lines,
            is_fenced,
            id,
        }
    }

    /// 获取显示用的语言名称
    pub fn language_display(&self) -> &str {
        if self.language.is_empty() {
            "code"
        } else {
            &self.language
        }
    }

    /// 检查是否为空代码块
    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }

    /// 获取代码行数
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

/// Base64 解码辅助函数
fn decode_base64(encoded: &str) -> Option<String> {
    if encoded.is_empty() {
        return None;
    }

    // 使用 base64 crate 解码
    match base64::decode(encoded) {
        Ok(bytes) => String::from_utf8(bytes).ok(),
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_base64() {
        // "rust" 的 Base64 编码是 "cnVzdA=="
        assert_eq!(decode_base64("cnVzdA=="), Some("rust".to_string()));
    }

    #[test]
    fn test_decode_base64_empty() {
        assert_eq!(decode_base64(""), None);
    }

    #[test]
    fn test_decode_base64_invalid() {
        assert_eq!(decode_base64("invalid!!!"), None);
    }
}
