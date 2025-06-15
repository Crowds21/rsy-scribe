use crate::component::block::{BlockComponent, RenderedBlock};
use crate::compositor::CompositorContext;
use ratatui::layout::Alignment;
use ratatui::{
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use syservice::lute::node::{Node, NodeType};
use unicode_width::UnicodeWidthStr;

/// 渲染文档的统一入口
pub fn create_document_blocks<'a>(
    node: &'a mut Node,
    cx: &'a mut CompositorContext,
) -> Vec<RenderedBlock<'a>> {
    let mut blocks = Vec::new();
    node.set_node_type_for_tree();
    // 提前处理 node_type 赋值
    create_tui_element(node, cx, &mut blocks);
    blocks
}

/// 组装传入的 Vec<RenderedBlock>.每一个 RenderedBlock 都是一个"可渲染的组件"
/// 各个组件自行判断是否需要递归调用以生成"可渲染组件"
///
/// TODO 手动控制渲染的开始和结束位置, 需要在加载文档的时候,
///     首先遍历一次,计算整个文档所占用的长度. 然后后续渲染直接渲染对应的部分
pub fn create_tui_element<'a>(
    node: &'a Node,
    cx: &'a CompositorContext,
    vec: &mut Vec<RenderedBlock<'a>>,
) {
    let element: Option<RenderedBlock> = match node.node_type {
        NodeType::Default => None,
        // 返回标题组件
        NodeType::NodeDocument => create_title(node, cx),
        NodeType::NodeHeading => create_heading(node, cx),
        NodeType::NodeParagraph => create_node_paragraph(node, cx),
        NodeType::NodeText => create_node_text(node.data.clone().unwrap()),
        NodeType::NodeTextMark => create_node_text_mark(node, cx),
        NodeType::NodeList => create_list(node, cx),
        NodeType::NodeListItem => create_list_item(node, cx),
        _ => None,
    };
    if let Some(block) = element {
        vec.push(block);
    }
    // 对于"文档块" 需要遍历子块来组成一个 "可渲染组件列表"
    // 而对于其他的"非文档块",需要自己按需判断
    if let NodeType::NodeDocument = node.node_type {
        for child in node.children.iter() {
            create_tui_element(child, cx, vec);
        }
    }
}

fn create_title<'a>(root: &'a Node, cx: &CompositorContext) -> Option<RenderedBlock<'a>> {
    let title = root.properties.as_ref().and_then(|map| {
        // 获取title并构建Paragraph
        map.get("title").map(|title| {
            let style = cx.theme.get("node.heading.title");
            let content = Span::styled(title.clone(), style);
            let line = "═".repeat(title.width());
            Paragraph::new(vec![
                Line::from(line.clone()),
                Line::from(vec![content]),
                Line::from(line),
            ])
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
        })
    });
    match title {
        None => None,
        Some(title) => Some(RenderedBlock {
            component: BlockComponent::Paragraph(title),
            rendered_height: 3,
        }),
    }
}
fn create_node_paragraph<'a>(
    node: &'a Node,
    cx: &'a CompositorContext,
) -> Option<RenderedBlock<'a>> {
    let mut item_vec: Vec<RenderedBlock> = Vec::new();
    for child in node.children.iter() {
        create_tui_element(child, cx, &mut item_vec);
    }

    // 收集所有Span组件（获取所有权）
    let spans: Vec<Span> = item_vec
        .into_iter() // 使用into_iter获取所有权
        .filter_map(|block| match block.component {
            // 直接匹配值而非引用
            BlockComponent::Span(span) => Some(span), // 直接返回span的所有权
            _ => None,
        })
        .collect();

    if spans.is_empty() {
        return None;
    }

    let line = Line::from(spans).alignment(Alignment::Left);
    Some(RenderedBlock {
        component: BlockComponent::Line(line),
        rendered_height: 1,
    })
}
fn create_node_text(data: String) -> Option<RenderedBlock<'static>> {
    Some(RenderedBlock {
        component: BlockComponent::Span(Span::from(data)),
        rendered_height: 1,
    })
}
fn create_node_text_mark<'a>(
    node: &'a Node,
    cx: &'a CompositorContext,
) -> Option<RenderedBlock<'a>> {
    if node.node_type != NodeType::NodeTextMark {
        return None;
    };
    if let Some(content) = &node.text_mark_text_content {
        let span = ratatui::text::Span::from(content);
        return Some(RenderedBlock {
            component: BlockComponent::Span(span),
            rendered_height: 1,
        });
    }
    None
}
fn create_text_strong(data: String, cx: CompositorContext) -> Span<'static> {
    let style = cx.theme.get("node.text.strong");
    Span::styled(data, style)
}

