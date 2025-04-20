use std::collections::HashMap;
use log::warn;
use once_cell::sync::Lazy;
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::HighlightSpacing;
use serde::Deserialize;
use toml::{from_str, map::Map, Value};

pub static DEFAULT_THEME_DATA: Lazy<Value> = Lazy::new(|| {
    let bytes = include_bytes!("../../../theme.toml");
    toml::from_str(std::str::from_utf8(bytes).unwrap()).expect("Failed to parse base default theme")
});

#[derive(Clone, Debug, Default)]
pub struct Theme {
    name: String,
    // UI styles are stored in a HashMap
    styles: HashMap<String, Style>,
    // tree-sitter highlight styles are stored in a Vec to optimize lookups
}
impl Theme {
    pub fn default() -> Self {
        let value = &*DEFAULT_THEME_DATA;
        let (palette, styles) = parse_theme_data(value);

        Self {
            name: "default".to_string(),
            styles: process_styles(styles, &palette),
        }
    }
}
fn parse_theme_data(value: &Value) -> (HashMap<String, String>, HashMap<String, Value>) {
    let mut palette = HashMap::new();
    let mut styles = HashMap::new();

    if let Some(table) = value.as_table() {
        for (key, value) in table {
            if key == "palette" {
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

// 处理样式并转换颜色
fn process_styles(raw_styles: HashMap<String, Value>, palette: &HashMap<String, String>) -> HashMap<String, Style> {
    let mut styles = HashMap::new();

    for (key, value) in raw_styles {
        match value {
            Value::String(color_name) => {
                if let Some(color) = resolve_color(&color_name, palette) {
                    styles.insert(key, Style::default().fg(color));
                }
            }
            Value::Table(table) => {
                let mut style = Style::default();

                if let Some(fg_value) = table.get("fg").and_then(|v| v.as_str()) {
                    if let Some(color) = resolve_color(fg_value, palette) {
                        style = style.fg(color);
                    }
                }

                if let Some(bg_value) = table.get("bg").and_then(|v| v.as_str()) {
                    if let Some(color) = resolve_color(bg_value, palette) {
                        style = style.bg(color);
                    }
                }

                if let Some(modifiers) = table.get("modifiers").and_then(|v| v.as_array()) {
                    for modifier in modifiers {
                        if let Some(m_str) = modifier.as_str() {
                            style = style.add_modifier(parse_modifier(m_str));
                        }
                    }
                }

                styles.insert(key, style);
            }
            _ => {}
        }
    }

    styles
}

// 颜色解析逻辑
fn resolve_color(color_ref: &str, palette: &HashMap<String, String>) -> Option<Color> {
    if color_ref.starts_with('#') {
        // 直接处理 HEX 颜色
        parse_hex_color(color_ref).ok()
    } else {
        // 从调色板查找颜色
        palette.get(color_ref)
            .and_then(|hex| parse_hex_color(hex).ok())
    }
}

// HEX 颜色解析
fn parse_hex_color(hex: &str) -> Result<Color, &'static str> {
    let hex = hex.trim_start_matches('#');

    let parse_component = |start, end| -> Result<u8, &'static str> {
        u8::from_str_radix(&hex[start..end], 16)
            .map_err(|_| "Invalid hex color")
    };

    match hex.len() {
        3 => {
            let r = parse_component(0, 1)? * 17;
            let g = parse_component(1, 2)? * 17;
            let b = parse_component(2, 3)? * 17;
            Ok(Color::Rgb(r, g, b))
        }
        6 => {
            let r = parse_component(0, 2)?;
            let g = parse_component(2, 4)?;
            let b = parse_component(4, 6)?;
            Ok(Color::Rgb(r, g, b))
        }
        _ => Err("Invalid hex length"),
    }
}

// 修饰符解析
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn test_theme_config() {
        let theme = Theme::default();
        assert_eq!(theme.name, "default".to_string());
    }
}