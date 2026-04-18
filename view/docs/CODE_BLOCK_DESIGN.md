# 代码块渲染设计文档

**文档状态：** 草案  
**创建日期：** 2026-03-31  
**作者：** Scribe Team  
**相关模块：** `view/src/document.rs`, `tui/src/component/block/doc.rs`

---

## 1. 概述

### 1.1 目标

在 TUI 中正确渲染思源笔记的代码块节点（`NodeCodeBlock`），提供：
- 清晰的视觉边界（边框）
- 语言标识显示
- 语法高亮支持（预留接口）
- 长代码自动换行/滚动

### 1.2 渲染效果

```
┌─[rust]─────────────────────────────────┐
│ fn main() {                            │
│     println!("Hello, world!");         │
│ }                                      │
└────────────────────────────────────────┘
```

### 1.3 输入数据示例

```json
{
  "ID": "20250619160746-6njrpbf",
  "Type": "NodeCodeBlock",
  "IsFencedCodeBlock": true,
  "Properties": {
    "id": "20250619160746-6njrpbf",
    "style": "line-height: 28px;",
    "updated": "20251205112140"
  },
  "Children": [
    {
      "Type": "NodeCodeBlockFenceOpenMarker",
      "Data": "```"
    },
    {
      "Type": "NodeCodeBlockFenceInfoMarker",
      "CodeBlockInfo": "cnVzdA=="  // Base64 编码的 "rust"
    },
    {
      "Type": "NodeCodeBlockCode",
      "Data": "let rust = \"Hello world!\"\n..."
    },
    {
      "Type": "NodeCodeBlockFenceCloseMarker",
      "Data": "```"
    }
  ]
}
```

---

## 2. 架构设计

### 2.1 模块划分

```
view/src/
├── document.rs          # AST → DocumentModel 转换
├── code_block.rs        # 代码块专用处理（新增）
│   ├── parser.rs        # 代码块解析
│   ├── renderer.rs      # 渲染逻辑
│   └── highlight.rs     # 语法高亮接口（预留）
└── styles.rs            # 样式定义

tui/src/
├── component/
│   └── block/
│       ├── doc.rs       # 文档块渲染
│       └── code.rs      # 代码块组件（新增）
```

### 2.2 数据流

```
NodeCodeBlock (AST)
    ↓
parse_code_block()
    ↓
CodeBlockModel {
    language: String,
    content: String,
    lines: Vec<String>,
}
    ↓
render_code_block()
    ↓
Vec<InLineItem> (带样式的行)
    ↓
TUI 渲染（带边框）
```

---

## 3. 详细设计

### 3.1 CodeBlockModel 结构

```rust
/// 代码块模型
#[derive(Clone, Debug)]
pub struct CodeBlockModel {
    /// 语言标识（如 "rust", "python", "markdown"）
    pub language: String,
    /// 原始代码内容
    pub content: String,
    /// 按行分割的代码（用于逐行渲染）
    pub lines: Vec<String>,
    /// 是否被围栏包围（```）
    pub is_fenced: bool,
    /// 代码块 ID（用于语法高亮缓存）
    pub id: String,
}

impl CodeBlockModel {
    /// 从 Node 解析代码块
    pub fn from_node(node: &Node) -> Self {
        // 实现见 3.2
    }
    
    /// 获取显示用的语言名称
    pub fn language_display(&self) -> &str {
        if self.language.is_empty() { "code" } else { &self.language }
    }
}
```

### 3.2 解析逻辑

```rust
impl CodeBlockModel {
    pub fn from_node(node: &Node) -> Self {
        let mut language = String::new();
        let mut content = String::new();
        let is_fenced = node.is_fenced_code_block;
        
        // 遍历子节点
        for child in &node.children {
            match child.node_type {
                NodeType::NodeCodeBlockFenceInfoMarker => {
                    // 解析语言标识（Base64 解码）
                    language = decode_base64(&child.code_block_info)
                        .unwrap_or_default();
                }
                NodeType::NodeCodeBlockCode => {
                    content = child.data.clone().unwrap_or_default();
                }
                _ => {}
            }
        }
        
        // 按行分割
        let lines: Vec<String> = content.lines().map(String::from).collect();
        
        // 提取 ID
        let id = node.id.clone().unwrap_or_default();
        
        Self {
            language,
            content,
            lines,
            is_fenced,
            id,
        }
    }
}
```

### 3.3 在 document.rs 中的集成

```rust
// document.rs - parse_node_by_type 方法
fn parse_node_by_type(&mut self, node: &Node) -> Vec<DocumentLine> {
    let available_width = self.max_line_len;
    let mut result: Vec<DocumentLine> = Vec::new();
    
    let mut lines = match node.node_type {
        NodeType::NodeParagraph => self.create_paragraph_block_lines(node, available_width),
        NodeType::NodeHeading => self.create_head_block_lines(node, available_width),
        NodeType::NodeList => self.create_list_block_lines(node, available_width),
        NodeType::NodeCodeBlock => self.create_code_block_lines(node, available_width), // 新增
        _ => Vec::new(),
    };
    
    result.append(&mut lines);
    result
}

