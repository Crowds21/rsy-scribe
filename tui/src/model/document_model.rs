use crate::component::block::doc::create_tui_element;
use crate::component::block::{BlockComponent, RenderedBlock};
use crate::model::utils;
use ratatui::layout::Rect;
use std::default::Default;
use std::fmt;
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
pub(crate) struct DocumentModel {
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
#[derive(Clone, Default)]
pub(crate) struct InLineItem {
    item_type: InLineMarkType,  // 行内元素类型
    pub(crate) content: String, // 展示内容
    link: Option<String>,       // 转跳
    /// 对应 theme.toml 中的配置
    style: Option<String>,
    line_break: bool,
}
#[derive(Clone, Default)]
pub(crate) struct DocumentLine {
    ///    行内需要渲染的元素
    pub(crate) content: Vec<InLineItem>,
    /// siyuan 对应的节点类型
    node_type: NodeType,
    /// 是否为块内软换行
    break_line: bool,
    /// 元素外层可能是 quote 或者其他超级块
    container: NodeType,
    /// 缩进.List类型的元素在换行时需要保持缩进
    indent_width: usize,
}
impl DocumentLine {
    fn default_with_item(item: InLineItem) -> DocumentLine {
        let flag = item.line_break;
        DocumentLine {
            content: vec![item],
            node_type: Default::default(),
            break_line: flag,
            container: Default::default(),
            indent_width: 0,
        }
    }
    fn default_with_items(items: Vec<InLineItem>) -> DocumentLine {
        DocumentLine {
            content: items,
            node_type: Default::default(),
            break_line: false,
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

        let line_width = self.area.width as usize;
        for child in root_node.children.iter_mut() {
            self.parse_node_by_type(child, line_width);
        }
    }
    fn parse_node_by_type(&mut self, node: &Node, available_width: usize) -> Vec<DocumentLine> {
        let mut result: Vec<DocumentLine> = Vec::new();
        let mut lines = match node.node_type {
            // NodeType::Default => {}
            // NodeType::NodeDocument => {}
            NodeType::NodeParagraph => self.create_paragraph_block_lines(node, available_width),
            // NodeType::NodeHeading => {}
            // NodeType::NodeHeadingC8hMarker => {}
            // NodeType::NodeThematicBreak => {}
            // NodeType::NodeBlockquote => {}
            // NodeType::NodeBlockquoteMarker => {}
            NodeType::NodeList => self.create_list_block_lines(node),
            _ => Vec::new(),
        };
        result.append(&mut lines);
        result
    }

    /// TOOD 如果有嵌入块查询结果,需要考虑添加缩进
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
            break_line: false,
            container: Default::default(),
            indent_width: 0,
        };
        let content: Vec<InLineItem> = vec![InLineItem::default_title(title.clone())];
        let title_line = DocumentLine::default_with_items(content);
        let bottom_decoration_line = top_decoration_line.clone();
        vec![top_decoration_line, title_line, bottom_decoration_line]
    }

