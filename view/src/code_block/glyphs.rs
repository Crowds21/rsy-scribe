//! 代码块外框字符，默认从 `icons.toml` 的 `[box]` 段加载（Nerd Font 兼容）。

use std::sync::OnceLock;
use toml::Value;

static GLYPHS: OnceLock<BoxGlyphs> = OnceLock::new();

/// 外框绘制字符
#[derive(Clone, Debug)]
pub struct BoxGlyphs {
    pub top_left: String,
    pub top_right: String,
    pub bottom_left: String,
    pub bottom_right: String,
    pub horizontal: String,
    pub vertical: String,
}

impl Default for BoxGlyphs {
    fn default() -> Self {
        Self {
            top_left: "╭".to_string(),
            top_right: "╮".to_string(),
            bottom_left: "╰".to_string(),
            bottom_right: "╯".to_string(),
            horizontal: "─".to_string(),
            vertical: "│".to_string(),
        }
    }
}

impl BoxGlyphs {
    fn from_toml(value: &Value) -> Self {
        let defaults = Self::default();
        let Some(table) = value.get("box").and_then(Value::as_table) else {
            return defaults;
        };
        let get = |key: &str, fallback: &str| -> String {
            table
                .get(key)
                .and_then(Value::as_str)
                .map(str::to_string)
                .unwrap_or_else(|| fallback.to_string())
        };
        Self {
            top_left: get("top_left", &defaults.top_left),
            top_right: get("top_right", &defaults.top_right),
            bottom_left: get("bottom_left", &defaults.bottom_left),
            bottom_right: get("bottom_right", &defaults.bottom_right),
            horizontal: get("horizontal", &defaults.horizontal),
            vertical: get("vertical", &defaults.vertical),
        }
    }
}

/// 获取外框字符（编译期嵌入 `icons.toml`，运行时只解析一次）
pub fn box_glyphs() -> &'static BoxGlyphs {
    GLYPHS.get_or_init(|| {
        let raw = include_str!("../../../icons.toml");
        let value: Value = toml::from_str(raw).unwrap_or(Value::Table(toml::map::Map::new()));
        BoxGlyphs::from_toml(&value)
    })
}
