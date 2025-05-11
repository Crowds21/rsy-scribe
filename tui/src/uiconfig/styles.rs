use lazy_static::lazy_static;
use ratatui::style::{Modifier, Style};

/// TODO 这里还需要想一下,样式表什么时候生成,是否需要做成全局静态变量
lazy_static! {
    static ref BOLD_STYLE: Style= Style::default().add_modifier(Modifier::BOLD);
    static ref ITALIC_STYLE: Style = Style::default().add_modifier(Modifier::ITALIC);
    static ref UNDERLINE_STYLE: Style = Style::default().add_modifier(Modifier::UNDERLINED);
    static ref STRIKETHROUGH_STYLE: Style = Style::default().add_modifier(Modifier::CROSSED_OUT);
    // static ref CODE_STYLE: Style = Style::default()
    //     .bg(Color::DarkGray)
    //     .fg(Color::LightYellow);
    // static ref LINK_STYLE: Style = Style::default()
    //     .fg(Color::Blue)
    //     .add_modifier(Modifier::UNDERLINED);
}