# 思源笔记工作空间配置指南

**创建日期：** 2026-04-01  
**相关模块：** `syservice/src/config.rs`, `syservice/src/file.rs`

---

## 1. 概述

思源笔记的文件存储在特定的数据目录中，例如：
```
/Users/crowds/Notes/SiYuanKnowledgeBase/data/
├── notebooks/
│   ├── 20230620162729-abc123/
│   │   ├── 20230629142416-def456/
│   │   │   └── doc.sy
│   └── 20230620162729-ghi789/
└── assets/
```

为了正确加载这些文件，需要配置**工作空间目录**（workspace_dir）。

---

## 2. 配置方式

### 2.1 配置文件（推荐）

**位置：** `~/.config/scribe/config.toml`

```toml
# SiYuan API 配置
base_url = "http://127.0.0.1:6806"
token = "1g4rmbq473pv40jo"
timeout_secs = 30

# 思源笔记工作空间目录
workspace_dir = "/Users/crowds/Notes/SiYuanKnowledgeBase/data"
```

### 2.2 环境变量

```bash
export SIYUAN_WORKSPACE_DIR="/Users/crowds/Notes/SiYuanKnowledgeBase/data"
```

**优先级：** 环境变量 > 配置文件

---

## 3. 使用示例

### 3.1 基础使用

```rust
use syservice::prelude::*;
use syservice::file::load_json_node_from_workspace;

fn main() -> Result<()> {
    // 1. 加载配置（自动从配置文件或环境变量读取 workspace_dir）
    let config = Config::load();
    
    // 2. 加载文件（传入相对于工作空间的路径）
    let relative_path = "notebooks/20230620162729-abc123/folder/doc.sy";
    let node = load_json_node_from_workspace(relative_path, &config)?;
    
    println!("Loaded document: {:?}", node.id);
    
    Ok(())
}
```

### 3.2 错误处理

```rust
match load_json_node_from_workspace(relative_path, &config) {
    Ok(node) => {
        // 成功加载
    }
    Err(e) => {
        // 错误信息已打印到 stderr
        // 可能原因：
        // - workspace_dir 未配置
        // - 文件不存在
        // - JSON 解析失败
        eprintln!("Error: {}", e);
    }
}
```

### 3.3 在 view 模块中的集成

```rust
// view/src/document.rs

use syservice::config::Config;
use syservice::file::load_json_node_from_workspace;

pub struct DocumentModel {
    config: Config,
    // ...
}

impl DocumentModel {
    pub fn open(notebook_id: &str, doc_path: &str) -> Result<Self> {
        let config = Config::load();
        
        // 拼接相对于工作空间的路径
        let relative_path = format!("notebooks/{}/{}", notebook_id, doc_path);
        
        // 加载文档
        let node = load_json_node_from_workspace(&relative_path, &config)?;
        
        // 继续处理...
        Ok(Self { config, /* ... */ })
    }
}
```

---

## 4. 配置验证

### 4.1 检查配置是否生效

```rust
let config = Config::load();

if let Some(workspace) = &config.workspace_dir {
    println!("✅ Workspace configured: {}", workspace);
} else {
    println!("❌ Workspace not configured");
    println!("   Set SIYUAN_WORKSPACE_DIR or add 'workspace_dir' to config file");
}
```

### 4.2 测试加载

```rust
#[test]
fn test_workspace_load() {
    let config = Config::load();
    
    // 跳过测试如果未配置
    if config.workspace_dir.is_none() {
        eprintln!("Skipping test: workspace_dir not configured");
        return;
    }
    
    let relative_path = "notebooks/20230620162729-abc123/test.sy";
    let result = load_json_node_from_workspace(relative_path, &config);
    
    assert!(result.is_ok(), "Failed to load test file: {:?}", result.err());
}
```

---

## 5. 常见问题

### Q1: workspace_dir 应该指向哪个目录？

**答：** 指向思源笔记数据目录的 `data` 子目录。

