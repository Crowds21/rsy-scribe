use unicode_width::UnicodeWidthStr;
use crate::document::{DocumentLine, InLineItem};

pub fn calculate_line_width(line: DocumentLine) -> usize {
    let mut width = 0;
    for item in line.content.iter() {
        width += item.content.width();
    }
    width
}
pub fn calculate_items_width(
    items: &Vec<InLineItem>,
) -> usize {
    let mut width = 0;
    for item in items.iter() {
        width += item.content.width();
    }
    width
}
pub fn calculate_item_width(item: &InLineItem) -> usize {
    item.content.width()
}
