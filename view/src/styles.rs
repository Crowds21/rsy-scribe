//! 行内样式 BitFlags 实现
//! 
//! 支持互斥样式组（基础样式）和叠加样式组（装饰样式）

use bitflags::bitflags;
use ratatui::style::{Modifier, Style};
use std::collections::HashMap;
use std::str::FromStr;

bitflags! {
    /// 基础样式 - 互斥组（同一时间只能有一个生效）
    /// 对应：Mark, Code, BlockRef, A(链接), Tag
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct BaseMark: u8 {
        const DEFAULT   = 0;
        const MARK      = 1 << 0;
        const CODE      = 1 << 1;
        const BLOCK_REF = 1 << 2;
        const A         = 1 << 3;
        const TAG       = 1 << 4;
    }
}

bitflags! {
    /// 装饰样式 - 叠加组（可以多个共存）
    /// 对应：Strong, Em, U(下划线), S(删除线)
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct DecorMark: u8 {
        const STRONG = 1 << 0;
        const EM     = 1 << 1;
        const U      = 1 << 2;
        const S      = 1 << 3;
    }
}

/// 基础样式类型（用于 match 匹配）
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BaseMarkKind {
    Default,
    Mark,
    Code,
    BlockRef,
    A,
    Tag,
}

impl BaseMark {
    /// 获取优先级最高的基础样式类型（按定义顺序）
    /// 
    /// 由于互斥组理论上只有一个标志被设置，此方法返回第一个匹配的标志
    /// 如果多个标志被设置（异常情况），按 MARK → CODE → BLOCK_REF → A → TAG 的优先级返回
    pub fn kind(&self) -> BaseMarkKind {
        if self.contains(BaseMark::MARK) {
            BaseMarkKind::Mark
        } else if self.contains(BaseMark::CODE) {
            BaseMarkKind::Code
        } else if self.contains(BaseMark::BLOCK_REF) {
            BaseMarkKind::BlockRef
        } else if self.contains(BaseMark::A) {
            BaseMarkKind::A
        } else if self.contains(BaseMark::TAG) {
            BaseMarkKind::Tag
        } else {
            BaseMarkKind::Default
        }
    }
}

/// 完整行内样式
#[derive(Clone, Copy, Debug, Default)]
pub struct InlineMarks {
    pub base: BaseMark,
    pub decor: DecorMark,
}

impl InlineMarks {
    pub fn new() -> Self {
        Self::default()
    }

    /// 检查是否为空（无任何样式）
    pub fn is_empty(&self) -> bool {
        self.base.is_empty() && self.decor.is_empty()
    }

    /// 合并另一个样式（装饰样式叠加，基础样式覆盖）
    pub fn merge(&mut self, other: &InlineMarks) {
        self.base = other.base;  // 基础样式覆盖
        self.decor |= other.decor;  // 装饰样式叠加
    }

    /// 获取基础样式类型（用于 match 匹配）
    pub fn base_kind(&self) -> BaseMarkKind {
        self.base.kind()
    }
}

// ============================================
// 样式解析
// ============================================

/// 解析复合样式字符串（如 "strong em code"）
/// 
/// - 互斥组：后出现的覆盖先前的
/// - 叠加组：累积
pub fn parse_marks(input: &str) -> InlineMarks {
    let mut marks = InlineMarks::new();

    for part in input.split_whitespace() {
        match part.to_lowercase().as_str() {
            // 基础样式（互斥）
            "mark" | "highlight" => marks.base = BaseMark::MARK,
            "code" => marks.base = BaseMark::CODE,
            "blockref" | "block-ref" => marks.base = BaseMark::BLOCK_REF,
            "a" | "link" | "weblink" | "hyperlink" => marks.base = BaseMark::A,
            "tag" => marks.base = BaseMark::TAG,

            // 装饰样式（叠加）
            "strong" | "bold" => marks.decor |= DecorMark::STRONG,
            "em" | "italic" => marks.decor |= DecorMark::EM,
            "u" | "underline" => marks.decor |= DecorMark::U,
            "s" | "strike" | "del" | "delete" | "strikethrough" => marks.decor |= DecorMark::S,

            _ => {}
        }
    }

    marks
}

// ============================================
// 样式到 Ratatui Style 的转换
// ============================================

/// 将 InlineMarks 转换为 Ratatui Style
/// 
/// 需要传入样式表来查找具体颜色配置
pub fn marks_to_style(marks: &InlineMarks, style_map: &HashMap<String, Style>) -> Style {
    let mut style = Style::default();

    // 应用基础样式（互斥，只应用一个）
    if marks.base.contains(BaseMark::MARK) {
        style = style_map.get("node.text.mark").copied().unwrap_or_default();
    } else if marks.base.contains(BaseMark::CODE) {
        style = style_map.get("node.text.code").copied().unwrap_or_default();
    } else if marks.base.contains(BaseMark::BLOCK_REF) {
        style = style_map.get("node.text.blockref").copied().unwrap_or_default();
    } else if marks.base.contains(BaseMark::A) {
        style = style_map.get("node.text.weblink").copied().unwrap_or_default();
    } else if marks.base.contains(BaseMark::TAG) {
        style = style_map.get("node.text.tag").copied().unwrap_or_default();
    }

    // 应用装饰样式（叠加）
    if marks.decor.contains(DecorMark::STRONG) {
        style = style.add_modifier(Modifier::BOLD);
    }
    if marks.decor.contains(DecorMark::EM) {
        style = style.add_modifier(Modifier::ITALIC);
    }
    if marks.decor.contains(DecorMark::U) {
        style = style.add_modifier(Modifier::UNDERLINED);
    }
    if marks.decor.contains(DecorMark::S) {
        style = style.add_modifier(Modifier::CROSSED_OUT);
    }

    style
}

/// 快捷方式：直接从样式名称获取 Style（用于简单场景）
pub fn style_from_name(name: &str, style_map: &HashMap<String, Style>) -> Style {
    style_map.get(name).copied().unwrap_or_default()
}

// ============================================
// 测试
// ============================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_decor() {
        let marks = parse_marks("strong");
        assert_eq!(marks.base, BaseMark::empty());
        assert!(marks.decor.contains(DecorMark::STRONG));
    }

    #[test]
    fn test_parse_multiple_decor() {
        let marks = parse_marks("strong em");
        assert!(marks.decor.contains(DecorMark::STRONG));
        assert!(marks.decor.contains(DecorMark::EM));
    }

    #[test]
    fn test_parse_base_override() {
        // 后出现的覆盖先前的
        let marks = parse_marks("code mark");
        assert_eq!(marks.base, BaseMark::MARK);
    }

    #[test]
    fn test_parse_mixed() {
        let marks = parse_marks("strong em code");
        assert_eq!(marks.base, BaseMark::CODE);
        assert!(marks.decor.contains(DecorMark::STRONG));
        assert!(marks.decor.contains(DecorMark::EM));
    }

    #[test]
    fn test_parse_case_insensitive() {
        let marks1 = parse_marks("STRONG EM");
        let marks2 = parse_marks("strong em");
        assert_eq!(marks1.decor, marks2.decor);
    }
}