典型路径：
- **macOS:** `~/Notes/SiYuanKnowledgeBase/data`
- **Linux:** `~/Notes/SiYuan/data`
- **Windows:** `C:\Users\{user}\Documents\SiYuan\data`

验证方法：该目录下应该包含 `notebooks/` 和 `assets/` 子目录。

### Q2: 相对路径的格式是什么？

**答：** 相对于 `workspace_dir` 的路径。

```
workspace_dir = "/Users/crowds/Notes/SiYuanKnowledgeBase/data"

# 相对路径示例
"notebooks/20230620162729-abc123/doc.sy"
"notebooks/20230620162729-abc123/folder/doc.sy"

# 完整路径（拼接后）
"/Users/crowds/Notes/SiYuanKnowledgeBase/data/notebooks/20230620162729-abc123/doc.sy"
```

### Q3: 未配置 workspace_dir 会怎样？

**答：** `load_json_node_from_workspace` 会返回错误：

```
Error: Workspace directory not configured.
   Set SIYUAN_WORKSPACE_DIR environment variable or add 'workspace_dir' to config file
```

### Q4: 如何迁移旧配置？

**答：** 旧版本使用硬编码路径或直接传入绝对路径。

**旧代码：**
```rust
let node = load_json_node("/absolute/path/to/doc.sy")?;
```

**新代码：**
```rust
let config = Config::load();
let node = load_json_node_from_workspace("notebooks/.../doc.sy", &config)?;
```

---

## 6. 配置示例

### 完整配置文件

```toml
# ============================================
# SiYuan API 配置
# ============================================
base_url = "http://127.0.0.1:6806"
token = "1g4rmbq473pv40jo"
timeout_secs = 30

# ============================================
# 思源笔记工作空间
# ============================================
workspace_dir = "/Users/crowds/Notes/SiYuanKnowledgeBase/data"
```

### 多环境配置

```bash
# 开发环境
export SIYUAN_WORKSPACE_DIR="/Users/crowds/Dev/SiYuan/data"

# 生产环境
export SIYUAN_WORKSPACE_DIR="/Users/crowds/Notes/SiYuanKnowledgeBase/data"
```

---

## 7. API 参考

### Config 结构

```rust
pub struct Config {
    pub base_url: String,
    pub token: String,
    pub timeout_secs: u64,
    pub workspace_dir: Option<String>,  // 新增字段
}
```

### 相关函数

```rust
// 加载配置
pub fn Config::load() -> Config;

// 从工作空间加载文件
pub fn load_json_node_from_workspace(
    relative_path: &str,
    config: &Config,
) -> Result<lute::node::Node>;

// 构建配置
pub fn Config::builder() -> ConfigBuilder;
pub fn ConfigBuilder::workspace_dir(&mut self, dir: &str) -> Self;
```

---

## 8. 测试

### 运行测试

```bash
# 运行文件加载测试
cargo test -p syservice file -- --nocapture

# 运行配置测试
cargo test -p syservice config -- --nocapture
```

### 测试用例

```rust
#[test]
fn test_load_without_workspace_config() {
    let config = Config::default();  // workspace_dir = None
    let result = load_json_node_from_workspace("test.sy", &config);
    
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string()
        .contains("Workspace directory not configured"));
}

#[test]
#[ignore]  // 需要真实的工作空间目录
fn test_load_node_from_workspace() {
    let config = Config::builder()
        .workspace_dir("/path/to/siyuan/data")
        .build();
    
    let relative_path = "notebooks/.../doc.sy";
    let node = load_json_node_from_workspace(relative_path, &config).unwrap();
    
    assert!(node.id.is_some());
}
```

---

## 9. 修订历史

| 日期 | 版本 | 作者 | 变更 |
|------|------|------|------|
| 2026-04-01 | 1.0 | Scribe Team | 初始版本 |

---

## 10. 相关文档

- [配置系统设计](CONFIG_IMPLEMENTATION.md)
- [笔记本访问控制](NOTEBOOK_ACCESS_CONTROL.md)
- [文件加载方案](NOTEBOOK_PATH_RESOLUTION.md)
