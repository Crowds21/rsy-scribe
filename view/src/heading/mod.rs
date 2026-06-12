//! 标题块渲染：分级装饰线与 theme 样式键
//!
//! ```text
//! NodeHeading → DocumentLine[] → theme `node.heading.hN`
//! ```

mod layout;

pub use layout::{apply_heading_styles, decorate_heading};

use syservice::lute::node::Node;

/// 标题正文对应的 theme 键
pub fn content_style_key(level: u8) -> &'static str {
    match level {
        1 => "node.heading.h1",
        2 => "node.heading.h2",
        3 => "node.heading.h3",
        4 => "node.heading.h4",
        5 => "node.heading.h5",
        6 => "node.heading.h6",
        _ => "node.heading",
    }
}

/// 标题装饰线对应的 theme 键
pub fn decor_style_key(level: u8) -> &'static str {
    match level {
        1 => "node.heading.h1.decor",
        2 => "node.heading.h2.decor",
        3 => "node.heading.h3.decor",
        4 => "node.heading.h4.decor",
        5 => "node.heading.h5.decor",
        6 => "node.heading.h6.decor",
        _ => "node.heading.decor",
    }
}

pub fn heading_level_from_node(node: &Node) -> u8 {
    node.heading_level.unwrap_or(1).clamp(1, 6) as u8
}
