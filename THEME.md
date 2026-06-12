# 主题系统文档

## 概述

Scribe 的主题与图标在**编译期**嵌入二进制：

- 默认主题：仓库根目录 [`theme.toml`](theme.toml)
- 默认图标：[`icons.toml`](icons.toml)
- 加载方式：`include_bytes!` + `Theme::default()` / `Icons::default()`（见 `tui/src/uiconfig/`）

修改主题后需重新编译：

```bash
cargo build -p app
```

[`theme.example.toml`](theme.example.toml) 提供常用样式键的示例片段，可复制到 `theme.toml` 中参考。

> **注意：** 当前版本**不支持**运行时从 `~/.config/scribe/theme.toml` 加载。若需自定义主题，请直接编辑仓库内 `theme.toml` 后重新构建。

## 样式键参考

### 编辑器与 Gutter

- `editor.bg` / `editor.fg` — 编辑器背景与默认文字
- `editor.gutter` / `editor.gutter.line_number` / `editor.gutter.icon` — 行号栏
- `ui.bufferline` / `ui.bufferline.active` / `ui.bufferline.inactive` — 顶部 Buffer 栏

### 标题

- `node.heading.h1` ~ `node.heading.h6` — 各级标题正文
- `node.heading.hN.decor` — 对应装饰线
- `node.heading.title` — 文档标题行

### 代码

- `code.block.border` / `code.block.header` / `code.block.content` — 代码块
- `code.inline` / `node.text.code` — 行内代码

### 行内元素

- `strong`, `em`, `mark`, `link`, `tag` — 快捷样式键
- `node.text.*` — 与思源 mark 对应的完整键

完整列表见 [`theme.toml`](theme.toml) 与 [THEME 设计说明](docs/design/002-inline-styles.md)。

## 样式格式

```toml
"style.name" = {
    fg = "red",           # 调色板名称或 #hex
    bg = "bg_darker",
    modifier = "bold"     # bold | italic | underlined | ...
}
```

调色板定义在 `theme.toml` 的 `[palette]` 段。

## 图标

Gutter 块图标来自 [`icons.toml`](icons.toml)（`head1`–`head6`、`code_block`、`outline` 等）。修改后同样需重新编译。

## 相关文档

- [006 — tui 模块](docs/design/006-tui-module.md)
- [002 — 行内样式](docs/design/002-inline-styles.md)
