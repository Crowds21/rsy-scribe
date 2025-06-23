use crate::component::gutter::{GutterConfig, GutterType};
use ratatui::layout::Rect;
use std::default::Default;
use std::num::NonZeroUsize;
use std::str::FromStr;
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
    pub area: Rect,
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
    ///    行内需要渲染的元素
    content: Vec<InLineItem>,
    /// siyuan 对应的节点类型
    node_type: NodeType,
    /// 元素外层可能是 quote 或者其他超级块
    container: NodeType,
    /// 缩进.List类型的元素在换行时需要保持缩进
    indent_width: usize,
}
impl Default for DocumentLine {
    fn default() -> Self {
        DocumentLine {
            content: vec![],
            node_type: Default::default(),
            container: Default::default(),
            indent_width: 0,
        }
    }
}
impl DocumentLine {
    fn default_with_item(item: InLineItem) -> DocumentLine {
        DocumentLine {
            content: vec![item],
            node_type: Default::default(),
            container: Default::default(),
            indent_width: 0,
        }
    }
    fn default_with_items(items: Vec<InLineItem>) -> DocumentLine {
        DocumentLine {
            content: items,
            node_type: Default::default(),
            container: Default::default(),
            indent_width: 0,
        }
    }
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

    ///  TOOD 如果有嵌入块查询结果,需要考虑添加缩进
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
            indent_width: 0,
        };
        let content: Vec<InLineItem> = vec![InLineItem::default_title(title.clone())];
        let title_line = DocumentLine::default_with_items(content);
        let bottom_decoration_line = top_decoration_line.clone();
        vec![top_decoration_line, title_line, bottom_decoration_line]
    }

    fn create_paragraph_lines(&mut self, node: Node) {
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
    /// 将一个 Block 内的元素按显示宽度分割成多行
    /// TODO List 等换行需要注意缩进
    fn wrap_lines_into_document(&mut self, node: &Node, items: Vec<InLineItem>) {
        let mut lines: Vec<DocumentLine> = vec![];
        let available_width = self.area.width as usize;

        // 表示当前正在处理的行
        let mut last_line: Option<&DocumentLine> = self.lines.last();

        let mut current_line = DocumentLine::default();
        let mut current_width = 0;
        for item in items {
            let item_width = self.calculate_item_width(&item);

            // 处理显式换行（原始文本包含\n）
            if let Some(pos) = item.content.find('\n') {
                let (before, after) = item.content.split_at(pos);
                let before_item = InLineItem {
                    content: before.to_string(),
                    ..item.clone()
                };

                // TODO 合并拆分的
                //  这里的合并操作应该直接在函数内完成.因为可能需要拆分成好几行,而不是简单拆一次.
                //  拆行的时候应该直接对长度做除法计算.而不是逐个字符计算  
                self.merge_item_into_line(before_item);
                // 处理 before 的部分
                let wrap_result =
                    self.wrap_line_by_length(self.lines.clone().last().unwrap(), self.area.width);
                // 处理 after 的部分
                // TODO 同样调用函数去生成行
                current_line.content.push(InLineItem {
                    content: after[1..].to_string(), // 跳过\n
                    ..item.clone()
                });
                current_width += self.calculate_text_width(&after[1..]);
                continue;
            }

            // 计算 current_line 是否超出长度限制
            if current_width + item_width > available_width {
                // 尝试在单词边界分割
                if let Some((head, tail)) =
                    self.split_item_at_boundary(&item, available_width - current_width)
                {
                    let head_line = DocumentLine {
                        content: vec![head],
                        node_type: node.node_type,
                        container: Default::default(),
                        indent_width: 0,
                    };
                    self.lines.push(head_line);
                    let tail_line = DocumentLine::default_with_item(tail.clone());
                    current_width = self.calculate_item_width(&tail);
                } else {
                    // 强制换行
                    self.lines.push(current_line);
                    current_line = DocumentLine {
                        content: vec![item],
                        node_type: node.node_type.clone(),
                        container: Default::default(),
                        indent_width: 0,
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

    /// 将 行级元素放入 Item
    fn merge_item_into_line(&mut self, item: InLineItem) {
        if let Some(last_line) = self.lines.last_mut() {
            // 将 item 添加到最后一个 DocumentLine 的 content 中
            last_line.content.push(item);
        } else {
            // 如果没有行，则创建一个新行并添加 item
            let new_line = DocumentLine {
                content: vec![item],
                node_type: NodeType::NodeParagraph, // 默认段落类型
                container: NodeType::Default,       // 默认容器
                indent_width: 0,
            };
            self.lines.push(new_line);
        }
    }

    /// 针对已经生成的 DocumentLine 判断是否需要进行分行
    /// 如果需要则返回分割后的两个 DocumentLine
    fn wrap_line_by_length(
        &mut self,
        line: &DocumentLine,
        available_width: u16,
    ) -> Option<(DocumentLine, DocumentLine)> {
        let usize_width = usize::from(available_width);
        let current_width = self.calculate_line_width(&line);
        if current_width <= usize_width {
            return None;
        }

        // 寻找最佳分割位置
        let mut best_split_index = None;
        let mut accumulated_width = 0;
        let mut last_space_position = None;

        // 寻找最佳分割点- 行内元素索引 和 字符索引(itemIndex,charIndex)
        for (i, item) in line.content.iter().enumerate() {
            let item_width = self.calculate_item_width(item);
            accumulated_width += item_width;

            // 记录最后一个空格位置（单词边界）
            if let Some(space_pos) = item.content.rfind(' ') {
                last_space_position = Some((i, space_pos));
            }

            // 找到第一个超过可用宽度的位置
            if accumulated_width > usize_width {
                // 优先在最后一个空格处分隔
                if let Some((item_idx, char_idx)) = last_space_position {
                    best_split_index = Some((item_idx, char_idx + 1)); // 在空格后分割
                } else {
                    // 没有空格，在当前项目中间分割
                    best_split_index = Some((i, 0));
                }
                break;
            }
        }

        // (InLineItem, 以及 Item 中的字符位置)
        let Some((split_item_idx, split_char_idx)) = best_split_index else {
            return None;
        };

        // 基于 item_idx 分割内容
        let mut first_part = line.content[..=split_item_idx].to_vec(); // ..= 包含右边界
        let mut second_part = if split_item_idx + 1 < line.content.len() {
            line.content[split_item_idx + 1..].to_vec() // 剩余元素
        } else {
            Vec::new()
        };

        // 基于 split_char_idx 分割内容
        if split_char_idx > 0 {
            if let Some(split_item) = first_part.last_mut() {
                if split_char_idx < split_item.content.len() {
                    let remaining_text = split_item.content.split_off(split_char_idx);
                    second_part.insert(
                        0,
                        InLineItem {
                            item_type: split_item.item_type.clone(),
                            content: remaining_text,
                            link: split_item.link.clone(),
                            style: split_item.style.clone(),
                        },
                    );
                }
            }
        }

        // 创建新行
        let first_line = DocumentLine {
            content: first_part,
            node_type: line.node_type.clone(),
            container: line.container.clone(),
            indent_width: 0,
        };

        let second_line = DocumentLine {
            content: second_part,
            node_type: line.node_type.clone(),
            container: line.container.clone(),
            indent_width: 0,
        };

        Some((first_line, second_line))
    }

    fn calculate_text_width(&self, text: &str) -> usize {
        text.width()
    }
    fn calculate_line_width(&self, line: &DocumentLine) -> usize {
        let mut width = 0;
        for item in line.content.iter() {
            width += item.content.width();
        }
        width
    }

    fn calculate_item_width(&self, item: &InLineItem) -> usize {
        item.content.width()
    }
    /// TODO 在单词边界处智能分割行内元素
    ///     遍历方式可以改成从后往前遍历.如果小于直接返回. 如果大于找到第一个空格返回
    fn split_item_at_boundary(
        &self,
        item: &InLineItem,
        max_width: usize,
    ) -> Option<(InLineItem, InLineItem)> {
        let mut accum = 0;
        let mut split_pos = None;

        // 查找最后一个可分割的空格位置
        for (i, c) in item.content.char_indices() {
            accum += c.width().unwrap_or(1);
            // TODO 如果超出位置刚好是空格
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
                },
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
    use crate::model::document_model::{DocumentId, DocumentLine, DocumentModel};
    use ratatui::layout::Rect;
    use syservice::lute::node::Node;

    fn create_empty_model_with_20x20() -> DocumentModel {
        let rect = Rect::new(0, 0, 20, 20);
        DocumentModel {
            id: DocumentId::default(),
            lines: vec![],
            area: rect,
        }
    }

    #[test]
    fn test_create_title_lines() {
        let model = create_empty_model_with_20x20();
        let root_node = syservice::test_utils::load_sy_test_file().unwrap_or_default();

        let title_lines = model.create_doc_title_lines(&root_node);
        assert_eq!(title_lines.len(), 3);
    }
    #[test]
    fn test_create_paragraph_lines() {
        let mut model = create_empty_model_with_20x20();
        let mut root_node = syservice::test_utils::load_sy_test_file().unwrap_or_default();
        let mut node: Node = Node::default();
        for child in root_node.children.iter_mut() {
            if child.clone().id.unwrap().eq("20250512161513-8hypgbv") {
                node = child.clone()
            }
        }
        node.set_node_type_for_tree();
        model.create_paragraph_lines(node);
        assert_eq!("", "");
    }
}
