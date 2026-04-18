# BitFlags 样式方案实现说明

## 实现日期
2026-03-31

## 改动概述

将 `InLineMarkType` 从单一枚举改为 BitFlags 组合方案，支持复合样式解析（如 `"strong em code"`）。

---

## 核心改动

### 1. 新增模块：`view/src/styles.rs`

定义了 BitFlags 结构、辅助枚举和解析逻辑：

```rust
// 基础样式 - 互斥组
bitflags! {
    pub struct BaseMark: u8 {
        const DEFAULT   = 0;
        const MARK      = 1 << 0;
        const CODE      = 1 << 1;
        const BLOCK_REF = 1 << 2;
        const A         = 1 << 3;
        const TAG       = 1 << 4;
    }
}

// 装饰样式 - 叠加组
bitflags! {
    pub struct DecorMark: u8 {
        const STRONG = 1 << 0;
        const EM     = 1 << 1;
        const U      = 1 << 2;
        const S      = 1 << 3;
    }
}

/// 基础样式类型（用于 match 匹配）
pub enum BaseMarkKind {
    Default, Mark, Code, BlockRef, A, Tag,
}

impl BaseMark {
    /// 获取优先级最高的基础样式类型
    pub fn kind(&self) -> BaseMarkKind { /* ... */ }
}

pub struct InlineMarks {
    pub base: BaseMark,
    pub decor: DecorMark,
}
```

**解析规则：**
- 互斥组：后出现的覆盖先前的（如 `"code mark"` → `MARK`）
- 叠加组：累积（如 `"strong em"` → `STRONG | EM`）

---

### 2. 修改 `view/src/document.rs`

#### 2.1 移除旧枚举

删除了 `InLineMarkType` 枚举。

#### 2.2 重构 `InLineItem` 结构体

```rust
// 旧结构
pub struct InLineItem {
    pub item_type: InLineMarkType,  // 单一类型
    pub content: String,
    pub link: Option<String>,
    pub style: Option<String>,
    line_break: bool,
}

// 新结构（第一次重构）
pub struct InLineItem {
    pub marks: InlineMarks,          // 复合样式
    pub content: String,             // 原始内容
    pub display_content: String,     // 显示内容（已转义）
    pub link: Option<String>,
    pub style_name: Option<String>,  // 对应 theme.toml 样式名
    line_break: bool,
}

// 新结构（第二次重构 - 样式列表）
pub struct InLineItem {
    pub marks: InlineMarks,          // 复合样式
    pub content: String,             // 原始内容
    pub display_content: String,     // 显示内容（已转义）
    pub link: Option<String>,
    pub styles: Vec<String>,         // 样式名称列表（基础 + 装饰）
    line_break: bool,
}
```

**字段变更说明：**
- `item_type` → `marks`: 支持复合样式
- `content` → 保留原始内容
- 新增 `display_content`: 存储 HTML 转义后的显示内容
- `style` → `style_name` → `styles`: 从单个样式变为样式列表

#### 2.3 更新 `create_node_text_mark` 函数

```rust
// 旧逻辑
let mark_type = node.text_mark_type.clone().unwrap_or_default();
let enum_mark_type = InLineMarkType::from_str(&mark_type).unwrap_or_default();

// 新逻辑（样式列表版本）
let mark_type_str = node.text_mark_type.clone().unwrap_or_default();
let marks = parse_marks(&mark_type_str);  // 解析复合样式

// 构建样式列表：基础样式 + 装饰样式
let mut styles: Vec<String> = Vec::new();

// 1. 添加基础样式（互斥，只选一个）
let link = match marks.base_kind() {
    BaseMarkKind::Mark => {
        styles.push("node.text.mark".to_string());
        None
    }
    BaseMarkKind::Code => {
        styles.push("node.text.code".to_string());
        None
    }
    BaseMarkKind::BlockRef => {
        styles.push("node.text.blockref".to_string());
        let block_ref = node.text_mark_block_ref_id.clone().unwrap_or_default();
        Some(block_ref)
    }
    BaseMarkKind::A => {
        styles.push("node.text.weblink".to_string());
        let web_link = node.text_mark_a_href.clone().unwrap_or_default();
        Some(web_link)
    }
    BaseMarkKind::Tag => {
        styles.push("node.text.tag".to_string());
        None
    }
    BaseMarkKind::Default => None,
};

// 2. 添加装饰样式（可叠加）
if marks.decor.contains(DecorMark::STRONG) {
    styles.push("node.text.strong".to_string());
}
if marks.decor.contains(DecorMark::EM) {
    styles.push("node.text.italic".to_string());
}
if marks.decor.contains(DecorMark::U) {
    styles.push("node.text.underline".to_string());
}
if marks.decor.contains(DecorMark::S) {
    styles.push("node.text.strikethrough".to_string());
}
```

