# Scribe 项目分析

## 📋 项目概览

**项目名称：** rsy-scribe (Rust SiYuan Scribe)  
**仓库：** https://github.com/Crowds21/rsy-scribe  
**类别：** 终端文本编辑器  
**语言：** Rust  
**版本：** 0.1.0

---

## 🎯 项目目标

**Scribe 是一个基于 SiYuan 笔记的终端编辑器**，允许用户在终端中浏览和编辑 SiYuan 笔记内容。

### 核心功能

1. **SiYuan 笔记集成** - 通过 `syservice` 模块与 SiYuan API 交互
2. **终端 UI** - 使用 `ratatui` (TUI) 构建终端界面
3. **文档渲染** - 将 SiYuan 的块结构渲染为终端可显示的格式
4. **异步任务处理** - 使用 `tokio` 处理后台任务

---

## 🏗️ 项目架构

### 工作区结构

```
scribe/
├── app/           # 应用程序入口
│   ├── src/
│   │   ├── main.rs        # 程序入口
│   │   └── application.rs # 应用主循环
│   └── Cargo.toml
│
├── syservice/     # SiYuan API 客户端（核心服务层）
│   ├── src/
│   │   ├── api.rs         # Trait 定义
│   │   ├── client.rs      # HTTP 客户端实现
│   │   ├── config.rs      # 配置管理
│   │   ├── error.rs       # 错误处理
│   │   ├── domain.rs      # 领域模型
│   │   └── lib.rs         # 库入口
│   └── Cargo.toml
│
├── view/          # 视图层（文档模型）
│   ├── src/
│   │   ├── document.rs    # 文档模型
│   │   ├── editor.rs      # 编辑器状态
│   │   └── lib.rs         # 库入口
│   └── Cargo.toml
│
├── tui/           # TUI 组件层
│   ├── src/
│   │   ├── compositor.rs  # 渲染合成器
│   │   ├── component/     # UI 组件
│   │   ├── uiconfig/      # UI 配置
│   │   └── lib.rs         # 库入口
│   └── Cargo.toml
│
└── Cargo.toml     # 工作区配置
```

---

## 🔧 模块职责

### 1. `app` - 应用程序层

**职责：** 程序入口和主循环

**关键代码：**
```rust
pub struct Application {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    compositor: Compositor,
    editor_model: EditorModel,
    pub jobs: JobQueue,
}
```

**功能：**
- 初始化终端 UI
- 处理事件循环（键盘输入、窗口调整）
- 协调 TUI 渲染和后台任务

---

### 2. `syservice` - SiYuan 服务层

**职责：** SiYuan API 客户端

**关键模块：**
- `api.rs` - Trait 定义（`SiYuanClient`）
- `client.rs` - HTTP 客户端实现（`HttpClient`）
- `config.rs` - 配置管理
- `error.rs` - 错误处理
- `domain.rs` - 领域模型（`SyBlock`, `Notebook`）
- `lute/` - Markdown 解析引擎

**功能：**
- 列出笔记本
- 创建/删除文档
- 块操作（插入、更新、删除）
- SQL 查询
- 属性管理

**API 端点示例：**
```rust
async fn list_notebooks(&self) -> Result<Vec<Notebook>>;
async fn create_doc_with_md(...) -> Result<String>;
async fn sql_query(&self, sql: &str) -> Result<Vec<SyBlock>>;
```

---

### 3. `view` - 视图层

**职责：** 文档模型和编辑器状态

**关键结构：**

#### `EditorModel`
```rust
pub struct EditorModel {
    pub next_document_id: DocumentId,
    pub current_id: Option<DocumentId>,
    pub documents: BTreeMap<DocumentId, DocumentModel>,
    pub view_position: ViewPosition,
}
```

#### `DocumentModel`
```rust
pub struct DocumentModel {
    pub id: DocumentId,
    pub lines: Vec<DocumentLine>,      // 渲染的行
    pub node: Option<Node>,            // SiYuan 节点
    pub max_line_len: u16,             // 最大行长度
}
```

#### `DocumentLine`
```rust
pub struct DocumentLine {
    pub content: Vec<InLineItem>,      // 行内元素
    node_type: NodeType,               // 块类型
    break_line: bool,                  // 软换行
    container: NodeType,               // 容器类型
    indent_width: u16,                 // 缩进
}
```

**功能：**
- 文档加载和解析
- 行内元素样式处理
- 文档状态管理
- 视图偏移管理

---

### 4. `tui` - TUI 组件层

**职责：** 终端 UI 组件和渲染

**关键模块：**
- `compositor.rs` - 渲染合成器
- `component/` - UI 组件（编辑器、搜索框、滚动条等）
- `uiconfig/` - UI 配置（主题、图标、样式）
- `job.rs` - 后台任务队列

**组件：**
- `editor.rs` - 编辑器组件
- `search_box.rs` - 搜索框（带防抖）
- `block.rs` - 块渲染
- `gutter.rs` - 行号区域
- `scroll.rs` - 滚动条

