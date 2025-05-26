// Lute - 一款结构化的 Markdown 引擎，支持 Go 和 JavaScript
// Copyright (c) 2019-present, b3log.org
//
// Lute is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan PSL v2.
// You may obtain a copy of Mulan PSL v2 at:
//         http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.
use chrono::Local;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use strum::EnumString;

// type NodeType = i32;
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Node {
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip)]
    pub box_: String,

    #[serde(skip)]
    pub path: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<String>,

    #[serde(skip)]
    pub node_type: NodeType,

    #[serde(skip)]
    pub parent: Option<Box<Node>>,

    #[serde(skip)]
    pub previous: Option<Box<Node>>,

    #[serde(skip)]
    pub next: Option<Box<Node>>,

    #[serde(skip)]
    pub first_child: Option<Box<Node>>,

    #[serde(skip)]
    pub last_child: Option<Box<Node>>,

    #[serde(rename = "Children", default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,

    #[serde(skip)]
    pub tokens: ByteBuf,

    /// 通过该字段来获取 Type 类型
    #[serde(rename = "Type")]
    pub type_str: String,

    #[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,

    // 解析过程标识
    #[serde(skip)]
    pub close: bool,

    #[serde(skip)]
    pub last_line_blank: bool,

    #[serde(skip)]
    pub last_line_checked: bool,

    // 代码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_marker_len: Option<i32>,

    // 代码块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_fenced_code_block: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_block_fence_char: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_block_fence_len: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_block_fence_offset: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_block_open_fence: Option<ByteBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_block_info: Option<ByteBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_block_close_fence: Option<ByteBuf>,

    // HTML 块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_block_type: Option<i32>,

    // 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_data: Option<ListData>,

    // 任务列表项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list_item_checked: Option<bool>,

    // 表
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub table_aligns: Vec<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_align: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_content_width: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_cell_content_max_width: Option<i32>,

    // 链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_type: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_ref_label: Option<ByteBuf>,

    // 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading_level: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading_setext: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading_normalized_id: Option<String>,

    // 数学公式块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math_block_dollar_offset: Option<i32>,

    // 脚注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footnotes_ref_label: Option<ByteBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub footnotes_ref_id: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub footnotes_refs: Vec<Node>,

    // HTML 实体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_entity_tokens: Option<ByteBuf>,

    // 属性
    #[serde(skip)]
    pub kramdown_ial: Vec<Vec<String>>,

    #[serde(rename = "Properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,

    // 文本标记
    #[serde(rename = "TextMarkType", skip_serializing_if = "Option::is_none")]
    pub text_mark_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mark_a_href: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mark_a_title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mark_inline_math_content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mark_inline_memo_content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mark_block_ref_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mark_block_ref_subtype: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mark_file_annotation_ref_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_mark_text_content: Option<String>,

    // 属性视图
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_view_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_view_type: Option<String>,

    // 自定义块
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_block_fence_offset: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_block_info: Option<String>,
}

impl Node {
    pub fn create_doc_component(&self) {}
    /// 递归设置节点类型
    pub fn set_node_type_for_tree(&mut self) {
        if let NodeType::Default = self.node_type {
            self.node_type = NodeType::from_str(&self.type_str).unwrap();
            for child in &mut self.children {
                child.set_node_type_for_tree();
            }
        }
    }
    