// 新增方法
fn create_code_block_lines(&mut self, node: &Node, available_width: u16) -> Vec<DocumentLine> {
    let code_block = CodeBlockModel::from_node(node);
    let mut doc_lines = Vec::new();
    
    // 1. 顶部边框（带语言标识）
    doc_lines.push(self.create_code_block_header(&code_block, available_width));
    
    // 2. 代码内容（逐行处理）
    for line in &code_block.lines {
        let content_lines = self.split_code_line(line, available_width - 4); // 减去边框宽度
        for code_line in content_lines {
            doc_lines.push(self.create_code_block_content_line(code_line));
        }
    }
    
    // 3. 底部边框
    doc_lines.push(self.create_code_block_footer(available_width));
    
    doc_lines
}
```

### 3.4 渲染样式

```rust
// styles.rs - 新增代码块样式
bitflags! {
    pub struct CodeBlockMark: u8 {
        const BORDER   = 1 << 0;  // 边框
        const HEADER   = 1 << 1;  // 头部（语言标识）
        const CONTENT  = 1 << 2;  // 代码内容
        const LINE_NUM = 1 << 3;  // 行号（未来扩展）
    }
}

// theme.toml - 新增样式配置
"code.block" = { fg = "fg", bg = "bg_darker" }
"code.block.border" = { fg = "comment" }
"code.block.header" = { fg = "cyan", modifier = "bold" }
"code.block.content" = { fg = "green" }
"code.block.line_num" = { fg = "comment" }
```

### 3.5 TUI 组件

```rust
// tui/src/component/block/code.rs
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

pub struct CodeBlock<'a> {
    language: &'a str,
    lines: Vec<&'a str>,
    theme: &'a Theme,
}

impl<'a> CodeBlock<'a> {
    pub fn new(language: &'a str, lines: Vec<&'a str>, theme: &'a Theme) -> Self {
        Self { language, lines, theme }
    }
}

impl<'a> Widget for CodeBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // 创建带边框的块
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", self.language))
            .style(self.theme.get("code.block"));
        
        let inner = block.inner(area);
        block.render(area, buf);
        
        // 逐行渲染代码内容
        for (i, line) in self.lines.iter().enumerate() {
            if inner.y + i as u16 >= inner.bottom() {
                break;
            }
            
            let span = Span::styled(
                line,
                self.theme.get("code.block.content"),
            );
            
            buf.set_span(inner.x, inner.y + i as u16, &span, inner.width as usize);
        }
    }
}
```

---

## 4. 语法高亮接口设计

### 4.1 Trait 定义

```rust
// view/src/code_block/highlight.rs

/// 语法高亮器 Trait
pub trait SyntaxHighlighter: Send + Sync {
    /// 高亮代码，返回带样式的行
    fn highlight(&self, language: &str, code: &str) -> Vec<StyledLine>;
    
    /// 检查是否支持该语言
    fn supports_language(&self, language: &str) -> bool;
}

/// 带样式的行
pub struct StyledLine {
    pub spans: Vec<StyledSpan>,
}

pub struct StyledSpan {
    pub content: String,
    pub style: Style,
}

/// 默认实现（无高亮）
pub struct NoOpHighlighter;

impl SyntaxHighlighter for NoOpHighlighter {
    fn highlight(&self, _language: &str, code: &str) -> Vec<StyledLine> {
        code.lines()
            .map(|line| StyledLine {
                spans: vec![StyledSpan {
                    content: line.to_string(),
                    style: Style::default(),
                }],
            })
            .collect()
    }
    
