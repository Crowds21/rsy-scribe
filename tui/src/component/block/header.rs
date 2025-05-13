use ratatui::layout::Rect;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::str::FromStr;
use syservice::lute::node::{Node, NodeType};

fn render_header(child: &mut Node) -> Paragraph<'static> {
    let text = child.children[0].data.clone().unwrap();
    let level = child.heading_level.unwrap();
    let content = Span::styled(text.clone(), Style::default().add_modifier(Modifier::BOLD));
    match level {
        1 => {
            // H1: 双实线包围 + 上下双横线
            let line = "═".repeat(text.len() + 2);
            Paragraph::new(vec![
                Line::from(line.clone()),
                Line::from(vec![content]),
                Line::from(line),
            ])
            .block(Block::default().borders(Borders::LEFT | Borders::RIGHT))
        }
        2..=3 => {
            // H2-H3: 单实线下划线
            Paragraph::new(vec![
                Line::from(content),
                Line::from("─".repeat(text.len())), // 实线
            ])
        }
        _ => {
            // H4-H6: 虚线下划线
            Paragraph::new(vec![
                Line::from(content),
                Line::from("﹏".repeat(text.len())), // 中文虚线符号
            ])
        }
    }
}

fn render_document(node: &mut Node, frame: &mut Frame, content_area: Rect) {
    node.node_type = NodeType::from_str(&node.type_str).unwrap();
    if let NodeType::NodeDocument = node.node_type {
        let document_header = render_header(node);
        frame.render_widget(document_header, content_area);
    }
    render_children(node, frame);
}
fn render_node_text(data: String) -> Span<'static> {
    Span::from(data)
}

fn render_node_strong(data: String) -> Span<'static> {
    let style = Style::default().add_modifier(Modifier::BOLD);
    Span::styled(data, style)
}