    fn create_paragraph_block_lines(
        &mut self,
        node: &Node,
        available_width: usize,
    ) -> Vec<DocumentLine> {
        let items = self.create_paragraph_inline_items(node);
        let lines = DocumentModel::split_item_to_document_lines(items, available_width);
        lines
    }
    fn create_list_block_lines(&mut self, node: &Node) -> Vec<DocumentLine> {
        let mut lines: Vec<DocumentLine> = Vec::new();
        // TODO 这里需要做层级的计算
        let line_width = self.area.width as usize;
        for child in node.children.iter() {
            let mut temp_result = self.crate_list_iterators(child, line_width);
            lines.append(&mut temp_result);
        }
        lines
    }
    fn crate_list_iterators(&mut self, node: &Node, available_width: usize) -> Vec<DocumentLine> {
        // TODO 这里返回的是 ListItem 中等待展示的Lines
        //  所以只需要对第一行插入BulletChar. 其他行插入等量的空格符
        //  但是这里没法考虑多级缩进的问题
        //  否则就需要在创建 Paragraph 前就获取到缩进,并且知道是第几层缩进
        let mut lines: Vec<DocumentLine> = Vec::new();
        let temp_result = match node.node_type {
            NodeType::NodeList => self.create_list_block_lines(node),
            NodeType::NodeListItem => self.create_list_item_block(node, available_width),
            NodeType::NodeParagraph => self.create_paragraph_block_lines(node, available_width),
            _ => Vec::new(),
        };
        lines = temp_result;
        lines
    }
    fn create_list_item_block(&mut self, node: &Node, available_width: usize) -> Vec<DocumentLine> {
        let mut result: Vec<DocumentLine> = Vec::new();
        for child in node.children.iter() {
            let mut temp = self.crate_list_iterators(child, available_width);
            result.append(&mut temp);
        }
        let bullet_char = node
            .list_data
            .as_ref()
            .and_then(|data| data.bullet_char)
            .map(|c| c as char)
            .unwrap_or('-');
        for (index, line) in result.iter_mut().enumerate() {
            if index == 0 {
                let item = InLineItem {
                    item_type: Default::default(),
                    content: format!("{} ", bullet_char),
                    link: None,
                    style: None,
                    line_break: false,
                };
                line.content.insert(0, item);
            } else {
                let item = InLineItem {
                    item_type: Default::default(),
                    content: "  ".to_string(),
                    link: None,
                    style: None,
                    line_break: false,
                };
                line.content.insert(0, item);
            }
        }
        result
    }
    /// 解析ParagraphNode同时获取对应的
    fn create_paragraph_inline_items(&mut self, node: &Node) -> Vec<InLineItem> {
        let mut items: Vec<InLineItem> = vec![];
        let mut total_width: usize = 0;
        for child in node.children.iter() {
            let (item, width) = match child.node_type {
                NodeType::NodeTextMark => self.create_node_text_mark(child),
                NodeType::NodeText => self.create_node_text(child),
                _ => continue,
            };
            items.push(item);
            total_width += width
        }
        items
    }
    fn create_node_text(&mut self, node: &Node) -> (InLineItem, usize) {
        let content = node
            .data
            .clone()
            .unwrap_or_default()
            .replace('\u{200b}', "");
        let item = InLineItem {
            item_type: InLineMarkType::Default,
            content,
            link: None,
            style: None,
            line_break: false,
        };
        let width = item.content.width();
        (item, width)
    }
    fn create_node_text_mark(&mut self, node: &Node) -> (InLineItem, usize) {
        let content = node
            .text_mark_text_content
            .clone()
            .unwrap_or_default()
            .replace('\u{200b}', "");
        let mark_type = node.text_mark_type.clone().unwrap_or_default().clone();
        let enum_mark_type = InLineMarkType::from_str(&mark_type).unwrap_or_default();
        let mut item = InLineItem {
            item_type: enum_mark_type,
            content,
            link: None,
            style: None,
            line_break: false,
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
                // http 超链接
                let web_link = node.text_mark_a_href.clone().unwrap_or_default();
                item.link = Some(web_link);
                item.style = Some("node.text.weblink".to_string())
            }
            _ => {}
        }
        let width = item.content.width();
        (item, width)
    }

    /// 将传入的 InLineItem 转换为多个可以直接渲染的 DocumentLine
    /// content: 行内块元素
    /// line_width: 可展示长度
    ///
    /// TODO List 类型的字段
    pub fn split_item_to_document_lines(
        content: Vec<InLineItem>,
        line_width: usize,
    ) -> Vec<DocumentLine> {
        let mut result = Vec::new();
        let mut current_line = Vec::new();
        let mut current_width = 0;
        let mut line_break = false;
        for item in content.into_iter() {
            let item_width = utils::calculate_item_width(&item);
            // 如果当前行加上这个项目不会超出行宽限制
            if current_width + item_width <= line_width {
                current_line.push(item);
                current_width += item_width;
                continue;
            }
            let remaining_width = line_width - current_width;
            let (first_part, second_part) =
                DocumentModel::split_inline_item(&item, remaining_width);

            current_line.push(first_part);
            result.push(DocumentLine::default_with_items(current_line));

            current_width = utils::calculate_item_width(&second_part);
            current_line = vec![second_part];
            while current_width > line_width {
                let second_part = current_line.first().unwrap().clone();
                let (sub_first_part, sub_second_part) =
                    DocumentModel::split_inline_item(&second_part.clone(), line_width);
                let new_line = vec![sub_first_part];
                result.push(DocumentLine {
                    content: new_line,
                    node_type: Default::default(),
                    break_line: false,
                    container: Default::default(),
                    indent_width: 0,
                });
                current_line = vec![sub_second_part];
                current_width = utils::calculate_items_width(&current_line);
            }
        }
        // 添加最后一行
        if !current_line.is_empty() {
            result.push(DocumentLine {
                content: current_line,
                node_type: Default::default(),
                break_line: false,
                container: Default::default(),
                indent_width: 0,
            });
        }
        result
    }

    fn split_inline_item(item: &InLineItem, max_width: usize) -> (InLineItem, InLineItem) {
        let content = &item.content;
        let (split_pos, line_break) = DocumentModel::find_best_split_position(content, max_width);
        let first_part = InLineItem {
            item_type: item.item_type.clone(),
            content: content[..split_pos].to_string(),
            link: None,
            style: item.style.clone(),
            line_break: false,
        };
        let second_part = InLineItem {
            item_type: item.item_type.clone(),
            content: content[split_pos..].to_string(),
            link: None,
            style: item.style.clone(),
            line_break,
        };
        (first_part, second_part)
    }
    /// 返回换行标识,以及当前行是否是因为块内换行符导致的换行
    fn find_best_split_position(content: &str, max_display_width: usize) -> (usize, bool) {
        let mut break_line = false;
        if content.is_empty() {
            return (0, break_line);
        }

        let mut current_display_width = 0;
        let mut last_space_pos = None;
        let mut last_safe_boundary = 0;
        let mut has_chinese = false;
        let mut last_newline_pos = None;

        for (i, c) in content.char_indices() {
            // 首先检查换行符（优先级最高）
            if c == '\n' {
                // 记录换行符位置，但不立即返回,因为换行符前的内容可能超过最大宽度
                last_newline_pos = Some(i);
            }

            // 检测是否中文字符
            if !has_chinese && ('\u{4e00}'..='\u{9fff}').contains(&c) {
                has_chinese = true;
            }

            let char_width = c.width().unwrap_or(1);
            let exceeds_width = current_display_width + char_width > max_display_width;

            // 优先处理换行符（如果存在且未超宽）
            if let Some(newline_pos) = last_newline_pos {
                let safe_newline_pos = newline_pos + 1; // 在换行符后拆分
                                                        // 确保换行符前的内容不超过最大宽度
                if safe_newline_pos <= last_safe_boundary || !exceeds_width {
                    break_line = true;
                    return (safe_newline_pos, break_line);
                }
            }

            // 如果超宽且没有未处理的换行符
            if exceeds_width {
                // 如果有空格且不是中文文本，优先在空格处分隔
                if let Some(space_pos) = last_space_pos.filter(|_| !has_chinese) {
                    return (space_pos + 1, break_line);
                }
                // 否则在当前安全边界处分隔
                return (last_safe_boundary, break_line);
            }

            // 更新当前显示宽度
            current_display_width += char_width;

            // 仅非中文文本记录空格位置
            if !has_chinese && c == ' ' {
                last_space_pos = Some(i);
            }

            // 更新最后一个安全边界
            last_safe_boundary = i + c.len_utf8();
        }

        // 处理文本末尾的换行符
        if let Some(newline_pos) = last_newline_pos {
            return (newline_pos + 1, break_line);
        }

        (content.len(), break_line)
    }
}
impl fmt::Display for DocumentModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, line) in self.lines.iter().enumerate() {
            if i > 0 {
                write!(f, "\n")?; // 行间换行
            }

