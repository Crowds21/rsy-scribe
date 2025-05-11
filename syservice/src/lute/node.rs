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
use std::collections::{HashMap, HashSet};
use serde_bytes::ByteBuf;

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
    
    #[serde(rename="Children",default,skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<Node>,

    #[serde(skip)]
    pub tokens: ByteBuf,

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
    #[serde(default,skip_serializing_if = "Vec::is_empty")]
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

    #[serde(default,skip_serializing_if = "Vec::is_empty")]
    pub footnotes_refs: Vec<Node>,

    // HTML 实体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_entity_tokens: Option<ByteBuf>,

    // 属性
    #[serde(skip)]
    pub kramdown_ial: Vec<Vec<String>>,

    #[serde(rename="Properties" ,skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, String>>,

    // 文本标记
    #[serde(rename="TextMarkType",skip_serializing_if = "Option::is_none")]
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


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListData {
    #[serde(rename = "Typ", skip_serializing_if = "Option::is_none")]
    pub typ: Option<i32>, // 0:无序列表, 1:有序列表, 3:任务列表

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tight: Option<bool>, // 是否紧凑模式

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

    #[serde(rename="Marker",skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize,Default)]
#[repr(i32)]
#[serde(rename_all = "camelCase")]
pub enum NodeType {
    #[default]
    Default=-1,
    // CommonMark
    Document = 0,
    Paragraph = 1,
    Heading = 2,
    HeadingC8hMarker = 3,
    ThematicBreak = 4,
    Blockquote = 5,
    BlockquoteMarker = 6,
    List = 7,
    ListItem = 8,
    HtmlBlock = 9,
    InlineHtml = 10,
    CodeBlock = 11,
    CodeBlockFenceOpenMarker = 12,
    CodeBlockFenceCloseMarker = 13,
    CodeBlockFenceInfoMarker = 14,
    CodeBlockCode = 15,
    Text = 16,
    Emphasis = 17,
    EmA6kOpenMarker = 18,
    EmA6kCloseMarker = 19,
    EmU8eOpenMarker = 20,
    EmU8eCloseMarker = 21,
    Strong = 22,
    StrongA6kOpenMarker = 23,
    StrongA6kCloseMarker = 24,
    StrongU8eOpenMarker = 25,
    StrongU8eCloseMarker = 26,
    CodeSpan = 27,
    CodeSpanOpenMarker = 28,
    CodeSpanContent = 29,
    CodeSpanCloseMarker = 30,
    HardBreak = 31,
    SoftBreak = 32,
    Link = 33,
    Image = 34,
    Bang = 35,
    OpenBracket = 36,
    CloseBracket = 37,
    OpenParen = 38,
    CloseParen = 39,
    LinkText = 40,
    LinkDest = 41,
    LinkTitle = 42,
    LinkSpace = 43,
    HtmlEntity = 44,
    LinkRefDefBlock = 45,
    LinkRefDef = 46,
    Less = 47,
    Greater = 48,

    // GFM (100-199)
    TaskListItemMarker = 100,
    Strikethrough = 101,
    Strikethrough1OpenMarker = 102,
    Strikethrough1CloseMarker = 103,
    Strikethrough2OpenMarker = 104,
    Strikethrough2CloseMarker = 105,
    Table = 106,
    TableHead = 107,
    TableRow = 108,
    TableCell = 109,

    // Emoji (200-299)
    Emoji = 200,
    EmojiUnicode = 201,
    EmojiImg = 202,
    EmojiAlias = 203,

    // Math (300-399)
    MathBlock = 300,
    MathBlockOpenMarker = 301,
    MathBlockContent = 302,
    MathBlockCloseMarker = 303,
    InlineMath = 304,
    InlineMathOpenMarker = 305,
    InlineMathContent = 306,
    InlineMathCloseMarker = 307,

    // Escape (400-404)
    Backslash = 400,
    BackslashContent = 401,

    // Vditor (405-409)
    VditorCaret = 405,

    // Footnotes (410-414)
    FootnotesDefBlock = 410,
    FootnotesDef = 411,
    FootnotesRef = 412,

    // TOC (415-419)
    Toc = 415,

    // Heading ID (420-424)
    HeadingId = 420,

    // YAML Front Matter (425-429)
    YamlFrontMatter = 425,
    YamlFrontMatterOpenMarker = 426,
    YamlFrontMatterContent = 427,
    YamlFrontMatterCloseMarker = 428,

    // Block Reference (430-449)
    BlockRef = 430,
    BlockRefId = 431,
    BlockRefSpace = 432,
    BlockRefText = 433,
    BlockRefDynamicText = 434,

    // Mark (450-454)
    Mark = 450,
    Mark1OpenMarker = 451,
    Mark1CloseMarker = 452,
    Mark2OpenMarker = 453,
    Mark2CloseMarker = 454,

    // Kramdown IAL (455-459)
    KramdownBlockIal = 455,
    KramdownSpanIal = 456,

    // Tag (460-464)
    Tag = 460,
    TagOpenMarker = 461,
    TagCloseMarker = 462,

    // Block Query (465-474)
    BlockQueryEmbed = 465,
    OpenBrace = 466,
    CloseBrace = 467,
    BlockQueryEmbedScript = 468,

    // Super Block (475-484)
    SuperBlock = 475,
    SuperBlockOpenMarker = 476,
    SuperBlockLayoutMarker = 477,
    SuperBlockCloseMarker = 478,

    // Sup/Sub (485-494)
    Sup = 485,
    SupOpenMarker = 486,
    SupCloseMarker = 487,
    Sub = 490,
    SubOpenMarker = 491,
    SubCloseMarker = 492,

    // Git Conflict (495-499)
    GitConflict = 495,
    GitConflictOpenMarker = 496,
    GitConflictContent = 497,
    GitConflictCloseMarker = 498,

    // Media (500-529)
    IFrame = 500,
    Audio = 505,
    Video = 510,
    Kbd = 515,
    KbdOpenMarker = 516,
    KbdCloseMarker = 517,
    Underline = 520,
    UnderlineOpenMarker = 521,
    UnderlineCloseMarker = 522,
    Br = 525,
    TextMark = 530,
    Widget = 535,

    // File Annotation (540-549)
    FileAnnotationRef = 540,
    FileAnnotationRefId = 541,
    FileAnnotationRefSpace = 542,
    FileAnnotationRefText = 543,

    // Attribute View (550-559)
    AttributeView = 550,

    // Custom Block (560-569)
    CustomBlock = 560,

    // HTML Tags (570-599)
    HtmlTag = 570,
    HtmlTagOpen = 571,
    HtmlTagClose = 572,

    // Max Value
    MaxVal = 1024,
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
            NodeType::TagOpenMarker | NodeType::TagCloseMarker => "#",
            NodeType::EmA6kOpenMarker | NodeType::EmA6kCloseMarker => "*",
            NodeType::EmU8eOpenMarker | NodeType::EmU8eCloseMarker => "_",
            NodeType::StrongA6kOpenMarker | NodeType::StrongA6kCloseMarker => "**",
            NodeType::StrongU8eOpenMarker | NodeType::StrongU8eCloseMarker => "__",
            NodeType::Strikethrough2OpenMarker | NodeType::Strikethrough2CloseMarker => "~~",
            NodeType::SupOpenMarker | NodeType::SupCloseMarker => "^",
            NodeType::SubOpenMarker | NodeType::SubCloseMarker => "~",
            NodeType::InlineMathOpenMarker | NodeType::InlineMathCloseMarker => "$",
            NodeType::KbdOpenMarker => "<kbd>",
            NodeType::KbdCloseMarker => "</kbd>",
            NodeType::UnderlineOpenMarker => "<u>",
            NodeType::UnderlineCloseMarker => "</u>",
            NodeType::Mark2OpenMarker | NodeType::Mark2CloseMarker => "==",
            NodeType::Bang => "!",
            NodeType::OpenBracket => "[",
            NodeType::CloseBracket => "]",
            NodeType::OpenParen => "(",
            NodeType::CloseParen => ")",
            _ => "",
        }
    }
}