**样式示例：**
- `"strong em code"` → `styles = ["node.text.code", "node.text.strong", "node.text.italic"]`
- `"mark strong u"` → `styles = ["node.text.mark", "node.text.strong", "node.text.underline"]`
- `"em"` → `styles = ["node.text.italic"]`

---

### 3. 修改 `view/src/utils.rs`

更新宽度计算函数使用 `display_content`：

```rust
// 旧代码
pub fn calculate_item_width(item: &InLineItem) -> u16 {
    item.content.width() as u16
}

// 新代码
pub fn calculate_item_width(item: &InLineItem) -> u16 {
    item.display_content.width() as u16
}
```

---

### 4. 更新依赖 `view/Cargo.toml`

```toml
[dependencies]
bitflags = "2.4"  # 新增
```

---

## 样式渲染流程

### 当前状态（待实现）

`document.rs` 负责解析样式并存储 `style_name`，但实际渲染逻辑在 TUI 模块中。

### 推荐渲染方案

在 TUI 模块中（如 `tui/src/component/block/doc.rs`），使用 `styles::marks_to_style` 函数：

```rust
use view::styles::{marks_to_style, InlineMarks};

fn render_inline_item(item: &InLineItem, theme: &Theme) -> Span<'static> {
    // 方案 A: 使用 style_name 直接查找
    let style = theme.get(item.style_name.as_deref().unwrap_or(""));
    
    // 方案 B: 使用 marks 动态生成（推荐）
    let style = marks_to_style(&item.marks, &theme.styles);
    
    Span::styled(item.display_content.clone(), style)
}
```

---

## 测试覆盖

### 单元测试（`view/src/styles.rs`）

```rust
#[test]
fn test_parse_single_decor()  // 单个装饰样式
fn test_parse_multiple_decor() // 多个装饰样式叠加
fn test_parse_base_override()  // 基础样式覆盖
fn test_parse_mixed()          // 混合样式
fn test_parse_case_insensitive() // 大小写不敏感
```

### 集成测试

所有原有测试通过：
- `test_create_title_lines`
- `test_create_paragraph_lines`
- `test_list_block`
- `test_list_with_inline_linebreak`
- `test_lib_html_escape`

---

## 样式示例

| 输入 | base | decor | style_name |
|------|------|-------|------------|
| `"strong"` | `DEFAULT` | `STRONG` | `None` |
| `"em"` | `DEFAULT` | `EM` | `None` |
| `"code"` | `CODE` | `∅` | `"node.text.code"` |
| `"strong em code"` | `CODE` | `STRONG\|EM` | `"node.text.code"` |
| `"mark strong u"` | `MARK` | `STRONG\|U` | `"node.text.mark"` |
| `"a em"` | `A` | `EM` | `"node.text.weblink"` |

---

## 后续工作

### 1. TUI 渲染层集成 ✅ 已完成

在 `tui/src/component/editor.rs` 中已更新渲染逻辑：

```rust
// 合并多个样式：累加修饰符，后面的颜色覆盖前面的
let mut style = Style::default();
for style_name in &item.styles {
    let theme_style = cx.theme.get(&style_name.clone());
    // 累加修饰符（bold, italic, underline 等）
    style = style
        .add_modifier(theme_style.add_modifier)
        .remove_modifier(theme_style.sub_modifier);
    // 颜色使用最后一个非默认值（后面的覆盖前面的）
    if theme_style.fg.is_some() {
        style = style.fg(theme_style.fg.unwrap());
    }
    if theme_style.bg.is_some() {
        style = style.bg(theme_style.bg.unwrap());
    }
}
let span = Span::from(item.display_content.clone()).style(style);
rendered_line.push_span(span)
```