    fn supports_language(&self, _language: &str) -> bool { false }
}
```

### 4.2 Tree-sitter 集成（未来）

```rust
// 未来实现示例
pub struct TreeSitterHighlighter {
    // Tree-sitter 相关字段
}

impl SyntaxHighlighter for TreeSitterHighlighter {
    fn highlight(&self, language: &str, code: &str) -> Vec<StyledLine> {
        // 1. 根据 language 选择对应的 Tree-sitter Parser
        // 2. 解析代码生成语法树
        // 3. 遍历语法树，识别 token 类型
        // 4. 根据 token 类型应用样式
        // 5. 返回带样式的行
    }
}
```

### 4.3 配置集成

```rust
// config.rs
pub struct CodeConfig {
    /// 是否启用语法高亮
    pub enable_highlight: bool,
    /// 高亮器选择（"none", "tree-sitter", "syntect"）
    pub highlighter: String,
    /// 显示行号
    pub show_line_numbers: bool,
    /// 自动换行
    pub word_wrap: bool,
}
```

---

## 5. 长代码处理

### 5.1 换行策略

```rust
fn split_code_line(line: &str, max_width: u16) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;
    
    for c in line.chars() {
        let char_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(1);
        
        if current_width + char_width > max_width {
            // 超出行宽，换行
            result.push(current_line);
            current_line = String::new();
            current_width = 0;
        }
        
        current_line.push(c);
        current_width += char_width;
    }
    
    if !current_line.is_empty() {
        result.push(current_line);
    }
    
    result
}
```

### 5.2 滚动支持（未来）

对于超长代码块，可以考虑：
- 垂直滚动：只显示可视区域内的行
- 水平滚动：单行过长时支持左右滚动
- 折叠：支持代码折叠（基于缩进或区域标记）

---

## 6. 测试计划

### 6.1 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_code_block_with_language() {
        // 测试解析带语言标识的代码块
    }
    
    #[test]
    fn test_parse_code_block_without_language() {
        // 测试解析无语言标识的代码块
    }
    
    #[test]
    fn test_split_long_code_line() {
        // 测试长代码行换行
    }
    
    #[test]
    fn test_code_block_base64_decode() {
        // 测试 Base64 解码语言标识
    }
}
```

### 6.2 集成测试

```rust
#[test]
fn test_render_code_block_rust() {
    // 测试 Rust 代码块渲染
}

#[test]
fn test_render_code_block_multiline() {
    // 测试多行代码块渲染
}

#[test]
fn test_render_code_block_long_line() {
    // 测试超长行换行
}
```

---

## 7. 实现计划

### Phase 1: 基础渲染（本周）
- [ ] 创建 `CodeBlockModel` 结构
- [ ] 实现 `from_node` 解析方法
- [ ] 在 `document.rs` 中集成 `NodeCodeBlock` 处理
- [ ] 实现基础边框渲染

### Phase 2: 样式优化（下周）
- [ ] 添加代码块主题配置
- [ ] 实现语言标识显示
- [ ] 优化边框样式

### Phase 3: 语法高亮（未来）
- [ ] 定义 `SyntaxHighlighter` Trait
- [ ] 实现 `NoOpHighlighter`（默认）
- [ ] 集成 Tree-sitter 或其他高亮库

### Phase 4: 高级功能（未来）
- [ ] 行号显示
- [ ] 代码折叠
- [ ] 滚动支持

---

## 8. 参考资料

