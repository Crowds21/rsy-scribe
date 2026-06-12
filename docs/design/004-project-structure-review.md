# 项目结构审查与优化建议

**状态：** 滚动已修复（2026-06-12）  
**创建日期：** 2026-06-12  
**范围：** 基于当前已实现功能的务实审查（避免过度设计）

---

## 1. 整体评价

五层 crate 划分清晰，依赖单向、无循环：

```
app → tui → view → syservice → infrastructure
```

主链路已跑通：搜索 → 读 `.sy` → `DocumentModel` 解析 → `EditorView` 渲染。`Compositor` + `dispatch()` 的异步回 UI 模式合适。

| 方面 | 说明 |
|------|------|
| Compositor | 层叠组件、SearchBox push/pop |
| view 文档解析 | AST → `DocumentLine`，宽度换行、代码块子模块 |
| syservice HTTP | `HttpClient` + trait + 中间件 |
| 配置与日志 | 配置文件 + `infrastructure` 文件日志 |
| 主题 | `theme.toml` + palette 回退 |

---

## 2. 建议优先处理

### P0 — 明确是 bug

**滚动高度算错** — `get_current_doc_height` 误用 `next_document_id`。**已修复：** 改用 `current_id`。

**`scroll` 未持久化** — 每帧 `scroll: None`。**已修复：** `Compositor.scroll` 持久化，事件后写回。

**`cursor_position()` 为 `todo!()`** — **已修复：** 返回 `None`，阅读模式不显示块光标。

### P1 — 结构冗余 / 未接线

**两套文档渲染 pipeline** — 在用的是 `view/document.rs`；`tui/src/component/block/doc.rs` 仅测试引用，建议删除以免漂移。

**代码块主题未接入** — `create_code_block_lines` 传入 `HashMap::new()`，应传入 `Theme.styles`。

**`syservice/document.rs` 绕过 HttpClient** — `search_doc_with_title` 裸 reqwest + SQL 拼接；`create_doc_with_md` 为 dead code。应迁到 `HttpClient`。

### P2 — 清理即可

- ~~未使用依赖：`ropey`、`slotmap`~~ **已移除**
- ~~空/死模块：`view/src/config.rs`、`tui/mode.rs`、`commands.rs`、`block/doc` 等~~ **已移除**
- Gutter 行数写死、icons 中 `log::warn!` 与 infrastructure 不统一（待办）

---

## 3. 建议暂缓

| 想法 | 原因 |
|------|------|
| view 与 TUI 完全解耦 | 已依赖 ratatui，改造成本高 |
| 合并 infrastructure | 仅日志，独立 crate 可接受 |
| 大改 scroll 架构 | 先持久化 scroll、修 doc height |

---

## 4. 建议执行顺序

1. 修 scroll 持久化 + `get_current_doc_height` + 上下键滚动
2. 代码块传入 theme.styles
3. 删除 `block/doc` 死代码 + 未使用依赖
4. search 迁到 HttpClient
5. 零散清理

---

## 5. 文件索引

|  Concern | 文件 |
|---------|------|
| 入口 | `app/src/main.rs`, `app/src/application.rs` |
| 滚动 | `tui/src/compositor.rs`, `tui/src/component/editor.rs` |
| 文档模型 | `view/src/editor.rs`, `view/src/document.rs` |
| 搜索打开 | `tui/src/component/search_box.rs` |
| 配置 | `syservice/src/config.rs`, `theme.toml`, `icons.toml` |
| 遗留渲染 | `tui/src/component/block/doc.rs`（未使用） |
