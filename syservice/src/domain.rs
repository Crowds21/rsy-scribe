use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct SyResponse{
    pub code: i32,
    pub msg: String,
    pub data: Vec<SyBlock>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyBlock {
    pub alias: String,
    /// 笔记本 id
    #[serde(rename = "box")]
    pub box_id : String,
    /// 去除了Markdown标记后的文本内容
    pub content: String,
    #[serde(rename = "created")]
    pub created_at: String,
    pub fcontent: String,
    pub hash: String,
    pub hpath: String,
    pub ial: String,
    pub id: String,
    pub length: i32,
    pub markdown: String,
    pub memo: String,
    pub name: String,
    pub parent_id: String,
    pub path: String,
    pub root_id: String,
    pub sort: i32,
    pub subtype: String,
    pub tag: String,
    #[serde(rename = "type")]
    pub block_type: String,
    pub updated: String,
}

pub enum SyBlockType{
    Document,
    // H1-H6
    Title, 
    // Order,Unordered,Task
    List,
    ListItem,
    Quote,
    SuperBlock,
    Paragraph,
    Code,
    Method,
    Table,
    // 数据库块
    DataBlock,
    QueryEmbed,
    Video,
    Audio,
    Widget,
    IFrame,
    Html,
    // 分割线
    Tb,
}
