use unicode_width::UnicodeWidthStr;

pub(crate) fn calculate_line_width(line: &crate::model::document_model::DocumentLine) -> usize {
    let mut width = 0;
    for item in line.content.iter() {
        width += item.content.width();
    }
    width
}

pub(crate) fn calculate_items_width(
    items: &Vec<crate::model::document_model::InLineItem>,
) -> usize {
    let mut width = 0;
    for item in items.iter() {
        width += item.content.width();
    }
    width
}

// 父模块可见
pub(crate) fn calculate_item_width(item: &crate::model::document_model::InLineItem) -> usize {
    item.content.width()
}