    pub fn has_child(&self) ->bool{
        self.children.len() > 0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListData {
    #[serde(rename = "Typ", skip_serializing_if = "Option::is_none")]
    pub typ: Option<i32>, // 0:无序列表, 1:有序列表, 3:任务列表

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tight: Option<bool>, // 是否紧凑模式

    /// ASCII 下对应的值
    #[serde(rename = "BulletChar", skip_serializing_if = "Option::is_none")]
    pub bullet_char: Option<u8>, // 无序列表标识(*/-/+)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>, // 有序列表起始序号

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<u8>, // 有序列表分隔符(./))

    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding: Option<i32>, // 内部缩进空格数(W+N)

    #[serde(rename = "MarkerOffset", skip_serializing_if = "Option::is_none")]
    pub marker_offset: Option<i32>, // 标识符相对缩进

    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>, // 任务列表项是否勾选

    /// Base64 下的 List 前缀标识
    #[serde(rename = "Marker", skip_serializing_if = "Option::is_none")]
    pub marker: Option<ByteBuf>, // 列表标识符原始字节

    #[serde(rename = "Num", skip_serializing_if = "Option::is_none")]
    pub num: Option<i32>, // 有序列表项修正序号
}

pub fn new_node_id() -> String {
    let now = Local::now();
    format!(
        "{}-{}",
        now.format("%Y%m%d%H%M%S"), // 等价于 Go 的 20060102150405
        rand_str(7)                 // 7位随机字母数字
    )
}

/// 生成指定长度的随机字符串（a-z0-9）
fn rand_str(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();

    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

pub fn is_node_id_pattern(s: &str) -> bool {
    // 检查总长度
    if s.len() != "20060102150405-1a2b3c4".len() {
        return false;
    }

    // 检查分隔符数量
    if s.matches('-').count() != 1 {
        return false;
    }

    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 2 {
        return false;
    }

    let id_part = parts[0];
    let rand_part = parts[1];

    // 验证时间部分（14位数字）
    if id_part.len() != 14 || !id_part.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    // 验证随机部分（7位字母数字）
    if rand_part.len() != 7
        || !rand_part
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    {
        return false;
    }

    true
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default, EnumString)]
#[repr(i32)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "PascalCase")]
#[strum(ascii_case_insensitive)]
pub enum NodeType {
    #[default]
    Default = -1,
    // CommonMark
    NodeDocument = 0,
    /// 可独立展示的段落(思源中的一个 Block)
    /// Ratatui - Line
    NodeParagraph = 1,
    NodeHeading = 2,
    NodeHeadingC8hMarker = 3,
    NodeThematicBreak = 4,
    NodeBlockquote = 5,
    NodeBlockquoteMarker = 6,
    NodeList = 7,
    /// 列表项.
    NodeListItem = 8,
    NodeHtmlBlock = 9,
    NodeInlineHtml = 10,
    NodeCodeBlock = 11,
    NodeCodeBlockFenceOpenMarker = 12,
    NodeCodeBlockFenceCloseMarker = 13,
    NodeCodeBlockFenceInfoMarker = 14,
    NodeCodeBlockCode = 15,
    NodeText = 16,
    NodeEmphasis = 17,
    NodeEmA6kOpenMarker = 18,
    NodeEmA6kCloseMarker = 19,
    NodeEmU8eOpenMarker = 20,
    NodeEmU8eCloseMarker = 21,
    NodeStrong = 22,
    NodeStrongA6kOpenMarker = 23,
    NodeStrongA6kCloseMarker = 24,
    NodeStrongU8eOpenMarker = 25,
    NodeStrongU8eCloseMarker = 26,
    NodeCodeSpan = 27,
    NodeCodeSpanOpenMarker = 28,
    NodeCodeSpanContent = 29,
    NodeCodeSpanCloseMarker = 30,
    NodeHardBreak = 31,
    NodeSoftBreak = 32,
    NodeLink = 33,
    NodeImage = 34,
    NodeBang = 35,
    NodeOpenBracket = 36,
    NodeCloseBracket = 37,
    NodeOpenParen = 38,
    NodeCloseParen = 39,
    NodeLinkText = 40,
    NodeLinkDest = 41,
    NodeLinkTitle = 42,
    NodeLinkSpace = 43,
    NodeHtmlEntity = 44,
    NodeLinkRefDefBlock = 45,
    NodeLinkRefDef = 46,
    NodeLess = 47,
    NodeGreater = 48,

    // GFM (100-199)
    NodeTaskListItemMarker = 100,
    NodeStrikethrough = 101,
    NodeStrikethrough1OpenMarker = 102,
    NodeStrikethrough1CloseMarker = 103,
    NodeStrikethrough2OpenMarker = 104,
    NodeStrikethrough2CloseMarker = 105,
    NodeTable = 106,
    NodeTableHead = 107,
    NodeTableRow = 108,
    NodeTableCell = 109,

    // Emoji (200-299)
    NodeEmoji = 200,
    NodeEmojiUnicode = 201,
    NodeEmojiImg = 202,
    NodeEmojiAlias = 203,

    // Math (300-399)
    NodeMathBlock = 300,
    NodeMathBlockOpenMarker = 301,
    NodeMathBlockContent = 302,
    NodeMathBlockCloseMarker = 303,
    NodeInlineMath = 304,
    NodeInlineMathOpenMarker = 305,
    NodeInlineMathContent = 306,
    NodeInlineMathCloseMarker = 307,

    // Escape (400-404)
    NodeBackslash = 400,
    NodeBackslashContent = 401,

    // Vditor (405-409)
    NodeVditorCaret = 405,

    // Footnotes (410-414)
    NodeFootnotesDefBlock = 410,
    NodeFootnotesDef = 411,
    NodeFootnotesRef = 412,

    // TOC (415-419)
    NodeToc = 415,

    // Heading ID (420-424)
    NodeHeadingId = 420,

    // YAML Front Matter (425-429)
    NodeYamlFrontMatter = 425,
    NodeYamlFrontMatterOpenMarker = 426,
    NodeYamlFrontMatterContent = 427,
    NodeYamlFrontMatterCloseMarker = 428,

    // Block Reference (430-449)
    NodeBlockRef = 430,
    NodeBlockRefId = 431,
    NodeBlockRefSpace = 432,
    NodeBlockRefText = 433,
    NodeBlockRefDynamicText = 434,

    // Mark (450-454)
    NodeMark = 450,
    NodeMark1OpenMarker = 451,
    NodeMark1CloseMarker = 452,
    NodeMark2OpenMarker = 453,
    NodeMark2CloseMarker = 454,

    // Kramdown IAL (455-459)
    NodeKramdownBlockIal = 455,
    NodeKramdownSpanIal = 456,

    // Tag (460-464)
    NodeTag = 460,
    NodeTagOpenMarker = 461,
    NodeTagCloseMarker = 462,

    // Block Query (465-474)
    NodeBlockQueryEmbed = 465,
    NodeOpenBrace = 466,
    NodeCloseBrace = 467,
    NodeBlockQueryEmbedScript = 468,

    // Super Block (475-484)
    NodeSuperBlock = 475,
    NodeSuperBlockOpenMarker = 476,
    NodeSuperBlockLayoutMarker = 477,
    NodeSuperBlockCloseMarker = 478,

    // Sup/Sub (485-494)
    NodeSup = 485,
    NodeSupOpenMarker = 486,
    NodeSupCloseMarker = 487,
    NodeSub = 490,
    NodeSubOpenMarker = 491,
    NodeSubCloseMarker = 492,

    // Git Conflict (495-499)
    NodeGitConflict = 495,
    NodeGitConflictOpenMarker = 496,
    NodeGitConflictContent = 497,
    NodeGitConflictCloseMarker = 498,

    // Media (500-529)
    NodeIFrame = 500,
    NodeAudio = 505,
    NodeVideo = 510,
    NodeKbd = 515,
    NodeKbdOpenMarker = 516,
    NodeKbdCloseMarker = 517,
    NodeUnderline = 520,
    NodeUnderlineOpenMarker = 521,
    NodeUnderlineCloseMarker = 522,
    NodeBr = 525,
    NodeTextMark = 530,
    NodeWidget = 535,

    // File Annotation (540-549)
    NodeFileAnnotationRef = 540,
    NodeFileAnnotationRefId = 541,
    NodeFileAnnotationRefSpace = 542,
    NodeFileAnnotationRefText = 543,

    // Attribute View (550-559)
    NodeAttributeView = 550,

    // Custom Block (560-569)
    NodeCustomBlock = 560,

    // HTML Tags (570-599)
    NodeHtmlTag = 570,
    NodeHtmlTagOpen = 571,
    NodeHtmlTagClose = 572,

    // Max Value
    NodeMaxVal = 1024,
}
impl From<NodeType> for i32 {
    fn from(val: NodeType) -> Self {
        val as i32
    }
}

impl Node {
    pub fn marker(&self, entering: bool) -> &'static str {
        if !entering {
            return "";
        }
        match self.node_type {
            NodeType::NodeTagOpenMarker | NodeType::NodeTagCloseMarker => "#",
            NodeType::NodeEmA6kOpenMarker | NodeType::NodeEmA6kCloseMarker => "*",
            NodeType::NodeEmU8eOpenMarker | NodeType::NodeEmU8eCloseMarker => "_",
            NodeType::NodeStrongA6kOpenMarker | NodeType::NodeStrongA6kCloseMarker => "**",
            NodeType::NodeStrongU8eOpenMarker | NodeType::NodeStrongU8eCloseMarker => "__",
            NodeType::NodeStrikethrough2OpenMarker | NodeType::NodeStrikethrough2CloseMarker => {
                "~~"
            }
            NodeType::NodeSupOpenMarker | NodeType::NodeSupCloseMarker => "^",
            NodeType::NodeSubOpenMarker | NodeType::NodeSubCloseMarker => "~",
            NodeType::NodeInlineMathOpenMarker | NodeType::NodeInlineMathCloseMarker => "$",
            NodeType::NodeKbdOpenMarker => "<kbd>",
            NodeType::NodeKbdCloseMarker => "</kbd>",
            NodeType::NodeUnderlineOpenMarker => "<u>",
            NodeType::NodeUnderlineCloseMarker => "</u>",
            NodeType::NodeMark2OpenMarker | NodeType::NodeMark2CloseMarker => "==",
            NodeType::NodeBang => "!",
            NodeType::NodeOpenBracket => "[",
            NodeType::NodeCloseBracket => "]",
            NodeType::NodeOpenParen => "(",
            NodeType::NodeCloseParen => ")",
            _ => "",
        }
    }
}