            // 拼接行内所有内容
            for item in &line.content {
                write!(f, "{}", item.content)?;
            }
        }
        Ok(())
    }
}
impl InLineItem {
    fn default_title(content: String) -> InLineItem {
        InLineItem {
            item_type: InLineMarkType::Default,
            content,
            link: None,
            style: Some("node.heading.title".to_string()),
            line_break: false,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::model::document_model::{DocumentId, DocumentModel};
    use ratatui::layout::Rect;
    use syservice::lute::node::Node;

    fn create_empty_model_with_50x50() -> DocumentModel {
        let rect = Rect::new(0, 0, 50, 50);
        DocumentModel {
            id: DocumentId::default(),
            lines: vec![],
            area: rect,
        }
    }
    fn get_node_by_node_id(node_id: &str) -> Node {
        let mut root_node = syservice::test_utils::load_sy_test_file().unwrap_or_default();
        let mut node: Node = Node::default();
        for child in root_node.children.iter_mut() {
            if child.clone().id.unwrap().eq(node_id) {
                node = child.clone();
                break;
            }
        }
        node.set_node_type_for_tree();
        node
    }
    #[test]
    fn test_create_title_lines() {
        let model = create_empty_model_with_50x50();
        let root_node = syservice::test_utils::load_sy_test_file().unwrap_or_default();
        let title_lines = model.create_doc_title_lines(&root_node);
        assert_eq!(title_lines.len(), 3);
    }
    #[test]
    fn test_create_paragraph_lines() {
        let node_id = "20250512161513-8hypgbv";
        let node: Node = get_node_by_node_id(node_id);
        let mut model = create_empty_model_with_50x50();

        assert_eq!("NodeParagraph", node.type_str);
        let items = model.create_paragraph_inline_items(&node);
        assert!(!items.is_empty());
        let doc_lines =
            DocumentModel::split_item_to_document_lines(items.clone(), model.area.width as usize);
        assert!(!doc_lines.is_empty());
        let m1 = DocumentModel {
            id: Default::default(),
            lines: doc_lines,
            area: Default::default(),
        };
        println!("{}", m1);
    }
    #[test]
    fn test_paragraph_with_longtext_and_linebreak() {
        let doc_id = "20250624150748-3u7nbr1";
        let node: Node = get_node_by_node_id(doc_id);
        let mut model = create_empty_model_with_50x50();

        assert_eq!("NodeParagraph", node.type_str);
        let items = model.create_paragraph_inline_items(&node);
        assert!(!items.is_empty());
        let doc_lines =
            DocumentModel::split_item_to_document_lines(items.clone(), model.area.width as usize);
        assert!(!doc_lines.is_empty());
        let m1 = DocumentModel {
            id: Default::default(),
            lines: doc_lines,
            area: Default::default(),
        };
        println!("{}", m1);
    }

    #[test]
    fn test_list_block() {
        let node_id = "20250512161441-kf86s1d";
        let node: Node = get_node_by_node_id(node_id);

        let mut model = create_empty_model_with_50x50();
        let lines = model.create_list_block_lines(&node);
        assert!(!lines.is_empty());
        let m1 = DocumentModel {
            id: Default::default(),
            lines,
            area: Default::default(),
        };
        println!("{}", m1);
    }
    #[test]
    fn test_list_with_inline_linebreak() {
        let node_id = "20250625152117-33q2b72";
        let node: Node = get_node_by_node_id(node_id);

        let mut model = create_empty_model_with_50x50();
        let lines = model.create_list_block_lines(&node);
        assert!(!lines.is_empty());
        let m1 = DocumentModel {
            id: Default::default(),
            lines,
            area: Default::default(),
        };
        println!("{}", m1);
    }
}
