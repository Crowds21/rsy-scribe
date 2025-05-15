use crate::lute::delimiter::Delimiter;
use crate::lute::lexer::Lexer;
use crate::lute::node::Node;
use chrono::{DateTime, Utc};
use serde_bytes::ByteBuf;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct Tree {
    // 使用 Option 表示可能为空的字段
    pub root: Option<Node>,
    pub context: Box<Option<Context>>,
    pub lexer: Option<Lexer>,
    pub inline_context: Option<InlineContext>,

    // 字符串类型字段
    pub name: String,
    pub id: String,
    pub container: String,  // 原Box字段，Rust中避免使用关键字命名
    pub path: PathBuf,      // 更适合路径处理的类型
    pub hpath: String,      // 人类可读路径
    pub marks: Vec<String>, // 字符串数组

    // 时间处理（使用chrono库）
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,

    // 哈希值
    pub hash: String,
}

pub struct Context {
    // 使用智能指针管理循环引用
    tree: Tree,                     // 使用Arc实现多所有权
    parse_options: Option<Options>, // 可空的解析选项

    // 节点引用
    tip: Node,
    old_tip: Node, // Rust命名规范使用下划线

    // 行数据
    current_line: Vec<u8>,   // 使用Vec<u8>替代[]byte
    current_line_len: usize, // Rust用usize替代int

    // 解析状态
    offset: usize,
    column: usize,
    next_nonspace: usize,
    next_nonspace_column: usize,
    indent: usize,

    // 状态标志
    indented: bool,
    blank: bool,
    partially_consumed_tab: bool,
    all_closed: bool,

    // 容器节点
    last_matched_container: Node,
    root_ial: Node, // 使用Option明确表示可空
}

pub struct InlineContext {
    tokens: ByteBuf, // 当前解析的 Tokens（生命周期管理）
    tokens_len: usize,
    pos: usize,            // 当前解析位置
    delimiters: Delimiter, // 分隔符栈（使用VecDeque实现高效两端操作）
    brackets: Delimiter,   // 括号栈
}

#[derive(Debug, Clone, Default)]
pub struct Options {
    // GFM 扩展
    /// GFMTable 设置是否打开“GFM 表”支持。
    pub gfm_table: bool,
    /// GFMTaskListItem 设置是否打开“GFM 任务列表项”支持。
    pub gfm_task_list_item: bool,
    /// GFMStrikethrough 设置是否打开“GFM 删除线”支持。
    pub gfm_strikethrough: bool,
    pub gfm_strikethrough_single: bool,
    pub gfm_auto_link: bool,

    // 文档结构扩展
    pub footnotes: bool,
    pub heading_id: bool,
    pub toc: bool,
    pub yaml_front_matter: bool,
    pub block_ref: bool,
    pub file_annotation_ref: bool,
    pub super_block: bool,
    pub indent_code_block: bool,

    // 行内元素扩展
    pub inline_math: bool,
    pub inline_math_allow_digit_after_open: bool,
    pub mark: bool,
    pub sup: bool,
    pub sub: bool,
    pub tag: bool,
    pub link_ref: bool,

    // 格式化扩展
    pub setext_heading: bool,
    pub paragraph_beginning_space: bool,
    pub img_path_allow_space: bool,
    pub data_image: bool,

    // kramdown 扩展
    pub kramdown_block_ial: bool,
    pub kramdown_span_ial: bool,

    // 兼容性扩展
    pub inline_asterisk: bool,
    pub inline_underscore: bool,
    pub git_conflict: bool,

    // 富文本处理
    pub text_mark: bool,
    pub html_tag_to_text_mark: bool,

    // 特殊处理
    pub spin: bool,

    // 表情符号处理
    pub emoji: bool,
    pub emoji_site: String,
    pub alias_emoji: HashMap<String, String>,
    pub emoji_alias: HashMap<String, String>,

    // 编辑器集成
    pub vditor_wysiwyg: bool,
    pub vditor_ir: bool,
    pub vditor_sv: bool,
    pub protyle_wysiwyg: bool,
}