/// 递归处理   NodeType::NodeHeading
fn create_heading<'a>(node: &'a Node, cx: &'a CompositorContext) -> Option<RenderedBlock<'a>> {
    if node.node_type != NodeType::NodeHeading {
        return None;
    };
    let mut vec: Vec<RenderedBlock> = Vec::new();
    for child in &node.children {
        create_tui_element(child, cx, &mut vec);
    }

    /// 基于 H1 -H6 生成对应的 tui组件
    fn assemble_header<'a>(
        level: i32,
        vec: Vec<RenderedBlock<'a>>,
        cx: &'a CompositorContext,
    ) -> RenderedBlock<'a> {
        let (spans, total_width) = vec
            .into_iter()
            .filter_map(|item| match item.component {
                BlockComponent::Span(span) => {
                    let text: &str = &span.content.clone();
                    Some((span, text.width().clone()))
                }
                _ => None,
            })
            .fold(
                (Vec::new(), 0),
                |(mut spans, total_width), (span, width)| {
                    spans.push(span);
                    (spans, total_width + width)
                },
            );

        let (paragraph, length) = match level {
            1 => {
                let style = cx.theme.get("node.heading.h1");
                let line = "═".repeat(total_width);
                let paragraph = Paragraph::new(vec![
                    Line::from(line.clone()),
                    Line::from(spans),
                    Line::from(line),
                ])
                .style(style)
                .block(Block::default().borders(Borders::LEFT | Borders::RIGHT));
                (paragraph, 3)
            }
            2..=3 => {
                // H2-H3: 单实线下划线
                let paragraph = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(spans),
                    Line::from("─".repeat(total_width)), // 实线
                ]);
                (paragraph, 3)
            }
            _ => {
                // H4-H6: 虚线下划线
                let paragraph = Paragraph::new(vec![
                    Line::from(""),
                    Line::from(spans),
                    Line::from("﹏".repeat(total_width)), // 中文虚线符号
                ]);
                (paragraph, 3)
            }
        };
        RenderedBlock {
            component: BlockComponent::Paragraph(paragraph),
            rendered_height: length,
        }
    }
    Some(assemble_header(node.heading_level.unwrap(), vec, cx))
}

/// 创建 List 组件
///
fn create_list<'a>(node: &'a Node, cx: &'a CompositorContext) -> Option<RenderedBlock<'a>> {
    if !matches!(node.node_type, NodeType::NodeList) {
        return None;
    }

    let mut list_items = Vec::new();
    for child in node.children.iter() {
        let mut child_blocks = Vec::new();
        create_tui_element(child, cx, &mut child_blocks);

        for block in child_blocks {
            if let BlockComponent::ListItem(items) = block.component {
                // TOOD  Line 的部分看看可不可以直接删除掉.
                // item 是 Vec<Line>. list_items 也是 Vec<Line>
                // 把 items 中的元素放入
                list_items.extend(items.into_iter());
            }
        }
    }
    let len = list_items.len();
    (!list_items.is_empty()).then_some(RenderedBlock {
        component: BlockComponent::List(list_items),
        rendered_height: len as u16,
    })
}

///
fn create_list_item<'a>(node: &'a Node, cx: &'a CompositorContext) -> Option<RenderedBlock<'a>> {
    let mut child_blocks = Vec::new();

    for child in node.children.iter() {
        create_tui_element(child, cx, &mut child_blocks);
    }

    // Get list item data
    let bullet_char = node
        .list_data
        .as_ref()
        .and_then(|data| data.bullet_char)
        .map(|c| c as char);

    // Process Line components and nested Lists
    for block in child_blocks.iter_mut() {
        match &mut block.component {
            BlockComponent::Line(line) => {
                if let Some(bullet) = bullet_char {
                    line.spans.insert(0, Span::raw(format!("{} ", bullet)));
                }
            }
            BlockComponent::List(list) => {
                // Indent each item in the nested list
                for item in list.iter_mut() {
                    item.spans.insert(0, Span::raw("  "));
                }
            }
            _ => {}
        }
    }

    // Convert child_blocks(List or Line) into a ListItem
    // 基于child_blocks 生成一个 Vec<Line>
    let mut lines: Vec<Line> = Vec::new();
    for block in child_blocks {
        match block.component {
            BlockComponent::Line(line) => {
                lines.push(line);
            }
            BlockComponent::List(nested_lines) => {
                // Flatten nested list items into our lines
                lines.extend(nested_lines);
            }
            _ => {}
        }
    }

    let len = lines.len();
    (!lines.is_empty()).then_some(RenderedBlock {
        component: BlockComponent::List(lines),
        rendered_height: len as u16,
    })
}
