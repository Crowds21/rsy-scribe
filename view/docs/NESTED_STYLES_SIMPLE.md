# 嵌套样式实现 - 简化版

## 📝 修改内容

### 1. `view/src/document.rs` - `create_node_text_mark()`

**修改点：** 直接将 `TextMarkType` 字符串（如 "strong em"）作为样式键保存

```rust
fn create_node_text_mark(&mut self, node: &Node) -> (InLineItem, u16) {
    let mark_type = node.text_mark_type.clone().unwrap_or_default();
    
    // 直接使用 mark_type 作为样式键（支持 "strong em" 组合）
    let style_key = if mark_type.is_empty() {
        None
    } else {
        Some(mark_type.clone())
    };
    
    let item = InLineItem {
        item_type: InLineMarkType::from_str(&mark_type).unwrap_or_default(),
        content: ...,
        style: style_key,  // ← 关键：直接保存 "strong em"
        ...
    };
    
    (item, width)
}
```

---

### 2. `tui/src/component/editor.rs` - 渲染逻辑（L112）

**修改点：** 使用样式键直接从主题中查找样式

```rust
for item in line.content.iter() {
    // 根据样式键获取主题样式（支持 "strong em" 组合）
    let style = match &item.style {
        None => Style::default(),
        Some(style_key) => cx.theme.get(style_key),  // ← 关键：直接查找
    };
    let span = Span::from(item.content.clone()).style(style);
    rendered_line.push_span(span);
}
```

---

### 3. `theme.toml` - OneDark 配色

**关键配置：** 定义组合样式

```toml
# 单一样式
"strong" = { fg = "fg", modifier = "bold" }
"em" = { fg = "fg", modifier = "italic" }
"u" = { fg = "fg", modifier = "underlined" }
"s" = { fg = "fg", modifier = "crossed_out" }

# 组合样式
"strong em" = { fg = "fg", modifier = "bold italic" }
"strong u" = { fg = "fg", modifier = "bold underlined" }
"em s" = { fg = "fg", modifier = "italic crossed_out" }
```

---

## 🎯 工作流程

```
思源笔记 NodeTextMark
  TextMarkType: "strong em"
  TextMarkTextContent: "粗体 + 斜体文本"
       ↓
document.rs::create_node_text_mark()
  item.style = Some("strong em")  ← 直接保存字符串
       ↓
editor.rs::render_document()
  cx.theme.get("strong em")  ← 直接查找
       ↓
theme.toml
  "strong em" = { modifier = "bold italic" }
       ↓
渲染效果
  **粗体 + 斜体文本**
```

---

## ✅ 支持的组合

| TextMarkType | theme.toml 键 | 效果 |
|-------------|--------------|------|
| `strong` | `strong` | **粗体** |
| `em` | `em` | *斜体* |
| `strong em` | `strong em` | ***粗体 + 斜体*** |
| `strong u` | `strong u` | **粗体 + 下划线** |
| `em s` | `em s` | *斜体 + 删除线* |
| `strong em u` | `strong em u` | ***粗体 + 斜体 + 下划线*** |

---

## 🔧 编译测试

```bash
cd /Users/crowds/RustroverProjects/scribe
cargo check
# ✅ Finished dev profile
```

---

## 📊 关键优势

1. **简单直接** - 不需要中间转换，`TextMarkType` 直接作为样式键
2. **保持兼容** - `InLineItem` 结构不变，只改变 `style` 字段的内容
3. **易于扩展** - 添加新组合只需在 theme.toml 中定义
4. **清晰明了** - 一眼就能看懂样式如何传递和应用

---

## 🚀 下一步

1. 在 theme.toml 中添加更多组合样式定义
2. 测试实际渲染效果
3. 根据需要调整配色

---

**修改时间：** 2026-03-29  
**状态：** ✅ 编译通过
