# 嵌套样式处理方案

## 📋 问题描述

思源笔记的 `NodeTextMark` 节点支持组合样式，例如：

```json
{
  "Type": "NodeTextMark",
  "TextMarkType": "strong em",
  "TextMarkTextContent": "italic and bold"
}
```

当前实现无法正确处理这种嵌套样式（如同时粗体 + 斜体）。

---

## ✅ Ratatui 支持验证

**测试结果：Ratatui 完全支持多种样式嵌套**

已通过 8 个单元测试验证：

```bash
running 8 tests
test test_multiple_spans_different_styles ... ok
test test_bold_and_underline_nesting ... ok
test test_conflicting_modifiers ... ok
test test_parse_style_from_string ... ok
test test_strikethrough_nesting ... ok
test test_style_accumulation ... ok
test test_three_modifiers_nesting ... ok
test test_bold_and_italic_nesting ... ok

test result: ok. 8 passed; 0 failed
```

### 支持的样式组合

| 组合 | Ratatui 支持 | 测试状态 |
|------|------------|---------|
| **粗体 + 斜体** | ✅ | 已验证 |
| **粗体 + 下划线** | ✅ | 已验证 |
| **粗体 + 斜体 + 下划线** | ✅ | 已验证 |
| **删除线** | ✅ | 已验证 |
| **样式叠加** | ✅ | 已验证 |

---

## 🎯 解决方案

### 方案概述

利用 Ratatui 的 `Modifier` 位运算特性，通过 `add_modifier()` 方法累加多种样式。

### 核心实现

#### 1. 样式解析（从 `TextMarkType` 字符串）

```rust
/// 从样式字符串解析（如 "strong em"）
pub fn parse_style_from_mark_type(mark_type: &str) -> Style {
    let mut style = Style::default();
    
    for part in mark_type.split_whitespace() {
        match part.to_lowercase().as_str() {
            "strong" | "bold" => {
                style = style.add_modifier(Modifier::BOLD);
            }
            "em" | "italic" => {
                style = style.add_modifier(Modifier::ITALIC);
            }
            "u" | "underline" => {
                style = style.add_modifier(Modifier::UNDERLINED);
            }
            "s" | "strike" | "del" => {
                style = style.add_modifier(Modifier::CROSSED_OUT);
            }
            "mark" => {
                // 高亮通常需要背景色
                style = style.bg(Color::Yellow).fg(Color::Black);
            }
            "code" => {
                style = style.fg(Color::Green).bg(Color::DarkGray);
            }
            _ => {}
        }
    }
    
    style
}
```

#### 2. 应用到 `InLineItem`

```rust
impl InLineItem {
    /// 从 NodeTextMark 创建 InLineItem
    pub fn from_text_mark(node: &Node) -> Self {
        // 获取 TextMarkType 字符串（如 "strong em"）
        let mark_type = node.text_mark_type.as_deref().unwrap_or("");
        
        // 解析样式
        let style = parse_style_from_mark_type(mark_type);
        
        Self {
            item_type: InLineMarkType::Default,
            content: node.data.clone().unwrap_or_default(),
            styles: InlineStyles::parse(mark_type), // 同时更新 InlineStyles
            link: None,
            style: Some(mark_type.to_string()),
            theme_key: None,
            line_break: false,
        }
    }
}
```

#### 3. 渲染时应用样式

```rust
// 在 editor.rs 的 render_document 中
for item in line.content.iter() {
    // 基础样式从 InlineStyles 获取
    let mut style = Style::default();
    
    if item.styles.strong {
        style = style.add_modifier(Modifier::BOLD);
    }
    if item.styles.em {
        style = style.add_modifier(Modifier::ITALIC);
    }
    if item.styles.underline {
        style = style.add_modifier(Modifier::UNDERLINED);
    }
    if item.styles.strikethrough {
        style = style.add_modifier(Modifier::CROSSED_OUT);
    }
    
    // 应用主题样式（如果有）
    if let Some(ref theme_key) = item.theme_key {
        let theme_style = cx.theme.get(theme_key);
        if let Some(fg) = theme_style.fg {
            style = style.fg(fg);
        }
        if let Some(bg) = theme_style.bg {
            style = style.bg(bg);
        }
    }
    
    let span = Span::from(item.content.clone()).style(style);
    rendered_line.push_span(span);
}
```

---

## 📝 实现步骤

### 第一步：增强 `InlineStyles::parse`

