use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, Paragraph, Table};
use ratatui::Frame;

pub mod doc;

/// 定义 Ratatui 元素枚举类,用于从统一函数获取返回
/// enum(Ratatui可绘制元素, 占据高度)
///
pub struct RenderedBlock<'a> {
    pub component: BlockComponent<'a>,
    pub rendered_height: usize,
}
#[derive(Clone)]
pub enum BlockComponent<'a> {
    InValid,
    Span(Span<'a>),
    Line(Line<'a>),
    Paragraph(Paragraph<'a>),
    Table(Table<'a>),
    List(Vec<Line<'a>>),
    // List( List<'a>),
    ListItem(Vec<Line<'a>>),
    BLock(Block<'a>),
}
impl<'a> BlockComponent<'a> {
    pub fn render(&self, frame: &mut Frame<'a>, rect: Rect) {
        match self {
            BlockComponent::InValid => {}
            BlockComponent::Span(_) => {}
            BlockComponent::Line(_) => {}
            BlockComponent::Paragraph(item) => {
                frame.render_widget(item, rect);
            }
            BlockComponent::Table(_) => {}
            BlockComponent::List(_) => {}
            BlockComponent::BLock(_) => {}
            BlockComponent::ListItem(_)=>{}
        }
    }
}