**功能：**
- 终端渲染
- 事件处理
- 组件布局
- 主题管理

---

## 🔄 数据流

```
┌─────────────┐
│   SiYuan    │
│   Server    │
│ (HTTP API)  │
└──────┬──────┘
       │ HTTP
       ▼
┌─────────────┐
│  syservice  │
│  (API 客户端) │
└──────┬──────┘
       │ Rust 对象
       ▼
┌─────────────┐
│    view     │
│ (文档模型)   │
└──────┬──────┘
       │ 渲染数据
       ▼
┌─────────────┐
│     tui     │
│ (TUI 组件)   │
└──────┬──────┘
       │ 终端输出
       ▼
┌─────────────┐
│   Terminal  │
│   (crossterm)│
└─────────────┘
```

---

## 🎨 核心概念

### 1. 文档 ID (`DocumentId`)

```rust
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct DocumentId(pub NonZeroUsize);
```

- 应用内唯一标识文档
- 使用 `NonZeroUsize` 优化内存（`Option<DocumentId>` 只占 1 字节）

### 2. 文档行 (`DocumentLine`)

```rust
pub struct DocumentLine {
    pub content: Vec<InLineItem>,  // 行内元素
    node_type: NodeType,           // 块类型（段落、标题、列表等）
    break_line: bool,              // 软换行
    container: NodeType,           // 容器（引用块、超级块等）
    indent_width: u16,             // 缩进
}
```

### 3. 行内元素 (`InLineItem`)

```rust
pub struct InLineItem {
    pub item_type: InLineMarkType,  // 样式类型（粗体、斜体、代码等）
    pub content: String,            // 内容
    pub link: Option<String>,       // 链接
    pub style: Option<String>,      // 样式
}
```

### 4. 块类型 (`NodeType`)

```rust
pub enum NodeType {
    Document,
    Blockquote,
    List,
    ListItem,
    CodeBlock,
    Paragraph,
    Heading { level: u8 },
    // ...
}
```

---

## 🚀 技术栈

### 核心依赖

| 库 | 用途 |
|------|------|
| `tokio` | 异步运行时 |
| `ratatui` | TUI 框架 |
| `crossterm` | 终端操作 |
| `serde` / `serde_json` | JSON 序列化 |
| `reqwest` | HTTP 客户端 |
| `thiserror` / `anyhow` | 错误处理 |
| `strum` | 枚举工具 |

### SiYuan 集成

| 模块 | 用途 |
|------|------|
| `syservice` | SiYuan API 客户端 |
| `lute` | Markdown 解析引擎（来自 SiYuan） |

---

## 📝 TODO 列表

根据 README.md：

### 待完成功能

1. **优化文档渲染**
   - 调整为只有首次加载文档时才进行渲染
   - 避免重复渲染提高性能

2. **终端调整处理**
   - 终端窗口大小改变时需要重新计算行长度
   - 保持视图状态

3. **Layer 叠加问题**
   - 不同层级的 Layer 叠加展示存在问题
   - 需要修复渲染顺序

### CICD

- 通过 git action 自动生成 ChangeLog
  ```bash
  git cliff -o CHANGELOG.md
  ```

---

## 🔍 代码统计

| 模块 | 文件数 | 代码行数（估算） |
|------|--------|-----------------|
| `app` | 2 | ~100 |
| `syservice` | ~10 | ~2000+ |
| `view` | ~5 | ~600+ |
| `tui` | ~15 | ~1000+ |
| **总计** | **~32** | **~3700+** |

---

## 🎯 项目定位

**Scribe 是一个实验性的 SiYuan 笔记终端客户端**，主要特点：

1. **轻量级** - 纯终端界面，资源占用低
2. **快速** - Rust 编译，性能优秀
3. **可扩展** - 模块化设计，易于添加功能
4. **SiYuan 原生** - 直接调用 SiYuan API，保持数据同步

### 适用场景

- 喜欢在终端工作的用户
- 需要快速浏览和编辑笔记
- 远程服务器环境（SSH）
- 低资源环境

---

## 📚 相关项目

### 参考项目

- **helix** - Rust 编写的现代编辑器（参考了文档 ID 设计）
- **kakoune** - 模态编辑器
- **SiYuan** - 思源笔记（后端服务）

### 依赖库

- **ratatui** - Rust TUI 库（原 tui-rs 的分支）
- **crossterm** - 跨平台终端操作库

---

## 🏁 总结

**Scribe = SiYuan + Terminal Editor**

这是一个将 SiYuan 笔记带入终端的创新项目，通过 Rust 的性能和安全性，提供了一个轻量级但功能完整的笔记编辑体验。

**核心价值：**
- ✅ 终端用户的 SiYuan 客户端
- ✅ 模块化、可扩展的架构
- ✅ 异步、高性能的实现
- ✅ 与 SiYuan API 深度集成

**当前状态：** 开发中（v0.1.0）

---

**分析时间：** 2026-03-27  
**分析师：** OpenClaw Agent
