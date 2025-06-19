use std::default::Default;
use crate::component::block::{BlockComponent, RenderedBlock};
use ratatui::prelude::Span;
use ratatui::text::Line;
use std::num::NonZeroUsize;
use std::str::FromStr;
use ratatui::layout::Rect;
use strum::EnumString;
use syservice::lute::node::{Node, NodeType};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// uses NonZeroUsize so Option<DocumentId> use a byte rather than two
/// 用于应用内标识文档.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DocumentId(NonZeroUsize);

impl Default for DocumentId {
    fn default() -> DocumentId {
        // Safety: 1 is non-zero
        DocumentId(unsafe { NonZeroUsize::new_unchecked(1) })
    }
}
impl std::fmt::Display for DocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
pub struct DocumentModel {
    pub(crate) id: DocumentId,
    pub lines: Vec<DocumentLine>,
    pub area:Rect,
}
#[derive(Clone, Default, EnumString)]
#[strum(serialize_all = "kebab-case")]
enum InLineMarkType {
    #[default]
    Default,
    Strong,
    Em,
    Mark,
    Code,
    BlockRef,
    A,
}
#[derive(Clone)]
struct InLineItem {
    item_type: InLineMarkType, // 行内元素类型
    content: String,           // 展示内容
    link: Option<String>,      // 转跳
    /// 对应 theme.toml 中的配置
    style: Option<String>,
}

#[derive(Clone)]
struct DocumentLine {
    content: Vec<InLineItem>, //	直接展示的内容
    node_type: NodeType,      // 	siyuan 对应的节点类型
    container: NodeType,      // 	元素外层可能是 quote 或者其他超级块
    indent: usize,            //	缩进, List项中的换行文本内容
}

impl DocumentModel {
    /// 将 SY AST 转换为  DocumentModel 的入口
    fn parse_ast_root_node(&mut self, mut root_node: Node, document_id: DocumentId) {
        self.id = document_id;
        root_node.set_node_type_for_tree();
        root_node.node_type = NodeType::from_str(&root_node.type_str).unwrap();
        let mut doc_title = self.create_doc_title_lines(&root_node);
        self.lines.append(&mut doc_title);

        for child in root_node.children.iter_mut() {
            self.parse_node_by_type(child);
        }
    }
    fn parse_node_by_type(&mut self, node: &Node) {
        match node.node_type {
            NodeType::Default => {}
            NodeType::NodeDocument => {}
            NodeType::NodeParagraph => {}
            NodeType::NodeHeading => {}
            NodeType::NodeHeadingC8hMarker => {}
            NodeType::NodeThematicBreak => {}
            NodeType::NodeBlockquote => {}
            NodeType::NodeBlockquoteMarker => {}
            NodeType::NodeList => {}
            NodeType::NodeListItem => {}
            _ => {}
        }
    }

    fn create_doc_title_lines(&self, root: &Node) -> Vec<DocumentLine> {
        let title = root
            .properties
            .as_ref()
            .and_then(|map| map.get("title"))
            .get_or_insert(&"".to_string())
            .clone();
        let length = title.width();
        let decoration_line = "═".repeat(length);
        let items: Vec<InLineItem> = vec![InLineItem::default_title(decoration_line)];
        let top_decoration_line = DocumentLine {
            content: items,
            node_type: Default::default(),
            container: Default::default(),
            indent: 0,
        };
        let content: Vec<InLineItem> = vec![InLineItem::default_title(title.clone())];
        let title_line = DocumentLine {
            content,
            node_type: Default::default(),
            container: Default::default(),
            indent: 0,
        };
        let bottom_decoration_line = top_decoration_line.clone();
        vec![top_decoration_line, title_line, bottom_decoration_line]
    }

    fn create_node_paragraph(&mut self, node: Node) {
        let mut items: Vec<InLineItem> = vec![];
        for child in node.children.iter() {
            let item = match child.node_type {
                NodeType::NodeTextMark => self.create_node_text_mark(child),
                NodeType::NodeText => self.create_node_text(child),
                _ => continue,
            };
            items.push(item);
        }
        self.wrap_lines_into_document(&node, items);
    }

