use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SyResponse{
    pub code: i32,
    pub msg: String,
    #[serde(default)]
    pub data: Vec<SyBlock>,
}

/// SQL 查询结果（灵活解析）
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SyBlock {
    #[serde(default)]
    pub alias: String,
    /// 笔记本 id
    #[serde(default, rename = "box")]
    pub box_id : String,
    /// 去除了 Markdown 标记后的文本内容
    #[serde(default)]
    pub content: String,
    #[serde(default, rename = "created")]
    pub created_at: String,
    #[serde(default)]
    pub fcontent: String,
    #[serde(default)]
    pub hash: String,
    #[serde(default)]
    pub hpath: String,
    #[serde(default)]
    pub ial: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub length: i32,
    #[serde(default)]
    pub markdown: String,
    #[serde(default)]
    pub memo: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub parent_id: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub root_id: String,
    #[serde(default)]
    pub sort: i32,
    #[serde(default)]
    pub subtype: String,
    #[serde(default)]
    pub tag: String,
    #[serde(default, rename = "type")]
    pub block_type: String,
    #[serde(default)]
    pub updated: String,
}

pub enum SyBlockType{
    Document,
    Title, 
    List,
    ListItem,
    Quote,
    SuperBlock,
    Paragraph,
    Code,
    Method,
    Table,
    DataBlock,
    QueryEmbed,
    Video,
    Audio,
    Widget,
    IFrame,
    Html,
    Tb,
}