**样式叠加逻辑：**
- **修饰符累加**：`BOLD | ITALIC | UNDERLINED` 等修饰符会累加
- **颜色覆盖**：后面的颜色会覆盖前面的（fg 和 bg 分别处理）
- **顺序重要**：`["node.text.code", "node.text.mark"]` → 黄色背景覆盖深色背景

**示例：**
```
styles = ["node.text.code", "node.text.strong"]
→ fg=Green, bg=code_bg, modifier=BOLD

styles = ["node.text.mark", "node.text.strong"]  
→ fg=default, bg=Yellow, modifier=BOLD

styles = ["node.text.code", "node.text.strong", "node.text.underline"]
→ fg=Green, bg=code_bg, modifier=BOLD|UNDERLINED
```

### 2. 主题配置优化

在 `theme.toml` 中添加更多复合样式快捷方式（可选）：

```toml
# 复合样式示例
"strong.em" = { fg = "fg", modifier = "bold|italic" }
"code.strong" = { fg = "green", bg = "code_bg", modifier = "bold" }
```

### 3. 样式优先级配置

如果某些样式组合需要特殊处理，可以在 `styles.rs` 中添加：

```rust
pub fn marks_to_style_with_overrides(
    marks: &InlineMarks,
    style_map: &HashMap<String, Style>,
    overrides: &HashMap<InlineMarks, Style>
) -> Style {
    // 检查是否有特殊覆盖
    if let Some(override_style) = overrides.get(marks) {
        return *override_style;
    }
    // 默认逻辑
    marks_to_style(marks, style_map)
}
```

---

## 性能考虑

- **解析性能**: O(n)，n 为样式字符串中的单词数
- **存储开销**: `InlineMarks` 占 2 字节（`BaseMark` 1 字节 + `DecorMark` 1 字节）
- **渲染性能**: O(1) 查找 + O(装饰样式数量) 叠加

---

## 兼容性

- **向后兼容**: 单一样式（如 `"strong"`）仍然正常工作
- **思源兼容**: 完全兼容思源的 `"strong em"` 拼接格式
- **扩展性**: 新增样式只需在 BitFlags 中添加常量

---

## 参考文档

- 设计方案：`/Users/crowds/RustroverProjects/scribe/view/doc/inline-mark-type-refactor.md`
- 主题配置：`/Users/crowds/RustroverProjects/scribe/theme.toml`
- 样式模块：`/Users/crowds/RustroverProjects/scribe/view/src/styles.rs`

---

## 总结

✅ **已完成：**
1. 新增 `styles.rs` 模块，定义 BitFlags 结构和解析逻辑
2. 新增 `BaseMarkKind` 枚举，支持高效的 match 匹配
3. 重构 `InLineItem` 结构体，`styles` 字段改为 `Vec<String>`
4. 更新 `create_node_text_mark` 函数，构建样式列表（基础 + 装饰）
5. 更新 `utils.rs` 中的宽度计算函数
6. 更新 `tui/src/component/editor.rs` 渲染逻辑，按顺序应用多个样式
7. 所有单元测试和集成测试通过

📊 **测试覆盖：**
- 5 个 styles 模块单元测试
- 6 个 document 模块集成测试
- 8 个样式嵌套测试（`style_nesting_test.rs`）
- 7 个样式合并测试（`style_merge_test.rs`）

🎯 **核心优势：**
- 支持复合样式（如 `"strong em code"`）
- 互斥样式自动覆盖（如 `"code mark"` → `MARK`）
- 装饰样式自动叠加（如 `"strong em"` → `STRONG | EM`）
- 样式列表按顺序应用，支持灵活的主题配置
- 完全兼容思源笔记的样式格式

📋 **样式列表示例：**

| 输入 | styles 列表 |
|------|------------|
| `"strong em"` | `["node.text.strong", "node.text.italic"]` |
| `"code"` | `["node.text.code"]` |
| `"strong em code"` | `["node.text.code", "node.text.strong", "node.text.italic"]` |
| `"mark strong u"` | `["node.text.mark", "node.text.strong", "node.text.underline"]` |
| `"a em"` | `["node.text.weblink", "node.text.italic"]` |
