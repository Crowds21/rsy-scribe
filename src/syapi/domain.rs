use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct SyResponse{
    pub code: i32,
    pub msg: String,
    pub data: Vec<SyBlock>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SyBlock {
    alias: String,
    /// 笔记本 id
    #[serde(rename = "box")]
    box_id : String,
    content: String,
    #[serde(rename = "created")]
    created_at: String,
    fcontent: String,
    hash: String,
    hpath: String,
    ial: String,
    id: String,
    length: i32,
    markdown: String,
    memo: String,
    name: String,
    parent_id: String,
    path: String,
    root_id: String,
    sort: i32,
    subtype: String,
    tag: String,
    type_: String,
    updated: String,
}