- [思源笔记 API 文档](https://github.com/siyuan-note/siyuan/blob/master/API.md)
- [Tree-sitter](https://tree-sitter.github.io/tree-sitter/)
- [Syntect](https://github.com/trishume/syntect) - Sublime Text 语法高亮
- [Ratatui Widgets](https://docs.rs/ratatui/latest/ratatui/widgets/)

---

## 9. 修订历史

| 日期 | 版本 | 作者 | 变更 |
|------|------|------|------|
| 2026-03-31 | 0.1 | Scribe Team | 初始草案 |
| 2026-03-31 | 0.2 | Scribe Team | 添加设计评审问答 |

---

## 10. 设计评审问答

### Q1: 样式方案问题

**问题：** 功能的整体实现很好！为什么样式部分这里也还是采用了 bitflag，这里存在同一个"内容"的样式嵌套吗？这套样式方案如果对接了 tree-sitter 是否能无缝对接？

**回答：**

感谢评审！这里需要澄清和修正：

#### 10.1.1 代码块样式的分层设计

代码块的样式应该分为**两个独立层级**：

```
┌─[rust]──────────────┐
│ 代码内容行 1          │
│ 代码内容行 2          │
└─────────────────────┘
  ↑                   ↑
  容器样式             内容样式
  (bitflag)           (SyntaxHighlighter)
```

**层级 1：容器样式（使用 bitflag）**
- 边框、背景色、头部语言标识
- 这些是**单一整体样式**，不需要嵌套
- 适合使用简单的主题样式查找

```rust
// 修正后的设计 - 不需要 bitflag
pub struct CodeBlockStyle {
    pub border_style: Style,      // 边框样式
    pub header_style: Style,      // 头部样式
    pub background: Style,        // 背景色
}
```

**层级 2：代码内容样式（使用 SyntaxHighlighter）**
- 每个 token（关键字、字符串、注释等）有独立样式
- **不是样式叠加**，而是**token 级别的独立样式**
- 通过 `SyntaxHighlighter` trait 实现

```rust
// 正确的高亮接口
pub trait SyntaxHighlighter {
    fn highlight(&self, language: &str, code: &str) -> Vec<StyledLine>;
}

pub struct StyledLine {
    pub spans: Vec<StyledSpan>,  // 每个 span 有独立样式
}

pub struct StyledSpan {
    pub content: String,
    pub style: Style,  // 不是 bitflag，是直接样式
}
```

#### 10.1.2 Tree-sitter 对接

**可以无缝对接**，因为：

1. **Trait 抽象**：`SyntaxHighlighter` trait 定义了统一接口
2. **插件化设计**：不同高亮器实现同一 trait

```rust
// Tree-sitter 实现
pub struct TreeSitterHighlighter {
    // Tree-sitter 相关字段
}

impl SyntaxHighlighter for TreeSitterHighlighter {
    fn highlight(&self, language: &str, code: &str) -> Vec<StyledLine> {
        // 1. 创建对应语言的 Parser
        let parser = self.get_parser(language);
        
        // 2. 解析代码生成语法树
        let tree = parser.parse(code, None);
        
        // 3. 遍历语法树，识别 token
        let tokens = self.tokenize(tree);
        
        // 4. 根据 token 类型应用样式
        tokens.into_iter().map(|token| {
            StyledSpan {
                content: token.content,
                style: self.get_style(token.kind),
            }
        }).collect()
    }
}

// 无高亮实现（默认）
pub struct NoOpHighlighter;

impl SyntaxHighlighter for NoOpHighlighter {
    fn highlight(&self, _language: &str, code: &str) -> Vec<StyledLine> {
        // 返回单一样式的行
        code.lines().map(|line| StyledLine {
            spans: vec![StyledSpan {
                content: line.to_string(),
                style: Style::default(),
            }],
        }).collect()
    }
}
```

#### 10.1.3 设计修正

**原文档中的问题：**
- 错误地引入了 `CodeBlockMark` bitflag
- 混淆了容器样式和内容高亮

**修正方案：**
1. 移除 `CodeBlockMark` bitflag
2. 容器样式使用简单的 `Style` 结构
3. 内容高亮通过 `SyntaxHighlighter` trait 处理

**修正后的数据流：**
```
NodeCodeBlock
    ↓
CodeBlockModel { language, content, lines }
    ↓
┌───────────────────────────────────┐
│ 容器渲染                           │
│ - 边框：theme.get("code.block.border") │
│ - 头部：theme.get("code.block.header") │
│ - 背景：theme.get("code.block.bg")   │
└───────────────────────────────────┘
    ↓
┌───────────────────────────────────┐
│ 内容渲染 (SyntaxHighlighter)      │
│ - Rust: Token[keyword] → Style1   │
│ - Rust: Token[string] → Style2    │
│ - Python: Token[def] → Style3     │
└───────────────────────────────────┘
```

---

### Q2: 项目文档管理问题

**问题：** Rust 本身似乎还提供了将项目文档导出成可以直接访问的 HTML 网站，类似 Rust 本身的官方教程。当前方案中项目文档还是零散在各处的，比如 view 模块下就有 doc 和 docs。

**回答：**

感谢指出！这里需要区分**两种不同类型的文档**：

#### 10.2.1 文档类型区分

| 文档类型 | 用途 | 格式 | 生成方式 | 示例 |
|----------|------|------|----------|------|
| **API 文档** | 代码接口说明 | Rust 注释 | `cargo doc` | `/// 函数说明` |
| **设计文档** | 架构决策、实现计划 | Markdown | 手动编写 | `CODE_BLOCK_DESIGN.md` |
| **用户文档** | 使用指南 | Markdown | 手动编写 | `README.md` |

#### 10.2.2 Rust 官方文档工具

**`cargo doc` 生成的是 API 文档：**

```bash
# 生成 API 文档（从代码注释）
cargo doc --open

# 输出到 target/doc/
# 可以通过浏览器访问
```

**示例：代码注释生成文档**
```rust
/// 代码块模型
/// 
/// 用于表示思源笔记中的代码块节点。
/// 
/// # Examples
/// 
/// ```
/// let block = CodeBlockModel::from_node(&node);
/// ```
pub struct CodeBlockModel {
    // ...
}
```

**局限性：**
- ❌ 只能从代码注释生成
- ❌ 不适合架构图、设计决策
- ❌ 不适合实现计划、替代方案

#### 10.2.3 当前问题

**问题 1：命名不统一**
```
view/doc/          # 设计文档
syservice/doc/     # 设计文档
docs/              # ？（未定义）
```

**问题 2：缺少统一入口**
- 没有文档索引页
- 难以查找相关文档

#### 10.2.4 修正方案

**统一文档结构：**
```
scribe/
├── README.md                    # 项目概述（第一入口）
├── CONTRIBUTING.md              # 贡献指南
├── DOC_GUIDE.md                 # 文档规范
│
├── docs/                        # 统一文档目录（新建）
│   ├── README.md                # 文档索引（新建）
│   ├── design/                  # 设计文档
│   │   ├── 001-code-block.md
│   │   └── 002-inline-styles.md
│   ├── user-guide/              # 用户文档
│   │   └── getting-started.md
│   └── api/                     # API 文档（cargo doc 生成）
│
├── view/
│   └── src/
│       ├── lib.rs               # 包含模块级文档注释
│       └── document.rs          # 包含函数级文档注释
│
└── target/
    └── doc/                     # cargo doc 输出（自动生成）
```

**docs/README.md 示例：**
```markdown
# Scribe 文档中心

## 📚 文档分类

### 设计文档
- [代码块渲染设计](design/001-code-block.md)
- [行内样式重构](design/002-inline-styles.md)

### 用户文档
- [快速开始](user-guide/getting-started.md)
- [配置指南](user-guide/config.md)

### API 文档
- [View API](../target/doc/view/)
- [Syservice API](../target/doc/syservice/)

## 🔧 生成本地文档

```bash
# 生成 API 文档
cargo doc --open

# 查看设计文档
# 直接在浏览器中打开 docs/README.md
```
```

#### 10.2.5 文档导出为 HTML 网站

**方案 1：使用 mdBook（推荐）**

```bash
# 安装
cargo install mdbook

# 创建书籍
mdbook init docs/book

# 本地预览
mdbook serve docs/book
```

**方案 2：使用 cargo-docs**

```toml
# Cargo.toml
[package.metadata.docs]
enabled = true
```

**方案 3：GitHub Pages**
```bash
# 推送 docs/ 到 gh-pages 分支
# 自动部署为 GitHub Pages 网站
```

#### 10.2.6 行动项

- [ ] 统一文档目录为 `docs/`
- [ ] 创建 `docs/README.md` 索引页
- [ ] 迁移现有设计文档到 `docs/design/`
- [ ] 考虑使用 mdBook 生成 HTML 网站
- [ ] 在 README.md 中添加文档链接

---

## 11. 待办事项（根据评审更新）

### 样式方案修正
- [ ] 移除 `CodeBlockMark` bitflag
- [ ] 使用简单 `Style` 结构作为容器样式
- [ ] 完善 `SyntaxHighlighter` trait 设计
- [ ] 添加 `NoOpHighlighter` 实现
- [ ] 更新主题配置示例

### 文档结构优化
- [ ] 创建 `docs/` 目录
- [ ] 创建 `docs/README.md` 索引
- [ ] 迁移 `view/doc/` → `docs/design/`
- [ ] 迁移 `syservice/doc/` → `docs/design/`
- [ ] 考虑集成 mdBook

### 代码实现调整
- [ ] 按照修正后的设计实现 Phase 1
- [ ] 确保与 Tree-sitter 接口兼容