    fn create_node_text(&mut self, node: &Node) -> InLineItem {
        let content = node.data.clone().unwrap_or_default();
        InLineItem {
            item_type: InLineMarkType::Default,
            content,
            link: None,
            style: None,
        }
    }
    fn create_node_text_mark(&mut self, node: &Node) -> InLineItem {
        let content = node
            .text_mark_text_content
            .clone()
            .unwrap_or_default()
            .clone();
        let mark_type = node.text_mark_type.clone().unwrap_or_default().clone();
        let enum_mark_type = InLineMarkType::from_str(&mark_type).unwrap_or_default();
        let mut item = InLineItem {
            item_type: enum_mark_type,
            content,
            link: None,
            style: None,
        };
        match item.item_type {
            InLineMarkType::Strong => item.style = Some("node.text.strong".to_string()),
            InLineMarkType::Em => item.style = Some("node.text.italic".to_string()),
            InLineMarkType::Mark => item.style = Some("node.text.mark".to_string()),
            InLineMarkType::Code => item.style = Some("node.text.code".to_string()),
            InLineMarkType::BlockRef => {
                item.style = Some("node.text.blockref".to_string());
                let block_ref = node.text_mark_block_ref_id.clone().unwrap_or_default();
                item.link = Some(block_ref);
            }
            InLineMarkType::A => {
                let web_link = node.text_mark_a_href.clone().unwrap_or_default();
                item.link = Some(web_link);
                item.style = Some("node.text.weblink".to_string())
            }
            _ => {}
        }
        item
    }
    /// 将行内元素按显示宽度分割成多行
    fn wrap_lines_into_document(&mut self, node: &Node, items: Vec<InLineItem>) {
        let mut current_line = DocumentLine {
            content: Vec::new(),
            node_type: node.node_type.clone(),
            container: NodeType::Default,
            indent: 0,
        };
        let mut current_width = 0;

        // 获取终端可用宽度（考虑缩进）
        let max_width = self.area.width as usize;
        let indent_spaces = current_line.indent * 2; // 假设每级缩进2空格
        let available_width = max_width.saturating_sub(indent_spaces);

        for item in items {
            let item_width = self.calculate_item_width(&item);

            // 处理显式换行（原始文本包含\n）
            if let Some(pos) = item.content.find('\n') {
                let (before, after) = item.content.split_at(pos);

                // 添加换行前的内容
                if !before.is_empty() {
                    current_line.content.push(InLineItem {
                        content: before.to_string(),
                        ..item.clone()
                    });
                    current_width += self.calculate_text_width(before);
                }

                // 保存当前行并创建新行
                self.lines.push(current_line);
                current_line = DocumentLine {
                    content: Vec::new(),
                    node_type: node.node_type.clone(),
                    container: Default::default(),
                    indent: 0,
                };
                current_width = 0;

                // 处理换行后的内容
                if !after.is_empty() {
                    current_line.content.push(InLineItem {
                        content: after[1..].to_string(), // 跳过\n
                        ..item.clone()
                    });
                    current_width += self.calculate_text_width(&after[1..]);
                }
                continue;
            }

            // 自动换行逻辑
            if current_width + item_width > available_width {
                // 尝试在单词边界分割
                if let Some((head, tail)) = self.split_item_at_boundary(&item, available_width - current_width) {
                    self.lines.push(current_line);

                    // 创建新行并添加剩余内容
                    current_line = DocumentLine {
                        content: vec![tail.clone()],
                        node_type: node.node_type.clone(),
                        container: Default::default(),
                        indent: 0,
                    };
                    current_width = self.calculate_item_width(&tail);
                } else {
                    // 强制换行
                    self.lines.push(current_line);
                    current_line = DocumentLine {
                        content: vec![item],
                        node_type: node.node_type.clone(),
                        container: Default::default(),
                        indent: 0,
                    };
                    current_width = item_width;
                }
            } else {
                current_line.content.push(item);
                current_width += item_width;
            }
        }

        // 添加最后一行
        if !current_line.content.is_empty() {
            self.lines.push(current_line);
        }
    }
    /// 计算纯文本宽度（无样式影响）
    fn calculate_text_width(&self, text: &str) -> usize {
        text.width()
    }
    
    /// 计算行内元素的显示宽度（考虑Unicode宽度）
    fn calculate_item_width(&self, item: &InLineItem) -> usize {
        item.content.width()
    }
    /// 在单词边界处智能分割行内元素
    fn split_item_at_boundary(&self, item: &InLineItem, max_width: usize) -> Option<(InLineItem, InLineItem)> {
        let mut accum = 0;
        let mut split_pos = None;

        // 查找最后一个可分割的空格位置
        for (i, c) in item.content.char_indices() {
            accum += c.width().unwrap_or(1);
            if c.is_whitespace() && accum <= max_width {
                split_pos = Some(i);
            }
            if accum > max_width {
                break;
            }
        }

        split_pos.map(|pos| {
            let (head, tail) = item.content.split_at(pos);
            (
                InLineItem {
                    content: head.trim_end().to_string(),
                    ..item.clone()
                },
                InLineItem {
                    content: tail.trim_start().to_string(),
                    ..item.clone()
                }
            )
        })
    }
    
}
impl InLineItem {
    fn default_title(content: String) -> InLineItem {
        InLineItem {
            item_type: InLineMarkType::Default,
            content,
            link: None,
            style: Some("node.heading.title".to_string()),
        }
    }
}

fn create_title_block(root: Node, handler: impl FnOnce(&String)) {
    let title = root.properties.as_ref().and_then(|map| {
        // 获取title并构建Paragraph
        map.get("title").map(|title| {
            handler(title);
        })
    });
}
#[cfg(test)]
mod test {
    #[test]
    fn test_document_model() {}
}
