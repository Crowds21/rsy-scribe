# Scribe

在终端中阅读 [思源笔记（SiYuan）](https://b3log.org/siyuan/) 文档的 Rust TUI 客户端。

Scribe 直接读取思源工作空间中的 `.sy` 文件，将块级 AST 解析为可渲染的行模型，并通过 Ratatui 在终端中展示标题、段落、列表、代码块等元素。适合在 SSH 环境、轻量终端或不想打开完整 IDE 时快速浏览笔记。

[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/crowds21/rsy-scribe)](https://rust-reportcard.xuri.me/report/github.com/crowds21/rsy-scribe)

## 功能概览

- **搜索打开文档** — 通过思源 API 搜索笔记，从工作空间加载 `.sy` 并渲染
- **Buffer 栏** — 顶部展示已打开文件，当前文档高亮，多文件间有视觉分隔
- **Gutter** — 行号、块类型 Nerd Font 图标（标题 / 文档名 / 多行代码块），独立配色
- **分级标题** — H1–H6 分级颜色与装饰线（双线、下划线、波浪线等）
- **代码块** — 圆角边框、语言标签、按终端宽度自动换行
- **行内样式** — 粗体、斜体、高亮、代码、链接等，样式来自 `theme.toml`
- **滚动阅读** — 方向键 / PageUp / PageDown / Home / End 浏览长文档
- **主题配置** — `theme.toml`、`icons.toml` 可自定义颜色与图标

## 环境要求

- Rust 1.70+（edition 2021）
- 已安装并运行的思源笔记，且 API 可访问（默认 `http://127.0.0.1:6806`）
- 支持 Nerd Font 的终端（Gutter 图标显示更佳）

## 快速开始

### 1. 克隆与构建

```bash
git clone https://github.com/Crowds21/rsy-scribe.git
cd rsy-scribe
cargo build --release
```

### 2. 配置

复制示例配置并按本机路径修改：

```bash
mkdir -p ~/.config/scribe
cp syservice/config.example.toml ~/.config/scribe/config.toml
```

`config.toml` 示例：

```toml
base_url = "http://127.0.0.1:6806"
token = "your-api-token"          # 思源：设置 → 关于 → API token
timeout_secs = 30
workspace_dir = "/path/to/SiYuanKnowledgeBase/data"
```

| 字段 | 说明 |
|------|------|
| `base_url` | 思源 API 地址 |
| `token` | API 令牌（搜索功能需要） |
| `workspace_dir` | 工作空间 `data` 目录，用于直接读取 `.sy` 文件 |

macOS 也可将配置放在 `~/Library/Application Support/scribe/config.toml`。

### 3. 运行

```bash
cargo run -p app
# 或
./target/release/app
```

### 4. 基本操作

| 按键 | 作用 |
|------|------|
| `Space` | 打开搜索框 |
| `Enter` | 打开选中的搜索结果 |
| `Esc` | 关闭搜索框 |
| `↑` `↓` | 搜索列表 / 文档滚动 |
| `PageUp` `PageDown` | 翻页滚动 |
| `Home` `End` | 跳到文档首尾 |
| `Ctrl+C` | 退出 |

## 项目结构

Workspace 采用分层 crate，依赖单向、无循环：

```
app            应用入口、事件循环、终端生命周期
 └─ tui        Compositor、EditorView、SearchBox、主题与图标
     └─ view   DocumentModel 解析、heading / code_block 渲染
         └─ syservice   思源 API、配置、.sy 文件读取、Lute AST
             └─ infrastructure   统一文件日志
```

| Crate | 职责 |
|-------|------|
| `app` | 启动、主循环、Job 调度 |
| `tui` | UI 组合、输入、Buffer / Gutter / 状态栏 |
| `view` | AST → `DocumentLine`，块级布局与样式键 |
| `syservice` | 思源 HTTP 客户端、配置、工作空间文件 IO |
| `infrastructure` | 日志初始化与写入 |

设计文档见 [`docs/`](docs/README.md)。

## 主题与图标

- **`theme.toml`** — 编辑器背景、标题 H1–H6、代码块、Buffer 栏、Gutter 等样式
- **`icons.toml`** — Gutter 块图标（Nerd Font 字符）

修改后重新编译即可生效（主题通过 `include_bytes!` 嵌入二进制）。

## 日志

运行日志写入项目根目录下的 `logs/`（已在 `.gitignore` 中忽略）。

## 版本

当前版本：**0.2.0**

详见 [CHANGELOG.md](CHANGELOG.md)。

设计文档见 [`docs/`](docs/README.md)。

## 开发路线

- [ ] 多笔记并行加载与 Buffer 栏切换
- [ ] 样式与主题进一步统一（代码块主题接入等）
- [ ] 引用块、表格等块类型完善

## 许可证

见仓库 LICENSE 文件（如有）。