fn render_children(node: &mut Node, frame: &mut Frame) {
    node.node_type = NodeType::from_str(&node.type_str).unwrap();
    for child in &mut node.children {
        match child.node_type {
            NodeType::Default => {}
            NodeType::NodeDocument => {}
            NodeType::NodeParagraph => {}
            NodeType::NodeHeading => {}
            NodeType::NodeHeadingC8hMarker => {}
            NodeType::NodeThematicBreak => {}
            NodeType::NodeBlockquote => {}
            NodeType::NodeBlockquoteMarker => {}
            NodeType::NodeList => {}
            NodeType::NodeListItem => {}
            NodeType::NodeHtmlBlock => {}
            NodeType::NodeInlineHtml => {}
            NodeType::NodeCodeBlock => {}
            NodeType::NodeCodeBlockFenceOpenMarker => {}
            NodeType::NodeCodeBlockFenceCloseMarker => {}
            NodeType::NodeCodeBlockFenceInfoMarker => {}
            NodeType::NodeCodeBlockCode => {}
            NodeType::NodeText => {
                render_node_text(child.data.clone().unwrap());
            }
            NodeType::NodeEmphasis => {}
            NodeType::NodeEmA6kOpenMarker => {}
            NodeType::NodeEmA6kCloseMarker => {}
            NodeType::NodeEmU8eOpenMarker => {}
            NodeType::NodeEmU8eCloseMarker => {}
            NodeType::NodeStrong => {}
            NodeType::NodeStrongA6kOpenMarker => {}
            NodeType::NodeStrongA6kCloseMarker => {}
            NodeType::NodeStrongU8eOpenMarker => {}
            NodeType::NodeStrongU8eCloseMarker => {}
            NodeType::NodeCodeSpan => {}
            NodeType::NodeCodeSpanOpenMarker => {}
            NodeType::NodeCodeSpanContent => {}
            NodeType::NodeCodeSpanCloseMarker => {}
            NodeType::NodeHardBreak => {}
            NodeType::NodeSoftBreak => {}
            NodeType::NodeLink => {}
            NodeType::NodeImage => {}
            NodeType::NodeBang => {}
            NodeType::NodeOpenBracket => {}
            NodeType::NodeCloseBracket => {}
            NodeType::NodeOpenParen => {}
            NodeType::NodeCloseParen => {}
            NodeType::NodeLinkText => {}
            NodeType::NodeLinkDest => {}
            NodeType::NodeLinkTitle => {}
            NodeType::NodeLinkSpace => {}
            NodeType::NodeHtmlEntity => {}
            NodeType::NodeLinkRefDefBlock => {}
            NodeType::NodeLinkRefDef => {}
            NodeType::NodeLess => {}
            NodeType::NodeGreater => {}
            NodeType::NodeTaskListItemMarker => {}
            NodeType::NodeStrikethrough => {}
            NodeType::NodeStrikethrough1OpenMarker => {}
            NodeType::NodeStrikethrough1CloseMarker => {}
            NodeType::NodeStrikethrough2OpenMarker => {}
            NodeType::NodeStrikethrough2CloseMarker => {}
            NodeType::NodeTable => {}
            NodeType::NodeTableHead => {}
            NodeType::NodeTableRow => {}
            NodeType::NodeTableCell => {}
            NodeType::NodeEmoji => {}
            NodeType::NodeEmojiUnicode => {}
            NodeType::NodeEmojiImg => {}
            NodeType::NodeEmojiAlias => {}
            NodeType::NodeMathBlock => {}
            NodeType::NodeMathBlockOpenMarker => {}
            NodeType::NodeMathBlockContent => {}
            NodeType::NodeMathBlockCloseMarker => {}
            NodeType::NodeInlineMath => {}
            NodeType::NodeInlineMathOpenMarker => {}
            NodeType::NodeInlineMathContent => {}
            NodeType::NodeInlineMathCloseMarker => {}
            NodeType::NodeBackslash => {}
            NodeType::NodeBackslashContent => {}
            NodeType::NodeVditorCaret => {}
            NodeType::NodeFootnotesDefBlock => {}
            NodeType::NodeFootnotesDef => {}
            NodeType::NodeFootnotesRef => {}
            NodeType::NodeToc => {}
            NodeType::NodeHeadingId => {}
            NodeType::NodeYamlFrontMatter => {}
            NodeType::NodeYamlFrontMatterOpenMarker => {}
            NodeType::NodeYamlFrontMatterContent => {}
            NodeType::NodeYamlFrontMatterCloseMarker => {}
            NodeType::NodeBlockRef => {}
            NodeType::NodeBlockRefId => {}
            NodeType::NodeBlockRefSpace => {}
            NodeType::NodeBlockRefText => {}
            NodeType::NodeBlockRefDynamicText => {}
            NodeType::NodeMark => {}
            NodeType::NodeMark1OpenMarker => {}
            NodeType::NodeMark1CloseMarker => {}
            NodeType::NodeMark2OpenMarker => {}
            NodeType::NodeMark2CloseMarker => {}
            NodeType::NodeKramdownBlockIal => {}
            NodeType::NodeKramdownSpanIal => {}
            NodeType::NodeTag => {}
            NodeType::NodeTagOpenMarker => {}
            NodeType::NodeTagCloseMarker => {}
            NodeType::NodeBlockQueryEmbed => {}
            NodeType::NodeOpenBrace => {}
            NodeType::NodeCloseBrace => {}
            NodeType::NodeBlockQueryEmbedScript => {}
            NodeType::NodeSuperBlock => {}
            NodeType::NodeSuperBlockOpenMarker => {}
            NodeType::NodeSuperBlockLayoutMarker => {}
            NodeType::NodeSuperBlockCloseMarker => {}
            NodeType::NodeSup => {}
            NodeType::NodeSupOpenMarker => {}
            NodeType::NodeSupCloseMarker => {}
            NodeType::NodeSub => {}
            NodeType::NodeSubOpenMarker => {}
            NodeType::NodeSubCloseMarker => {}
            NodeType::NodeGitConflict => {}
            NodeType::NodeGitConflictOpenMarker => {}
            NodeType::NodeGitConflictContent => {}
            NodeType::NodeGitConflictCloseMarker => {}
            NodeType::NodeIFrame => {}
            NodeType::NodeAudio => {}
            NodeType::NodeVideo => {}
            NodeType::NodeKbd => {}
            NodeType::NodeKbdOpenMarker => {}
            NodeType::NodeKbdCloseMarker => {}
            NodeType::NodeUnderline => {}
            NodeType::NodeUnderlineOpenMarker => {}
            NodeType::NodeUnderlineCloseMarker => {}
            NodeType::NodeBr => {}
            NodeType::NodeTextMark => {}
            NodeType::NodeWidget => {}
            NodeType::NodeFileAnnotationRef => {}
            NodeType::NodeFileAnnotationRefId => {}
            NodeType::NodeFileAnnotationRefSpace => {}
            NodeType::NodeFileAnnotationRefText => {}
            NodeType::NodeAttributeView => {}
            NodeType::NodeCustomBlock => {}
            NodeType::NodeHtmlTag => {}
            NodeType::NodeHtmlTagOpen => {}
            NodeType::NodeHtmlTagClose => {}
            NodeType::NodeMaxVal => {}
        }
    }
}
