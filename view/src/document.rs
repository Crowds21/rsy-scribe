use crate::styles::{parse_marks, BaseMark, BaseMarkKind, DecorMark, InlineMarks};
use crate::utils;
use std::default::Default;
use std::fmt;
use std::num::NonZeroUsize;
use std::str::FromStr;
use syservice::lute::node::{Node, NodeType};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// uses NonZeroUsize so Option<DocumentId> use a byte rather than two
/// 用于应用内标识文档.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DocumentId(pub NonZeroUsize);

impl Default for DocumentId {
    fn default() -> DocumentId {
        // Safety: 1 is non-zero
        DocumentId(unsafe { NonZeroUsize::new_unchecked(1) })
    }
}
impl fmt::Display for DocumentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
/// 文档对象.
#[derive(Default)]
pub struct DocumentModel {
    pub id: DocumentId,
    /// 可直接渲染的一行
    pub lines: Vec<DocumentLine>,
    pub node: Option<Node>,
    /// 最大长度
    pub max_line_len: u16,
}

#[derive(Clone, Default)]
pub struct InLineItem {
    /// 复合样式（部分样式互斥 + 部分样式可叠加）
    pub marks: InlineMarks,
    /// 行内元素原始内容
    pub content: String,
    /// 展示内容（已转义）
    pub display_content: String,
    /// 跳转链接（用于 BlockRef 或 HyperLink）
    pub link: Option<String>,
    /// 对应 theme.toml 中的样式名称列表（按优先级：基础样式 + 装饰样式）
    pub styles: Vec<String>,
    /// 是否为块内软换行
    pub line_break: bool,
}
#[derive(Clone, Default)]
pub struct DocumentLine {
    ///    行内需要渲染的元素
    pub content: Vec<InLineItem>,
    /// siyuan 对应的节点类型
    node_type: NodeType,
    /// 是否为块内软换行
    break_line: bool,
    /// 元素外层可能是 quote 或者其他超级块
    container: NodeType,
    /// 缩进.List类型的元素在换行时需要保持缩进
    indent_width: u16,
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
    pub fn open(mut node: Node, id: DocumentId, line_len: u16) -> Self {
        node.set_node_type_for_tree();
        let mut doc = DocumentModel {
            id,
            lines: vec![],
            node: Some(node),
            max_line_len: line_len,
        };
        doc.parse_ast_root_node(line_len);
        doc
    }
    /// 将 SY AST 转换为  DocumentModel 的入口
    ///
    /// TODO 当 resize 事件被触发时,需要重新计算
    pub fn parse_ast_root_node(&mut self, width: u16) {
        if self.node.is_none() {
            return;
        }
        let mut root_node = self.node.take().unwrap();
        root_node.node_type = NodeType::from_str(&root_node.type_str).unwrap();
        let mut doc_title = self.create_doc_title_lines(&root_node);
        self.lines.append(&mut doc_title);

        for child in root_node.children.iter_mut() {
            let mut content_lines = self.parse_node_by_type(child);
            self.lines.append(&mut content_lines);
        }
    }
    fn parse_node_by_type(&mut self, node: &Node) -> Vec<DocumentLine> {
        let available_width = self.max_line_len;
        let mut result: Vec<DocumentLine> = Vec::new();
        let mut lines = match node.node_type {
            // NodeType::Default => {}
            // NodeType::NodeDocument => {}
            NodeType::NodeParagraph => self.create_paragraph_block_lines(node, available_width),
            NodeType::NodeHeading => self.create_head_block_lines(node, available_width),
            // NodeType::NodeHeadingC8hMarker => {}
            // NodeType::NodeThematicBreak => {}
            // NodeType::NodeBlockquote => {}
            // NodeType::NodeBlockquoteMarker => {}
            NodeType::NodeList => self.create_list_block_lines(node, available_width),
            NodeType::NodeCodeBlock => self.create_code_block_lines(node, available_width),
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
    /// 创建标题块对应的行
    fn create_head_block_lines(&mut self, node: &Node, line_width: u16) -> Vec<DocumentLine> {
        // 处理逻辑与 paragraph block 一致
        let items = self.create_inline_items(node);
        let mut content_lines = DocumentModel::split_item_to_document_lines(items, line_width);
        // TODO 标题块整体的颜色样式怎么处理 ? 渲染的时候手动添加?
        //  行内元素的多级嵌套怎么处理
        content_lines
            .iter_mut()
            .for_each(|it| it.node_type = NodeType::NodeHeading);
        // 添加装饰线
        if !content_lines.is_empty() {
            let length = utils::calculate_line_width(content_lines.first().unwrap());
            self.decorate_heading_line(node, length, &mut content_lines)
        }
        content_lines
    }
    fn decorate_heading_line(
        &mut self,
        node: &Node,
        content_length: u16,
        content: &mut Vec<DocumentLine>,
    ) {
        if node.heading_level.is_none() {
            return;
        }
        /// vec<DocumentLine> 前后插入修饰线
        fn add_decoration(
            decorate: &str,
            content: &mut Vec<DocumentLine>,
            length: u16,
        ) {
            let mut before_heading = DocumentLine::default();
            let decoration_line = decorate.repeat(length as usize);
            let decoration_item = InLineItem {
                marks: InlineMarks::default(),
                content: decoration_line.clone(),
                display_content: decoration_line,
                link: None,
                styles: Vec::new(),
                line_break: false,
            };
            before_heading.content = vec![decoration_item];
            let after_heading = before_heading.clone();
            content.insert(0, before_heading);
            content.push(after_heading);
        }
        let level = node.heading_level.unwrap();
        match level {
            1 => add_decoration("=", content, content_length),
            2 => add_decoration("=", content, content_length),
            3 => add_decoration("=", content, content_length),
            4 => add_decoration("~", content, content_length),
            5 => add_decoration("-", content, content_length),
            6 => add_decoration("-", content, content_length),
            _ => {}
        }
    }
    fn create_paragraph_block_lines(
        &mut self,
        node: &Node,
        available_width: u16,
    ) -> Vec<DocumentLine> {
        let items = self.create_inline_items(node);
        DocumentModel::split_item_to_document_lines(items, available_width)
    }
    fn create_list_block_lines(&mut self, node: &Node, line_width: u16) -> Vec<DocumentLine> {
        let mut lines: Vec<DocumentLine> = Vec::new();
        // TODO 这里需要做层级的计算
        for child in node.children.iter() {
            let mut temp_result = self.crate_list_iterators(child, line_width);
            lines.append(&mut temp_result);
        }
        lines
    }

    /// 创建代码块行
    fn create_code_block_lines(&mut self, node: &Node, available_width: u16) -> Vec<DocumentLine> {
        use crate::code_block::{parse_code_block, render_code_block, get_code_block_style};
        use std::collections::HashMap;

        let code_block = parse_code_block(node);
        
        // 使用默认样式（未来可以从主题加载）
        let style = get_code_block_style(&HashMap::new());
        
        // 使用无高亮渲染器
        let items = render_code_block(&code_block, available_width, &style, None);
        
        // 转换为 DocumentLine
        let mut lines = Vec::new();
        let mut current_line_items = Vec::new();
        
        for item in items {
            if item.line_break && !current_line_items.is_empty() {
                lines.push(DocumentLine::default_with_items(current_line_items));
                current_line_items = Vec::new();
            }
            current_line_items.push(item);
        }

        
        if !current_line_items.is_empty() {
            lines.push(DocumentLine::default_with_items(current_line_items));
        }
        
        lines
    }
    fn crate_list_iterators(&mut self, node: &Node, available_width: u16) -> Vec<DocumentLine> {
        // TODO 这里返回的是 ListItem 中等待展示的Lines
        //  所以只需要对第一行插入BulletChar. 其他行插入等量的空格符
        //  但是这里没法考虑多级缩进的问题
        //  否则就需要在创建 Paragraph 前就获取到缩进,并且知道是第几层缩进
        let mut lines: Vec<DocumentLine> = Vec::new();
        let temp_result = match node.node_type {
            NodeType::NodeList => self.create_list_block_lines(node, available_width),
            NodeType::NodeListItem => self.create_list_item_block(node, available_width),
            NodeType::NodeParagraph => self.create_paragraph_block_lines(node, available_width),
            // TODO
            // NodeType::NodeCodeBlock
            _ => Vec::new(),
        };
        lines = temp_result;
        lines
    }
    fn create_list_item_block(&mut self, node: &Node, available_width: u16) -> Vec<DocumentLine> {
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
                    marks: InlineMarks::default(),
                    content: format!("{} ", bullet_char),
                    display_content: format!("{} ", bullet_char),
                    link: None,
                    styles: Vec::new(),
                    line_break: false,
                };
                line.content.insert(0, item);
            } else {
                let item = InLineItem {
                    marks: InlineMarks::default(),
                    content: "  ".to_string(),
                    display_content: "  ".to_string(),
                    link: None,
                    styles: Vec::new(),
                    line_break: false,
                };
                line.content.insert(0, item);
            }
        }
        result
    }
    /// 解析行内元素
    fn create_inline_items(&mut self, node: &Node) -> Vec<InLineItem> {
        let mut items: Vec<InLineItem> = vec![];
        let mut total_width: u16 = 0;
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
    fn create_node_text(&mut self, node: &Node) -> (InLineItem, u16) {
        let content = node
            .data
            .clone()
            .unwrap_or_default()
            .replace('\u{200b}', "");
        // 将 html 字符转换为 unicode 字符
        let after_parse = html_escape::decode_html_entities(&content);
        let display_content = after_parse.to_string();
        let width = display_content.width();
        let item = InLineItem {
            marks: InlineMarks::default(),
            content: content.to_string(),
            display_content,
            link: None,
            styles: Vec::new(),
            line_break: false,
        };
        (item, width as u16)
    }
    /// 为带有行内样式的 item 添加样式
    fn create_node_text_mark(&mut self, node: &Node) -> (InLineItem, u16) {
        let content = node
            .text_mark_text_content
            .clone()
            .unwrap_or_default()
            .replace('\u{200b}', "");
        let after_parse = html_escape::decode_html_entities(&content);
        let display_content = after_parse.to_string();
        
        // 解析复合样式（如 "strong em code"）
        let mark_type_str = node.text_mark_type.clone().unwrap_or_default();
        let marks = parse_marks(&mark_type_str);
        
        // 构建样式列表：基础样式 + 装饰样式
        let mut styles: Vec<String> = Vec::new();
        
        // 1. 添加基础样式（互斥，只选一个）
        let link = match marks.base_kind() {
            BaseMarkKind::Mark => {
                styles.push("node.text.mark".to_string());
                None
            }
            BaseMarkKind::Code => {
                styles.push("node.text.code".to_string());
                None
            }
            BaseMarkKind::BlockRef => {
                styles.push("node.text.blockref".to_string());
                let block_ref = node.text_mark_block_ref_id.clone().unwrap_or_default();
                Some(block_ref)
            }
            BaseMarkKind::A => {
                styles.push("node.text.weblink".to_string());
                let web_link = node.text_mark_a_href.clone().unwrap_or_default();
                Some(web_link)
            }
            BaseMarkKind::Tag => {
                styles.push("node.text.tag".to_string());
                None
            }
            BaseMarkKind::Default => None,
        };
        
        // 2. 添加装饰样式（可叠加）
        if marks.decor.contains(DecorMark::STRONG) {
            styles.push("node.text.strong".to_string());
        }
        if marks.decor.contains(DecorMark::EM) {
            styles.push("node.text.italic".to_string());
        }
        if marks.decor.contains(DecorMark::U) {
            styles.push("node.text.underline".to_string());
        }
        if marks.decor.contains(DecorMark::S) {
            styles.push("node.text.strikethrough".to_string());
        }
        
        let item = InLineItem {
            marks,
            content: content.to_string(),
            display_content,
            link,
            styles,
            line_break: false,
        };
        let width = item.display_content.width();
        (item, width as u16)
    }

    /// 将传入的 InLineItem 转换为多个可以直接渲染的 DocumentLine
    /// content: 行内块元素
    /// line_width: 可展示长度
    ///
    /// TODO List 类型的字段
    pub fn split_item_to_document_lines(
        content: Vec<InLineItem>,
        line_width: u16,
    ) -> Vec<DocumentLine> {
        let mut result = Vec::new();
        let mut current_line = Vec::new();
        let mut current_width: u16 = 0;
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

    fn split_inline_item(item: &InLineItem, max_width: u16) -> (InLineItem, InLineItem) {
        let content = &item.display_content;
        let (split_pos, line_break) = DocumentModel::find_best_split_position(content, max_width);
        let first_display = content[..split_pos as usize].to_string();
        let second_display = content[split_pos as usize..].to_string();
        
        let first_part = InLineItem {
            marks: item.marks,
            content: item.content[..split_pos as usize].to_string(),
            display_content: first_display,
            link: item.link.clone(),
            styles: item.styles.clone(),
            line_break: false,
        };
        let second_part = InLineItem {
            marks: item.marks,
            content: item.content[split_pos as usize..].to_string(),
            display_content: second_display,
            link: item.link.clone(),
            styles: item.styles.clone(),
            line_break,
        };
        (first_part, second_part)
    }
    /// 返回换行标识,以及当前行是否是因为块内换行符导致的换行
    fn find_best_split_position(content: &str, max_display_width: u16) -> (u16, bool) {
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
            // 首先检查换行符(优先级最高)
            if c == '\n' {
                // 记录换行符位置,但不立即返回,因为换行符前的内容可能超过最大宽度
                last_newline_pos = Some(i);
            }

            // 检测是否中文字符
            if !has_chinese && ('\u{4e00}'..='\u{9fff}').contains(&c) {
                has_chinese = true;
            }

            let char_width = c.width().unwrap_or(1) as u16;
            let exceeds_width = current_display_width + char_width > max_display_width;

            // 优先处理换行符(如果存在且未超宽)
            if let Some(newline_pos) = last_newline_pos {
                let safe_newline_pos = newline_pos + 1; // 在换行符后拆分
                                                        // 确保换行符前的内容不超过最大宽度
                if safe_newline_pos <= last_safe_boundary || !exceeds_width {
                    break_line = true;
                    return (safe_newline_pos as u16, break_line);
                }
            }

            // 如果超宽且没有未处理的换行符
            if exceeds_width {
                // 如果有空格且不是中文文本,优先在空格处分隔
                if let Some(space_pos) = last_space_pos.filter(|_| !has_chinese) {
                    return (space_pos as u16 + 1, break_line);
                }
                // 否则在当前安全边界处分隔
                return (last_safe_boundary as u16, break_line);
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
            return (newline_pos as u16 + 1, break_line);
        }

        (content.len() as u16, break_line)
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
            marks: InlineMarks::default(),
            content: content.clone(),
            display_content: content,
            link: None,
            styles: vec!["node.heading.title".to_string()],
            line_break: false,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::document::{DocumentId, DocumentModel};
    use ratatui::layout::Rect;
    use syservice::lute::node::Node;

    fn create_empty_model_with_50x50() -> DocumentModel {
        let rect = Rect::new(0, 0, 50, 50);
        DocumentModel {
            id: DocumentId::default(),
            lines: vec![],
            node: None,
            max_line_len: 0,
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
        let rect = Rect::new(0, 0, 50, 50);
        let node: Node = get_node_by_node_id(node_id);
        let mut model = create_empty_model_with_50x50();

        assert_eq!("NodeParagraph", node.type_str);
        let items = model.create_inline_items(&node);
        assert!(!items.is_empty());
        let doc_lines = DocumentModel::split_item_to_document_lines(items.clone(), rect.width);
        assert!(!doc_lines.is_empty());
        let m1 = DocumentModel {
            id: Default::default(),
            lines: doc_lines,
            node: None,
            max_line_len: 0,
        };
        println!("{}", m1);
    }
    #[test]
    fn test_paragraph_with_longtext_and_linebreak() {
        let rect = Rect::new(0, 0, 50, 50);
        let doc_id = "20250624150748-3u7nbr1";
        let node: Node = get_node_by_node_id(doc_id);
        let mut model = create_empty_model_with_50x50();

        assert_eq!("NodeParagraph", node.type_str);
        let items = model.create_inline_items(&node);
        assert!(!items.is_empty());
        let doc_lines = DocumentModel::split_item_to_document_lines(items.clone(), rect.width);
        assert!(!doc_lines.is_empty());
        let m1 = DocumentModel {
            id: Default::default(),
            lines: doc_lines,
            node: None,
            max_line_len: 0,
        };
        println!("{}", m1);
    }

    #[test]
    fn test_list_block() {
        let node_id = "20250512161441-kf86s1d";
        let rect = Rect::new(0, 0, 50, 50);
        let node: Node = get_node_by_node_id(node_id);

        let mut model = create_empty_model_with_50x50();
        let lines = model.create_list_block_lines(&node, rect.width as u16);
        assert!(!lines.is_empty());
        let m1 = DocumentModel {
            id: Default::default(),
            lines,
            node: None,
            max_line_len: 0,
        };
        println!("{}", m1);
    }
    #[test]
    fn test_list_with_inline_linebreak() {
        let node_id = "20250625152117-33q2b72";
        let rect = Rect::new(0, 0, 50, 50);
        let node: Node = get_node_by_node_id(node_id);

        let mut model = create_empty_model_with_50x50();

        let lines = model.create_list_block_lines(&node, rect.width);
        assert!(!lines.is_empty());
        let m1 = DocumentModel {
            id: Default::default(),
            lines,
            node: None,
            max_line_len: 0,
        };
        println!("{}", m1);
    }

    #[test]
    fn test_lib_html_escape(){
        // html_escape 只解码 HTML 实体，不解码 Unicode 零宽空格
        let str = "&lt;";
        let after_parse = html_escape::decode_html_entities(&str);
        assert_eq!("<", after_parse);
        
        // 零宽空格需要手动替换
        let str_with_zwsp = "\u{200b}&lt;";
        let after_replace = str_with_zwsp.replace('\u{200b}', "");
        let after_parse2 = html_escape::decode_html_entities(&after_replace);
        assert_eq!("<", after_parse2);
    }
}
