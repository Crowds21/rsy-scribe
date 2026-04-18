# 代码块渲染设计文档

**状态：** Phase 1 已完成  
**创建日期：** 2026-03-31  
**更新日期：** 2026-03-31  
**作者：** Scribe Team  
**相关 Issue：** #N/A

---

## 1. 概述

### 1.1 目标

在 TUI 中正确渲染思源笔记的代码块节点（`NodeCodeBlock`），提供：
- 清晰的视觉边界（边框）
- 语言标识显示
- 语法高亮支持（预留接口）
- 长代码自动换行

### 1.2 渲染效果

```
┌─[rust]─────────────────────────────────┐
│ fn main() {                            │
│     println!("Hello, world!");         │
│ }                                      │
└────────────────────────────────────────┘
```

---

## 2. 实现状态

### Phase 1: 基础渲染 ✅ 已完成

- [x] 创建 `CodeBlockModel` 结构
- [x] 实现 `from_node` 解析方法（含 Base64 解码）
- [x] 在 `document.rs` 中集成 `NodeCodeBlock` 处理
- [x] 实现基础边框渲染
- [x] 实现长代码换行
- [x] 定义 `SyntaxHighlighter` trait
- [x] 实现 `NoOpHighlighter`（默认无高亮）

### Phase 2: 样式优化 ⏳ 待实现

- [ ] 添加代码块主题配置
- [ ] 优化边框样式
- [ ] 支持行号显示

### Phase 3: 语法高亮 ⏳ 待实现

- [ ] 集成 Tree-sitter 或 Syntect
- [ ] 实现语言自动检测
- [ ] 高亮性能优化

### Phase 4: 高级功能 ⏳ 待实现

- [ ] 代码折叠
- [ ] 滚动支持
- [ ] 代码复制功能

---

## 3. 架构设计

### 3.1 模块结构

```
view/src/code_block/
├── mod.rs          # 模块导出
├── model.rs        # CodeBlockModel 数据模型
├── parser.rs       # 解析逻辑（Base64 解码、换行）
├── renderer.rs     # 渲染逻辑（边框、样式）
└── highlight.rs    # 语法高亮接口
```

### 3.2 数据流

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

## 4. 设计评审问答

### Q1: 样式方案

**问题：** 为什么样式部分采用了 bitflag？是否存在样式嵌套？能否对接 Tree-sitter？

**回答：**

代码块样式分为**两个独立层级**：

1. **容器样式**（边框、背景、头部）→ 简单 `Style`，不需要 bitflag
2. **内容高亮**（token 级别）→ `SyntaxHighlighter` trait

**Tree-sitter 对接：** 通过 `SyntaxHighlighter` trait 无缝集成：

```rust
pub trait SyntaxHighlighter {
    fn highlight(&self, language: &str, code: &str) -> Vec<StyledLine>;
}

// Tree-sitter 实现
impl SyntaxHighlighter for TreeSitterHighlighter { ... }
```

### Q2: 文档管理

**问题：** Rust 有 `cargo doc`，当前文档零散在各处。

**回答：**

已统一文档结构：

```
scribe/docs/
├── README.md                # 文档索引
├── design/                  # 设计文档
│   ├── 001-code-block-rendering.md
│   └── ...
└── user-guide/              # 用户文档
```

API 文档通过 `cargo doc --open` 生成。

---

## 5. 测试覆盖

### 单元测试

```bash
cargo test -p view code_block
```

**测试结果：** 12 个测试全部通过

| 测试 | 说明 | 状态 |
|------|------|------|
| `test_decode_base64` | Base64 解码 | ✅ |
| `test_split_short_line` | 短行不分割 | ✅ |
| `test_split_long_line` | 长行分割 | ✅ |
| `test_split_unicode` | Unicode 字符处理 | ✅ |
| `test_noop_highlighter` | 无高亮渲染 | ✅ |
| `test_create_header_item` | 头部渲染 | ✅ |
| `test_create_footer_item` | 底部渲染 | ✅ |

### 集成测试

待实现（需要完整的 TUI 渲染环境）

---

## 6. 已知问题

1. **样式配置** - 当前使用硬编码样式，需要从主题加载
2. **语法高亮** - 仅支持 `NoOpHighlighter`（无高亮）
3. **性能** - 长代码块（>1000 行）可能需要优化

---

## 7. 后续计划

### 短期（本周）
- [ ] 集成主题配置
- [ ] 优化边框样式
- [ ] 添加代码块测试用例

### 中期（下周）
- [ ] 调研 Tree-sitter 集成方案
- [ ] 实现行号显示
- [ ] 性能基准测试

### 长期
- [ ] 代码折叠功能
- [ ] 滚动支持
- [ ] 代码搜索/跳转

---

## 8. 参考资料

- [思源笔记 API 文档](https://github.com/siyuan-note/siyuan/blob/master/API.md)
- [Tree-sitter](https://tree-sitter.github.io/tree-sitter/)
- [Ratatui Widgets](https://docs.rs/ratatui/latest/ratatui/widgets/)

---

## 9. 修订历史

| 日期 | 版本 | 作者 | 变更 |
|------|------|------|------|
| 2026-03-31 | 0.1 | Scribe Team | 初始草案 |
| 2026-03-31 | 0.2 | Scribe Team | 设计评审问答 |
| 2026-03-31 | 1.0 | Scribe Team | Phase 1 完成 |
