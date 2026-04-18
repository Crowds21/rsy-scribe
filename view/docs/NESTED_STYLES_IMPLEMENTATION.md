# 嵌套样式实现说明

## 📝 修改文件

### 1. `theme.toml` - OneDark 配色方案

**主要变更：**
- ✅ 采用 Atom OneDark Pro 配色方案
- ✅ 支持所有样式组合（粗体、斜体、下划线、删除线及其组合）
- ✅ 添加完整的组合样式定义（如 `strong.em`、`strong.u` 等）

**配色特点：**
```toml
bg = "#282c34"      # 主背景
fg = "#abb2bf"      # 主前景
red = "#e06c75"     # 标题 H1
orange = "#d19a66"  # 标题 H2
yellow = "#e5c07b"  # 高亮背景
green = "#98c379"   # 代码、成功
cyan = "#56b6c2"    # 链接、数学
blue = "#61afef"    # 链接、标题 H6
purple = "#c678dd"  # 标签、关键字
```

---

### 2. `view/src/document.rs` - 嵌套样式支持

#### 新增结构体：`InlineStyles`

```rust
#[derive(Clone, Default, Debug)]
pub struct InlineStyles {
    pub strong: bool,        // 粗体
    pub em: bool,            // 斜体
    pub mark: bool,          // 高亮
    pub code: bool,          // 代码
    pub underline: bool,     // 下划线
    pub strikethrough: bool, // 删除线
}
```

**核心方法：**

1. **`parse(style_str: &str)`** - 从字符串解析样式
   ```rust
   let styles = InlineStyles::parse("strong em");
   // 结果：strong=true, em=true, 其他=false
   ```

2. **`to_theme_key()`** - 生成主题键
   ```rust
   styles.to_theme_key()  // 返回 "strong.em"
   ```

3. **`to_style()`** - 转换为 ratatui Style
   ```rust
   let style = styles.to_style();
   // 自动添加 BOLD | ITALIC 等修饰符
   ```

---

#### 修改的结构体：`InLineItem`

**新增字段：**
```rust
pub struct InLineItem {
    pub item_type: InLineMarkType,
    pub content: String,
    pub styles: InlineStyles,      // ✨ 新增：支持组合样式
    pub link: Option<String>,
    pub style: Option<String>,
    pub theme_key: Option<String>, // ✨ 新增：主题查找键
    pub line_break: bool,
}
```

---

#### 修改的函数

##### 1. `create_node_text_mark()` 

**功能：** 处理 `NodeTextMark` 节点，支持组合样式

**支持的组合：**
| 组合 | TextMarkType | theme_key |
|------|-------------|-----------|
| 粗体 | `strong` | `strong` |
| 斜体 | `em` | `em` |
| 粗体 + 斜体 | `strong em` | `strong.em` |
| 粗体 + 下划线 | `strong underline` | `strong.u` |
| 粗体 + 删除线 | `strong strike` | `strong.s` |
| 斜体 + 下划线 | `em underline` | `em.u` |
| 斜体 + 删除线 | `em strike` | `em.s` |
| 下划线 + 删除线 | `underline strike` | `u.s` |
| 三组合 | `strong em underline` | `strong.em.u` |

**实现逻辑：**
```rust
fn create_node_text_mark(&mut self, node: &Node) -> (InLineItem, u16) {
    // 1. 解析样式字符串（支持 "strong em" 组合）
    let styles = InlineStyles::parse(&mark_type);
    
    // 2. 检测组合样式
    if mark_type.contains("strong") && mark_type.contains("em") {
        item.theme_key = Some("strong.em".to_string());
    }
    // ... 其他组合
    
    // 3. 设置样式
    item.styles = styles;
    item.style = Some(mark_type);
    
    (item, width)
}
```

##### 2. `create_node_text()`

**修改：** 添加 `styles` 和 `theme_key` 字段初始化

##### 3. `split_inline_item()`

**修改：** 分割时保留样式信息
```rust
let first_part = InLineItem {
    styles: item.styles.clone(),      // ✨ 保留样式
    theme_key: item.theme_key.clone(), // ✨ 保留主题键
    ...
};
```

---

## 🎯 支持的样式组合

### InLineMarkType 枚举

```rust
enum InLineMarkType {
    #[default]
    Default,
    Strong,      // 粗体
    Em,          // 斜体
    Mark,        // 高亮
    Code,        // 代码
    BlockRef,    // 块引用
    A,           // 超链接
    Tag,         // 标签
    U,           // 下划线
    S,           // 删除线
}
```

