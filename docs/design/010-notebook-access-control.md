# 010 — 笔记本访问控制（规划中）

**状态：** 规划中 / 未实现  
**创建日期：** 2026-03-31  
**最后更新：** 2026-06-11

---

## 1. 背景

当前实现使用单一 `workspace_dir`（见 [008 — syservice](008-syservice-module.md)），通过 `box_id` + 相对路径拼接读取 `.sy` 文件。**未**实现多笔记本白名单与路径逃逸防护。

本设计描述未来多笔记本场景下的访问控制方案。

---

## 2. 目标

1. **多笔记本配置** — 在 `config.toml` 中声明多个思源笔记本
2. **白名单访问** — 仅允许读取已启用笔记本下的文件
3. **安全路径解析** — 规范化路径，拒绝 `../` 逃逸
4. **明确错误** — 未授权笔记本或非法路径返回可诊断错误

---

## 3. 拟定配置格式

```toml
base_url = "http://127.0.0.1:6806"
token = "your-api-token-here"
timeout_secs = 30

default_notebook_id = "20230620162729-abc123"

[[notebooks]]
id = "20230620162729-abc123"
name = "Main"
path = "/path/to/SiYuan/data/notebooks/20230620162729-abc123"
enabled = true

[[notebooks]]
id = "20230620162729-def456"
name = "Archive"
path = "/path/to/SiYuan/data/notebooks/20230620162729-def456"
enabled = false
```

> **注意：** 上述 `[[notebooks]]` 段**尚未**被 `Config` 解析；当前仅支持顶层 `workspace_dir`。

---

## 4. 核心组件（拟定）

```rust
pub struct NotebookConfig {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub enabled: bool,
}

pub struct NotebookManager {
    notebooks: HashMap<String, NotebookConfig>,
    default_notebook_id: Option<String>,
    strict_mode: bool,
}

impl NotebookManager {
    /// 将 (notebook_id, relative_path) 解析为绝对路径，或拒绝访问
    pub fn resolve_path(&self, notebook_id: &str, relative: &str) -> Result<PathBuf, AccessError>;
}
```

### 访问规则

| 场景 | 预期 |
|------|------|
| 已启用笔记本 + 合法相对路径 | 返回绝对路径 |
| 未配置 / 已禁用笔记本 | `AccessError::NotebookNotAllowed` |
| 路径含 `..` 或越界 | `AccessError::PathTraversal` |

---

## 5. 与现有代码的集成点

| 位置 | 改动 |
|------|------|
| [`syservice/src/config.rs`](../../syservice/src/config.rs) | 解析 `[[notebooks]]`，可选保留 `workspace_dir` 作默认 |
| [`syservice/src/file.rs`](../../syservice/src/file.rs) | `load_json_node_from_workspace` 经 `NotebookManager` 校验 |
| [`tui/search_box.rs`](../../tui/src/component/search_box.rs) | 搜索结果携带 `box_id`，打开时走统一解析 API |

---

## 6. 实现阶段建议

1. **Phase 1** — `NotebookManager` + 配置解析 + 单元测试（路径规范化）
2. **Phase 2** — `file.rs` / 搜索打开路径统一入口
3. **Phase 3** — 可选 UI：按笔记本过滤搜索、配置校验命令

---

## 7. 关联文档

- [008 — syservice 模块](008-syservice-module.md)
- [003 — 配置系统](003-config-system.md)
