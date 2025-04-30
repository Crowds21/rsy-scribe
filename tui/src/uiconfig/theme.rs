use once_cell::sync::Lazy;
use ratatui::prelude::{Color as RaColor, Color};
use ratatui::style::Color as RatColor;
use ratatui::style::{Modifier, Style};
use std::collections::HashMap;
use std::str::FromStr;
use toml::{ map::Map, Value};

pub static DEFAULT_THEME_DATA: Lazy<Value> = Lazy::new(|| {
    let bytes = include_bytes!("../../../theme.toml");
    toml::from_str(std::str::from_utf8(bytes).unwrap()).expect("Failed to parse base default theme")
});
const PALETTE_NAME: &str = "palette";
#[derive(Clone, Debug, Default)]
pub struct Theme {
    // them name
    name: String,
    // UI styles are stored in a HashMap
    styles: HashMap<String, Style>,
    // tree-sitter highlight styles are stored in a Vec to optimize lookups
}
#[derive(Debug, Clone)]
pub struct ThemeColorItem {
    /// 背景颜色（十六进制格式，示例："#282c34"）
    background: String,
    /// 前景颜色（十六进制格式，示例："#abb2bf"）
    foreground: String,
    /// 北京亮度调整系数
    /// - 1.0 表示保持原色
    /// - 0.0 < value < 1.0 表示变暗（示例：0.75 表示亮度降低25%）
    /// - value > 1.0 表示变亮（示例：1.2 表示亮度提升20%）
    bg_scale: f32,
    /// 前景亮度调整系数
    /// - 1.0 表示保持原色
    /// - 0.0 < value < 1.0 表示变暗（示例：0.75 表示亮度降低25%）
    /// - value > 1.0 表示变亮（示例：1.2 表示亮度提升20%）
    fg_scale: f32,
    /// 样式修饰器集合（加粗/斜体/下划线等）
    /// 使用 bitflags 模式组合多个样式
    style_modifiers: Modifier,
}

impl Theme {
    pub fn default() -> Self {
        let value = &*DEFAULT_THEME_DATA;
        let (palette, styles) = load_theme_config_file(value);

        Self {
            name: "default".to_string(),
            styles: process_styles(styles, &palette),
        }
    }
}
fn load_theme_config_file(value: &Value) -> (HashMap<String, String>, HashMap<String, Value>) {
    let mut palette = HashMap::new();
    let mut styles = HashMap::new();

    if let Some(table) = value.as_table() {
        for (key, value) in table {
            if key == PALETTE_NAME {
                if let Some(palette_table) = value.as_table() {
                    for (color_name, color_value) in palette_table {
                        if let Some(color_str) = color_value.as_str() {
                            palette.insert(color_name.clone(), color_str.to_string());
                        }
                    }
                }
            } else {
                styles.insert(key.clone(), value.clone());
            }
        }
    }

    (palette, styles)
}

/// 处理样式并转换颜色
fn process_styles(
    raw_styles: HashMap<String, toml::Value>,
    palette: &HashMap<String, String>,
) -> HashMap<String, Style> {
    let mut styles = HashMap::new();

    // 颜色解析辅助函数（返回 Result 类型）
    fn resolve_color(
        color_key: &str,
        table: &Map<String, toml::Value>,
        palette: &HashMap<String, String>,
    ) -> Result<(String, f32), String> {
        // 获取原始颜色值（可选字段）
        let raw_color = table.get(color_key).and_then(|v| v.as_str()).unwrap_or(""); // 空字符串表示无颜色

        // 处理空颜色值
        if raw_color.is_empty() {
            return Ok(("".to_string(), 1.0));
        }

        // 颜色值解析逻辑
        let color = if raw_color.starts_with('#') {
            // HEX 颜色直接使用
            raw_color.to_string()
        } else {
            // 调色板查询（带错误处理）
            palette
                .get(raw_color)
                .map(|c| c.as_str())
                .ok_or_else(|| format!("调色板中未找到颜色键 '{}'", raw_color))?
                .to_string()
        };

        // 获取缩放比例（可选字段，默认1.0）
        let scale_key = format!("{}_scale", color_key);
        let scale = table
            .get(&scale_key)
            .and_then(|v| v.as_float())
            .unwrap_or(1.0) as f32;

        Ok((color, scale))
    }

    for (style_name, value) in raw_styles {
        if let toml::Value::Table(style_table) = value {
            // 背景色处理（带错误回退）
            let (bg_str, bg_scale) =
                resolve_color("bg", &style_table, palette).unwrap_or_else(|e| {
                    eprintln!("bg parse failed: {}", e);
                    ("#000000".to_string(), 1.0)
                });

            // 前景色处理（带错误回退）
            let (fg_str, fg_scale) =
                resolve_color("fg", &style_table, palette).unwrap_or_else(|e| {
                    eprintln!("fg parse failed: {}", e);
                    ("#FFFFFF".to_string(), 1.0)
                });

            let mut style = Style::default();
            if let Ok(bg_color) = change_brightness(&bg_str, bg_scale) {
                style = style.bg(bg_color);
            }
            if let Ok(fg_color) = change_brightness(&fg_str, fg_scale) {
                style = style.bg(fg_color);
            }

            // 修饰符处理（可选字段）
            let modifier = style_table
                .get("modifier")
                .and_then(|v| v.as_str())
                .map(|s| parse_modifier(s))
                .unwrap_or_default();

            style = style.add_modifier(modifier);

            styles.insert(style_name, style);
        }
    }

    styles
}

/// 修饰符解析
fn parse_modifier(modifier: &str) -> Modifier {
    match modifier.to_lowercase().as_str() {
        "bold" => Modifier::BOLD,
        "dim" => Modifier::DIM,
        "italic" => Modifier::ITALIC,
        "underlined" => Modifier::UNDERLINED,
        // "rapid_blink" => Modifier::RAPID_BLINK,
        // "slow_blink" => Modifier::SLOW_BLINK,
        "reversed" => Modifier::REVERSED,
        "hidden" => Modifier::HIDDEN,
        "crossed_out" => Modifier::CROSSED_OUT,
        _ => Modifier::empty(),
    }
}

fn change_brightness(
    color: &str,
    amount: f32,
) -> Result<Color, ratatui::style::ParseColorError> {
    // 解析基础颜色
    let base_color = csscolorparser::parse(color).map_err(|_| ratatui::style::ParseColorError)?;

    // 转换为 HSL 颜色空间
    let mut hsl = base_color.to_hsla();

    // 调整亮度分量（按比例增加）
    hsl[2] = (hsl[2] * amount).clamp(0.0, 1.0); // 确保亮度在 0-1 范围
    let hex_color =
        csscolorparser::Color::from_hsla(hsl[0], hsl[1], hsl[2], hsl[3]).to_hex_string();
    RatColor::from_str(&hex_color)
}

#[cfg(test)]
mod test {
    use crate::uiconfig::theme::Theme;

    #[test]
    fn test_theme_parse() {
        let theme = Theme::default();
        assert_eq!(theme.name, "default");
    }
}