### 组合方式

**单一样式：**
- ✅ `strong` - 粗体
- ✅ `em` - 斜体
- ✅ `u` - 下划线
- ✅ `s` - 删除线
- ✅ `mark` - 高亮
- ✅ `code` - 代码
- ✅ `link` - 链接
- ✅ `tag` - 标签

**双组合：**
- ✅ `strong em` - 粗体 + 斜体
- ✅ `strong u` - 粗体 + 下划线
- ✅ `strong s` - 粗体 + 删除线
- ✅ `em u` - 斜体 + 下划线
- ✅ `em s` - 斜体 + 删除线
- ✅ `u s` - 下划线 + 删除线

**三组合：**
- ✅ `strong em u` - 粗体 + 斜体 + 下划线
- ✅ `strong em s` - 粗体 + 斜体 + 删除线
- ✅ `strong u s` - 粗体 + 下划线 + 删除线
- ✅ `em u s` - 斜体 + 下划线 + 删除线

---

## 📊 测试验证

### 单元测试

运行测试验证样式嵌套：
```bash
cargo test --package view --test style_nesting_test
```

**测试结果：**
```
running 8 tests
test test_bold_and_italic_nesting ... ok
test test_bold_and_underline_nesting ... ok
test test_three_modifiers_nesting ... ok
test test_parse_style_from_string ... ok
...
test result: ok. 8 passed; 0 failed
```

### 可视化测试

运行真实终端测试：
```bash
cargo run --package view --example test_styles
```

---

## 🔧 使用示例

### 思源笔记中的 TextMark

```json
{
  "Type": "NodeTextMark",
  "TextMarkType": "strong em",
  "TextMarkTextContent": "这是粗体 + 斜体文本"
}
```

**解析结果：**
```rust
InLineItem {
    item_type: InLineMarkType::Default,
    content: "这是粗体 + 斜体文本",
    styles: InlineStyles {
        strong: true,
        em: true,
        ..Default::default()
    },
    theme_key: Some("strong.em"),
    style: Some("strong.em"),
}
```

**渲染效果：**
- 文本显示为 **粗体 + 斜体**
- 从 theme.toml 查找 `strong.em` 样式
- 应用 `modifier = "bold italic"`

---

## 🎨 主题配置示例

### 在 theme.toml 中定义组合样式

```toml
# 单一样式
"strong" = { fg = "fg", modifier = "bold" }
"em" = { fg = "fg", modifier = "italic" }

# 组合样式
"strong.em" = { fg = "fg", modifier = "bold italic" }
"strong.u" = { fg = "fg", modifier = "bold underlined" }
"em.s" = { fg = "fg", modifier = "italic crossed_out" }

# 三组合
"strong.em.u" = { fg = "fg", modifier = "bold italic underlined" }
```

---

## ⚠️ 注意事项

### 1. 样式优先级

当同时存在多种样式定义时：
1. 优先使用 `theme_key` 查找主题
2. 如果找不到，使用 `style` 作为后备
3. 最后应用 `InlineStyles` 的基础样式

### 2. 终端兼容性

不是所有终端都支持所有样式组合：
- **粗体 + 斜体**：大多数现代终端支持 ✅
- **下划线**：基本都支持 ✅
- **删除线**：部分终端不支持 ⚠️
- **三组合**：取决于终端实现 ⚠️

### 3. 向后兼容

- ✅ 保留了 `InLineMarkType` 枚举（向后兼容）
- ✅ 单一样式仍然通过 `item_type` 处理
- ✅ 新增 `InlineStyles` 支持组合样式

---

## 📚 相关文件

- `theme.toml` - OneDark 配色主题
- `view/src/document.rs` - 文档模型和样式解析
- `view/tests/style_nesting_test.rs` - 样式嵌套测试
- `view/examples/test_styles.rs` - 可视化测试工具
- `view/docs/style_nesting_solution.md` - 原始方案设计

---

## 🚀 下一步

1. **测试验证** - 在真实环境中测试各种组合
2. **性能优化** - 缓存解析后的样式
3. **更多组合** - 支持颜色 + 样式的组合
4. **主题切换** - 支持运行时切换主题

---

**修改完成时间：** 2026-03-29  
**基于：** Ratatui 样式嵌套测试结果  
**状态：** ✅ 编译通过，待实际测试验证