```rust
impl InlineStyles {
    pub fn parse(style_str: &str) -> Self {
        let mut styles = InlineStyles::default();
        for part in style_str.split_whitespace() {
            match part.to_lowercase().as_str() {
                "strong" | "b" => styles.strong = true,
                "em" | "i" => styles.em = true,
                "mark" => styles.mark = true,
                "code" => styles.code = true,
                "u" => styles.underline = true,
                "s" | "del" => styles.strikethrough = true,
                _ => {}
            }
        }
        styles
    }
    
    /// 转换为 ratatui Style
    pub fn to_style(&self) -> Style {
        let mut style = Style::default();
        if self.strong {
            style = style.add_modifier(Modifier::BOLD);
        }
        if self.em {
            style = style.add_modifier(Modifier::ITALIC);
        }
        if self.mark {
            style = style.bg(Color::Yellow).fg(Color::Black);
        }
        if self.code {
            style = style.fg(Color::Green).bg(Color::DarkGray);
        }
        if self.underline {
            style = style.add_modifier(Modifier::UNDERLINED);
        }
        if self.strikethrough {
            style = style.add_modifier(Modifier::CROSSED_OUT);
        }
        style
    }
}
```

### 第二步：修改 `collect_inline_items` 处理 `NodeTextMark`

```rust
fn collect_inline_items(&mut self, node: &Node, items: &mut Vec<InLineItem>) {
    use syservice::lute::node::NodeType as NT;
    
    match node.node_type {
        // ... 其他节点类型
        
        // 新增：处理 TextMark 节点
        NT::NodeTextMark => {
            // 获取 TextMarkType 字符串（如 "strong em"）
            let mark_type = node.text_mark_type.as_deref().unwrap_or("");
            
            if let Some(ref data) = node.data {
                let item = InLineItem {
                    item_type: InLineMarkType::Default,
                    content: data.clone(),
                    styles: InlineStyles::parse(mark_type),
                    link: None,
                    style: Some(mark_type.to_string()),
                    theme_key: None,
                    line_break: false,
                };
                items.push(item);
            }
        }
        
        // ... 默认处理
    }
}
```

### 第三步：添加测试用例

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_text_mark_strong_em() {
        // 模拟 "strong em" 样式
        let styles = InlineStyles::parse("strong em");
        assert!(styles.strong);
        assert!(styles.em);
        
        let style = styles.to_style();
        assert_eq!(style.add_modifier, 
            Style::default()
                .add_modifier(Modifier::BOLD)
                .add_modifier(Modifier::ITALIC)
                .add_modifier);
    }
}
```

---

## 🔧 注意事项

### 1. 样式优先级

当同时存在 `InlineStyles` 和主题样式时：

```rust
// 1. 先应用基础样式（粗体、斜体等）
let mut style = item.styles.to_style();

// 2. 再应用主题样式（颜色等）
if let Some(theme_key) = &item.theme_key {
    let theme_style = cx.theme.get(theme_key);
    // 主题样式会覆盖或补充基础样式
}
```

### 2. 互斥样式

某些样式是互斥的，后应用的会覆盖先应用的：
- `BOLD` ↔ `DIM`（粗体 ↔ 暗淡）
- 避免同时使用

### 3. 终端兼容性

不是所有终端都支持所有样式组合：
- 某些终端可能不支持"粗体 + 斜体"同时显示
- 建议在实际终端中测试效果

---

## 📊 测试覆盖率

| 场景 | 测试用例 | 状态 |
|------|---------|------|
| 单一粗体 | `test_bold_only` | ⏳ 待添加 |
| 单一斜体 | `test_italic_only` | ⏳ 待添加 |
| 粗体 + 斜体 | `test_bold_and_italic_nesting` | ✅ 已验证 |
| 粗体 + 下划线 | `test_bold_and_underline_nesting` | ✅ 已验证 |
| 三样式组合 | `test_three_modifiers_nesting` | ✅ 已验证 |
| 字符串解析 | `test_parse_style_from_string` | ✅ 已验证 |
| 多 Span 样式 | `test_multiple_spans_different_styles` | ✅ 已验证 |

---

## 🚀 后续优化

1. **性能优化** - 缓存解析后的 Style，避免重复解析
2. **主题集成** - 支持从 theme.toml 定义组合样式
3. **更多样式** - 支持 Blink（闪烁）、Reverse（反色）等
4. **可视化测试** - 创建手动测试工具，在真实终端验证效果

---

## 📚 参考资料

- [Ratatui Style Documentation](https://docs.rs/ratatui/latest/ratatui/style/index.html)
- [Modifier 源码](https://docs.rs/ratatui/latest/ratatui/style/struct.Modifier.html)
- 测试文件：`view/tests/style_nesting_test.rs`

---

**结论：Ratatui 完全支持嵌套样式，可以直接实现。** ✅
