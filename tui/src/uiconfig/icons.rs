use std::fs;
use std::path::Path;
use once_cell::sync::Lazy;
use toml::Value;

pub static DEFAULT_ICON_DATA: Lazy<Value> = Lazy::new(|| {
    let bytes = include_bytes!("../../../icons.toml");
    toml::from_str(std::str::from_utf8(bytes).unwrap()).expect("Failed to parse default icons")
});
#[derive(Debug)]
pub struct Icons {
    // 文档结构
    pub document: String,
    pub notes: String,
    pub outline: String,

    // 标题级别
    pub head1: String,
    pub head2: String,
    pub head3: String,
    pub head4: String,
    pub head5: String,
    pub head6: String,

    // 列表类型
    pub unordered_list: String,
    pub ordered_list: String,
    pub task_list: String,

    // 其他元素
    pub paragraph: String,
    pub quote: String,
    pub code_block: String,
    pub table: String,
}
impl Icons {
    /// 从外部文件加载 (优先于默认配置)
    pub fn from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let value = toml::from_str(&content)?;
        Ok(Self::from_toml_value(&value)?)
    }

    /// 从 TOML Value 解析
    fn from_toml_value(value: &Value) -> anyhow::Result<Self> {
        let icon_table = value.get("icon")
            .and_then(Value::as_table)
            .ok_or_else(|| anyhow::anyhow!("Missing [icon] section in config"))?;

        let get_icon = |key: &str| -> String {
            icon_table.get(key)
                .and_then(Value::as_str)
                .map(ToString::to_string)
                .unwrap_or_else(|| {
                    log::warn!("Missing icon key: {}", key);
                    "?".into()
                })
        };

        Ok(Self {
            document: get_icon("document"),
            notes: get_icon("notes"),
            outline: get_icon("outline"),
            head1: get_icon("head1"),
            head2: get_icon("head2"),
            head3: get_icon("head3"),
            head4: get_icon("head4"),
            head5: get_icon("head5"),
            head6: get_icon("head6"),
            unordered_list: get_icon("unordered_list"),
            ordered_list: get_icon("ordered_list"),
            task_list: get_icon("task_list"),
            paragraph: get_icon("paragraph"),
            quote: get_icon("quote"),
            code_block: get_icon("code_block"),
            table: get_icon("table"),
        })
    }
}
impl Default for Icons {
    fn default() -> Self {
        Self::from_toml_value(&DEFAULT_ICON_DATA)
            .expect("Failed to load default icons")
    }
}