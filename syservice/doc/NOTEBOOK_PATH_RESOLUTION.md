# 笔记本路径解析方案设计

**状态：** 草案  
**创建日期：** 2026-03-31  
**作者：** Scribe Team  
**问题来源：** `load_json_node` 无法解析相对路径

---

## 1. 问题背景

### 1.1 当前问题

`load_json_node(file_path: &str)` 函数在加载文件时，传入的路径是**相对于笔记本的相对路径**，例如：

```
"20230620162729-abc123/20230629142416-def456/20240107160843-ghi789.sy"
```

但 `File::open()` 需要**绝对路径**才能正确加载文件。

### 1.2 问题原因

1. 思源笔记的文档 ID 和路径是**逻辑路径**（相对于笔记本）
2. 文件系统需要**物理路径**（绝对路径）
3. 当前代码缺少**笔记本 ID → 文件系统路径**的映射

### 1.3 错误示例

```rust
// 当前调用方式
let path = "20230620162729-abc123/20230629142416-def456/doc.sy";
let node = load_json_node(path);  // ❌ 失败：相对路径无法打开

// 需要的方式
let path = "/Users/crowds/Notes/SiYuan/data/notebooks/20230620162729-abc123/20230629142416-def456/doc.sy";
let node = load_json_node(path);  // ✅ 成功
```

---

## 2. 解决方案

### 2.1 核心思路

```
相对路径 + 笔记本 ID → 查找笔记本配置 → 拼接绝对路径 → 加载文件
```

### 2.2 架构设计

```
┌─────────────────────────────────────────────────────────┐
│  配置文件 (~/.config/scribe/config.toml)                 │
│  ─────────────────────────────────────────────────────  │
│  [[notebooks]]                                          │
│  id = "20230620162729-abc123"                           │
│  name = "主笔记本"                                       │
│  path = "/Users/crowds/Notes/SiYuan/data/notebooks/..." │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│  NotebookManager (新增)                                  │
│  ─────────────────────────────────────────────────────  │
│  - 加载配置文件中的笔记本列表                            │
│  - 提供 notebook_id → path 的映射查询                    │
│  - 支持动态添加/移除笔记本                               │
└─────────────────────────────────────────────────────────┘
                          ↓
┌─────────────────────────────────────────────────────────┐
│  load_json_node(notebook_id, relative_path)              │
│  ─────────────────────────────────────────────────────  │
│  1. 通过 NotebookManager 获取 notebook 路径              │
│  2. 拼接：notebook_path + "/" + relative_path           │
│  3. 调用 File::open() 加载文件                          │
└─────────────────────────────────────────────────────────┘
```

---

## 3. 详细设计

### 3.1 配置文件格式

在 `~/.config/scribe/config.toml` 中添加笔记本配置：

```toml
# SiYuan 基础配置
base_url = "http://127.0.0.1:6806"
token = "1g4rmbq473pv40jo"
timeout_secs = 30

# 笔记本配置（新增）
[[notebooks]]
id = "20230620162729-abc123"
name = "主笔记本"
path = "/Users/crowds/Notes/SiYuan/data/notebooks/20230620162729-abc123"

[[notebooks]]
id = "20230620162729-def456"
name = "工作笔记"
path = "/Users/crowds/Notes/SiYuan/data/notebooks/20230620162729-def456"
```

### 3.2 NotebookConfig 结构

```rust
/// 笔记本配置
#[derive(Clone, Debug, Deserialize)]
pub struct NotebookConfig {
    /// 笔记本 ID（如 "20230620162729-abc123"）
    pub id: String,
    /// 笔记本名称（如 "主笔记本"）
    pub name: String,
    /// 笔记本在文件系统中的绝对路径
    pub path: String,
}

/// 笔记本管理器
pub struct NotebookManager {
    notebooks: HashMap<String, NotebookConfig>,
}

impl NotebookManager {
    /// 从配置加载笔记本列表
    pub fn from_config() -> Result<Self> {
        // 读取配置文件，解析 [[notebooks]] 部分
    }
    
    /// 根据 notebook ID 获取路径
    pub fn get_notebook_path(&self, notebook_id: &str) -> Option<&str> {
        self.notebooks.get(notebook_id).map(|c| c.path.as_str())
    }
    
    /// 添加笔记本配置
    pub fn add_notebook(&mut self, config: NotebookConfig) {
        self.notebooks.insert(config.id, config);
    }
}
```

### 3.3 更新 load_json_node 签名

