use ratatui::layout::Rect;
use ratatui::prelude::{Color, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use unicode_width::UnicodeWidthStr;

/// 为每一行生成对应的GUtter 样式
pub type GutterFn<'doc> = Box<dyn FnMut(usize, bool, bool, &mut String) -> Option<Style> + 'doc>;

/// 编辑器侧边栏
pub struct GutterConfig {
    /// 侧边栏展示顺序(从左到右)
    pub layout: Vec<GutterType>,
}

pub enum GutterType {
    /// Block type icons
    Icon,
    /// Show one blank space
    Spacer,
}
impl Default for GutterConfig {
    fn default() -> Self {
        Self {
            layout: vec![
                // GutterType::LineNumbers,
                GutterType::Spacer,
                GutterType::Icon,
            ],
        }
    }
}

/// TODO 渲染Gutter区域. 
///  渲染组件时,渲染对应的 icon
///  空隙
pub fn render_gutter(frame: &mut Frame, area: Rect, config: &GutterConfig, total_lines: usize) {
    let mut x_offset = area.x;
    let height = area.height as usize;
    for gutter_type in &config.layout {
        let (width, content) = match gutter_type {
            GutterType::Spacer => {
                (1, "\n".repeat(height - 1)) // 1字符宽的空白
            }
            GutterType::Icon => {
                let mut text = String::new();
                let icon_span = "󰘹 \n".width() as u16;
                for visual_line in 0..height {
                    let doc_line = visual_line + 1;
                    if doc_line <= total_lines {
                        let icon_span = "󰘹 \n";
                        // 简单实现：每行显示一个符号
                        text.push_str(icon_span); // 左对齐并填充空格
                    }
                }
                (icon_span, text)
            }
        };

        // 渲染当前Gutter部分
        let gutter_area = Rect {
            x: x_offset,
            y: area.y,
            width,
            height: area.height,
        };

        frame.render_widget(
            Paragraph::new(content.trim_end()).block(Block::default().style(Style::default())),
            gutter_area,
        );

        x_offset += width;
    }
}

impl GutterType {
    fn width(&self, total_lines: usize) -> u16 {
        match self {
            GutterType::Spacer => 1,
            GutterType::Icon => 1,
        }
    }
}
