use unicode_width::UnicodeWidthStr;
use crate::document::{DocumentLine, InLineItem};

pub fn calculate_line_width(line: &DocumentLine) ->  u16{
    let mut width = 0;
    for item in line.content.iter() {
        width += item.content.width();
    }
    width as u16
}
pub fn calculate_items_width(
    items: &Vec<InLineItem>,
) -> u16{
    let mut width = 0;
    for item in items.iter() {
        width += item.content.width();
    }
    width as u16
}
pub fn calculate_item_width(item: &InLineItem) -> u16{
    item.content.width() as u16
}