```rust
/// 从指定笔记本加载 JSON 节点文件
/// 
/// # Arguments
/// * `notebook_id` - 笔记本 ID（如 "20230620162729-abc123"）
/// * `relative_path` - 相对于笔记本的路径（如 "20230629142416-def456/doc.sy"）
/// * `notebook_manager` - 笔记本管理器（用于查找笔记本路径）
/// 
/// # Returns
/// * `Ok(Node)` - 成功加载的节点
/// * `Err` - 加载失败，错误信息已打印到 stderr
pub fn load_json_node(
    notebook_id: &str,
    relative_path: &str,
    notebook_manager: &NotebookManager,
) -> Result<lute::node::Node> {
    // 1. 获取笔记本路径
    let notebook_path = match notebook_manager.get_notebook_path(notebook_id) {
        Some(path) => path,
        None => {
            eprintln!(
                "❌ Notebook not found: {}\n   \
                 Available notebooks: {:?}",
                notebook_id,
                notebook_manager.list_notebook_ids()
            );
            return Err(anyhow::anyhow!("Notebook not found: {}", notebook_id));
        }
    };
    
    // 2. 拼接绝对路径
    let full_path = Path::new(notebook_path).join(relative_path);
    
    // 3. 加载文件
    load_json_node_from_path(&full_path)
}

/// 内部函数：从绝对路径加载
fn load_json_node_from_path(path: &Path) -> Result<lute::node::Node> {
    // 原有的文件加载逻辑
}
```

### 3.4 获取文档所属笔记本

**问题：** 调用 `load_json_node` 时需要知道 `notebook_id`，但调用方可能只有文档 ID。

**解决方案：**

#### 方案 A：通过思源 API 查询

```rust
/// 查询文档所属的笔记本 ID
pub async fn get_doc_notebook_id(
    client: &HttpClient,
    doc_id: &str,
) -> Result<String> {
    // SQL 查询文档的 notebook_id
    let sql = format!(
        "SELECT notebook_id FROM blocks WHERE id = '{}'",
        doc_id
    );
    
    let result = client.sql_query(&sql).await?;
    
    // 从结果中提取 notebook_id
    result[0]["notebook_id"]
        .as_str()
        .map(String::from)
        .ok_or_else(|| anyhow::anyhow!("Notebook ID not found"))
}
```

#### 方案 B：调用方自行维护映射

调用方（如 `view` 模块）在解析文档时，已经知道文档所属的笔记本，直接传递即可。

**推荐：** 方案 B（避免额外的 API 调用）

---

## 4. 使用示例

### 4.1 初始化配置

```rust
use syservice::config::{Config, NotebookManager};
use syservice::file::load_json_node;

// 1. 加载配置（包含笔记本配置）
let config = Config::load();
let notebook_manager = NotebookManager::from_config()?;

// 2. 加载文档（需要 notebook_id 和相对路径）
let notebook_id = "20230620162729-abc123";
let relative_path = "20230629142416-def456/20240107160843-ghi789.sy";

let node = load_json_node(notebook_id, relative_path, &notebook_manager)?;
```

### 4.2 在 view 模块中的集成

```rust
// view/src/document.rs

use syservice::file::{load_json_node, NotebookManager};

pub struct DocumentModel {
    // ...
    notebook_manager: NotebookManager,
}

impl DocumentModel {
    pub fn open_with_notebook(
        notebook_id: &str,
        doc_path: &str,
        notebook_manager: &NotebookManager,
    ) -> Self {
        // 加载文档
        let node = load_json_node(notebook_id, doc_path, notebook_manager)?;
        
        // 继续处理...
    }
}
```

---

## 5. 配置管理

### 5.1 自动发现笔记本

提供工具函数，自动扫描思源数据目录下的所有笔记本：

```rust
/// 自动扫描思源数据目录下的笔记本
pub fn auto_discover_notebooks(siyuan_data_dir: &str) -> Result<Vec<NotebookConfig>> {
    let notebooks_dir = Path::new(siyuan_data_dir).join("data/notebooks");
    let mut notebooks = Vec::new();
    
    for entry in fs::read_dir(&notebooks_dir)? {
        let entry = entry?;
        let notebook_id = entry.file_name().to_string_lossy().to_string();
        
        // 尝试读取 notebook 配置（如 notebook.json）
        let config_path = entry.path().join("notebook.json");
        let name = if config_path.exists() {
            // 从配置读取名称
            // ...
        } else {
            notebook_id.clone()
        };
        
        notebooks.push(NotebookConfig {
            id: notebook_id,
            name,
            path: entry.path().to_string_lossy().to_string(),
        });
    }
    
    Ok(notebooks)
}
```

