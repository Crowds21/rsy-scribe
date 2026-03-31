use unicode_width::UnicodeWidthStr;
use crate::document::{DocumentLine, InLineItem};

/// 计算行的显示宽度（使用 display_content）
pub fn calculate_line_width(line: &DocumentLine) -> u16 {
    let mut width = 0;
    for item in line.content.iter() {
        width += item.display_content.width();
    }
    width as u16
}

/// 计算多个 InLineItem 的总显示宽度
pub fn calculate_items_width(items: &Vec<InLineItem>) -> u16 {
    let mut width = 0;
    for item in items.iter() {
        width += item.display_content.width();
    }
    width as u16
}

/// 计算单个 InLineItem 的显示宽度
pub fn calculate_item_width(item: &InLineItem) -> u16 {
    item.display_content.width() as u16
}
