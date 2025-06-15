use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, List, ListItem, Paragraph, Table, Wrap};
use ratatui::Frame;

pub mod doc;

/// 定义 Ratatui 元素枚举类,用于从统一函数获取返回
/// enum(Ratatui可绘制元素, 占据高度)
///
pub struct RenderedBlock<'a> {
    pub component: BlockComponent<'a>,
    pub rendered_height: u16,
}
impl<'a> RenderedBlock<'a>{
        pub fn render(&self, frame: &mut Frame, rect: Rect) {
            match &self.component {
                BlockComponent::InValid => {}
                BlockComponent::Span(_) => {}
                BlockComponent::Line(line) => {
                    frame.render_widget(line, rect);
                }
                BlockComponent::Paragraph(item) => {
                    frame.render_widget(item, rect);
                }
                BlockComponent::Table(_) => {}
                BlockComponent::List(lines) => {
                    let paragraph = Paragraph::new(lines.clone())
                        .wrap(Wrap{ trim: false}); // 启用自动换行
                    frame.render_widget(paragraph, rect);
                }
                BlockComponent::BLock(_) => {}
                BlockComponent::ListItem(_)=>{}
            }
        }
}
#[derive(Clone)]
pub enum BlockComponent<'a> {
    InValid,
    Span(Span<'a>),
    Line(Line<'a>),
    //  Ratatui paragraph 组件可以实现换行
    Paragraph(Paragraph<'a>),
    Table(Table<'a>),
    List(Vec<Line<'a>>),
    // List( List<'a>),
    ListItem(Vec<Line<'a>>),
    BLock(Block<'a>),
}