### 5.2 配置生成工具

提供 CLI 工具，自动生成笔记本配置：

```bash
# 扫描思源数据目录，生成笔记本配置
scribe config scan-notebooks --siyuan-dir ~/Notes/SiYuan

# 输出：
# 发现 3 个笔记本：
# - 20230620162729-abc123 (主笔记本)
# - 20230620162729-def456 (工作笔记)
# - 20230620162729-ghi789 (个人笔记)
#
# 是否添加到配置文件？[y/N]
```

---

## 6. 错误处理

### 6.1 错误类型

```rust
#[derive(Debug, Error)]
pub enum NotebookError {
    #[error("Notebook not found: {0}")]
    NotFound(String),
    
    #[error("Invalid notebook path: {0}")]
    InvalidPath(String),
    
    #[error("Failed to load notebook config: {0}")]
    ConfigError(String),
}
```

### 6.2 错误输出

```
❌ Notebook not found: 20230620162729-unknown
   Available notebooks:
   - 20230620162729-abc123 (主笔记本)
   - 20230620162729-def456 (工作笔记)

❌ Failed to open file: /path/to/notebook/doc.sy
   Path: /path/to/notebook/doc.sy
   Error: No such file or directory (os error 2)
```

---

## 7. 实现计划

### Phase 1: 基础支持（本周）
- [ ] 添加 `NotebookConfig` 和 `NotebookManager` 结构
- [ ] 更新配置文件格式，支持 `[[notebooks]]`
- [ ] 修改 `load_json_node` 签名，接受 `notebook_id` 参数
- [ ] 更新测试用例

### Phase 2: 自动发现（下周）
- [ ] 实现 `auto_discover_notebooks` 函数
- [ ] 添加 CLI 工具支持
- [ ] 文档更新

### Phase 3: 集成测试（下周）
- [ ] 端到端测试（配置 → 加载 → 渲染）
- [ ] 多笔记本支持测试
- [ ] 错误处理测试

---

## 8. 配置示例

### 完整配置文件

```toml
# SiYuan API 配置
base_url = "http://127.0.0.1:6806"
token = "1g4rmbq473pv40jo"
timeout_secs = 30

# 笔记本配置
[[notebooks]]
id = "20230620162729-abc123"
name = "主笔记本"
path = "/Users/crowds/Notes/SiYuan/data/notebooks/20230620162729-abc123"

[[notebooks]]
id = "20230620162729-def456"
name = "工作笔记"
path = "/Users/crowds/Notes/SiYuan/data/notebooks/20230620162729-def456"

[[notebooks]]
id = "20230620162729-ghi789"
name = "个人笔记"
path = "/Users/crowds/Notes/SiYuan/data/notebooks/20230620162729-ghi789"
```

### 最小配置（无笔记本）

```toml
# 仅 API 配置（向后兼容）
base_url = "http://127.0.0.1:6806"
token = "1g4rmbq473pv40jo"
timeout_secs = 30

# 笔记本配置可选
```

---

## 9. 向后兼容性

### 9.1 现有调用方式

为保持向后兼容，保留旧的 `load_json_node` 签名：

```rust
/// 旧版本（保留，标记为 deprecated）
#[deprecated(since = "0.3.0", note = "Use load_json_node_with_notebook instead")]
pub fn load_json_node(file_path: &str) -> Result<lute::node::Node> {
    // 尝试从当前工作目录加载
    load_json_node_from_path(Path::new(file_path))
}

/// 新版本（推荐）
pub fn load_json_node_with_notebook(
    notebook_id: &str,
    relative_path: &str,
    notebook_manager: &NotebookManager,
) -> Result<lute::node::Node> {
    // ...
}
```

### 9.2 配置迁移

旧配置文件仍然有效，笔记本配置为可选：

```toml
# 旧配置（仍然有效）
base_url = "http://127.0.0.1:6806"
token = "1g4rmbq473pv40jo"

# 新配置（可选）
[[notebooks]]
id = "..."
path = "..."
```

---

## 10. 参考资料

- [思源笔记数据存储结构](https://github.com/siyuan-note/siyuan/blob/master/API.md)
- [TOML 配置格式](https://toml.io/)
- [Rust 路径处理最佳实践](https://doc.rust-lang.org/std/path/)

---

## 11. 修订历史

| 日期 | 版本 | 作者 | 变更 |
|------|------|------|------|
| 2026-03-31 | 0.1 | Scribe Team | 初始草案 |
