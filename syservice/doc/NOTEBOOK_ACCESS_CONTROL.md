# 笔记本访问控制与路径管理完整方案

**状态：** 设计评审  
**创建日期：** 2026-03-31  
**作者：** Scribe Team  
**安全级别：** 重要（文件系统访问控制）

---

## 1. 需求说明

### 1.1 功能需求

1. **多笔记本支持** - 支持配置多个思源笔记本
2. **访问控制** - 只能访问配置中的笔记本，禁止访问未授权目录
3. **路径解析** - 将相对路径转换为绝对路径
4. **配置管理** - 支持配置文件的加载、验证和更新

### 1.2 安全需求

1. **白名单机制** - 只有配置中的笔记本才能访问
2. **路径限制** - 禁止通过 `../` 等方式逃逸笔记本目录
3. **错误提示** - 访问未授权笔记本时给出明确错误信息

### 1.3 使用场景

```rust
// 场景 1：加载已知笔记本中的文档
let node = load_json_node("notebook-001", "path/to/doc.sy")?;

// 场景 2：尝试访问未配置的笔记本（应拒绝）
let node = load_json_node("unknown-notebook", "doc.sy");  // ❌ 拒绝

// 场景 3：尝试路径逃逸（应拒绝）
let node = load_json_node("notebook-001", "../../etc/passwd");  // ❌ 拒绝
```

---

## 2. 完整设计

### 2.1 配置文件格式

**位置：** `~/.config/scribe/config.toml`

```toml
# ============================================
# SiYuan API 配置
# ============================================
base_url = "http://127.0.0.1:6806"
token = "1g4rmbq473pv40jo"
timeout_secs = 30

# ============================================
# 笔记本配置（访问控制白名单）
# ============================================

# 默认笔记本（可选，用于简化调用）
default_notebook_id = "20230620162729-abc123"

# 笔记本列表
[[notebooks]]
id = "20230620162729-abc123"
name = "主笔记本"
path = "/Users/crowds/Notes/SiYuan/data/notebooks/20230620162729-abc123"
enabled = true
description = "日常笔记和文档"

[[notebooks]]
id = "20230620162729-def456"
name = "工作笔记"
path = "/Users/crowds/Notes/SiYuan/data/notebooks/20230620162729-def456"
enabled = true
description = "工作相关文档"

[[notebooks]]
id = "20230620162729-ghi789"
name = "归档笔记"
path = "/Users/crowds/Notes/SiYuan/data/notebooks/20230620162729-ghi789"
enabled = false  # 已禁用，无法访问
description = "历史归档"
```

### 2.2 数据结构

```rust
/// 笔记本配置
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NotebookConfig {
    /// 笔记本唯一 ID（思源笔记生成）
    pub id: String,
    /// 笔记本显示名称
    pub name: String,
    /// 笔记本在文件系统中的绝对路径
    pub path: String,
    /// 是否启用（禁用的笔记本无法访问）
    #[serde(default = "default_true")]
    pub enabled: bool,
    /// 可选描述
    #[serde(default)]
    pub description: String,
}

fn default_true() -> bool { true }

/// 笔记本管理器（访问控制核心）
#[derive(Clone)]
pub struct NotebookManager {
    /// 所有配置的笔记本
    notebooks: HashMap<String, NotebookConfig>,
    /// 默认笔记本 ID（可选）
    default_notebook_id: Option<String>,
    /// 是否严格模式（严格模式下禁止任何未配置路径）
    strict_mode: bool,
}

/// 笔记本访问结果
#[derive(Debug)]
pub enum NotebookAccessResult {
    /// 访问成功
    Allowed {
        notebook: NotebookConfig,
        full_path: PathBuf,
    },
    /// 笔记本未配置
    NotebookNotFound {
        notebook_id: String,
    },
    /// 笔记本已禁用
    NotebookDisabled {
        notebook_id: String,
    },
    /// 路径非法（尝试逃逸）
    InvalidPath {
        reason: String,
    },
}
```

### 2.3 核心 API

```rust
impl NotebookManager {
    /// 从配置文件加载笔记本管理器
    /// 
    /// # Returns
    /// * `Ok(NotebookManager)` - 成功加载
    /// * `Err` - 配置文件解析失败或路径验证失败
    pub fn from_config() -> Result<Self> {
        let config_path = get_config_path()
            .ok_or_else(|| anyhow!("Config path not found"))?;
        
        Self::from_config_file(&config_path)
    }
    
    /// 从指定配置文件加载
    pub fn from_config_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config: {}", path.display()))?;
        
        Self::from_toml(&content)
    }
    
    /// 从 TOML 字符串解析
    pub fn from_toml(content: &str) -> Result<Self> {
        let config: ConfigFile = toml::from_str(content)
            .map_err(|e| anyhow!("Failed to parse config: {}", e))?;
        
        // 验证所有笔记本路径
        let mut notebooks = HashMap::new();
        for nb_config in config.notebooks {
            // 验证路径存在
            if !Path::new(&nb_config.path).exists() {
                eprintln!(
                    "⚠️  Warning: Notebook path does not exist: {} ({})",
                    nb_config.name,
                    nb_config.path
                );
            }
            
            // 验证路径是绝对路径
            if !Path::new(&nb_config.path).is_absolute() {
                return Err(anyhow!(
                    "Notebook path must be absolute: {} ({})",
                    nb_config.name,
                    nb_config.path
                ));
            }
            
            notebooks.insert(nb_config.id.clone(), nb_config);
        }
        
        Ok(Self {
            notebooks,
            default_notebook_id: config.default_notebook_id,
            strict_mode: true,  // 默认严格模式
        })
    }
    
    /// 验证并获取笔记本的完整路径
    /// 
    /// # Security
    /// 此方法会进行以下安全检查：
    /// 1. 笔记本是否在白名单中
    /// 2. 笔记本是否启用
    /// 3. 相对路径是否尝试逃逸（包含 `..`）
    /// 4. 最终路径是否仍在笔记本目录内
    pub fn resolve_path(
        &self,
        notebook_id: &str,
        relative_path: &str,
    ) -> NotebookAccessResult {
        // 检查 1: 笔记本是否存在
        let notebook = match self.notebooks.get(notebook_id) {
            Some(nb) => nb,
            None => {
                return NotebookAccessResult::NotebookNotFound {
                    notebook_id: notebook_id.to_string(),
                };
            }
        };
        
        // 检查 2: 笔记本是否启用
        if !notebook.enabled {
            return NotebookAccessResult::NotebookDisabled {
                notebook_id: notebook_id.to_string(),
            };
        }
        
        // 检查 3: 相对路径是否合法（不包含路径逃逸）
        if relative_path.contains("..") {
            return NotebookAccessResult::InvalidPath {
                reason: "Relative path contains '..' which is not allowed".to_string(),
            };
        }
        
        // 检查 4: 拼接路径并验证
        let notebook_path = Path::new(&notebook.path);
        let full_path = notebook_path.join(relative_path);
        
        // 规范化路径并验证是否在笔记本目录内
        match full_path.canonicalize() {
            Ok(canonical) => {
                // 确保规范化后的路径仍在笔记本目录内
                let canonical_notebook = notebook_path.canonicalize().ok();
                if let Some(nb_canonical) = canonical_notebook {
                    if !canonical.starts_with(&nb_canonical) {
                        return NotebookAccessResult::InvalidPath {
                            reason: "Resolved path is outside notebook directory".to_string(),
                        };
                    }
                }
                
                NotebookAccessResult::Allowed {
                    notebook: notebook.clone(),
                    full_path: canonical,
                }
            }
            Err(_) => {
                // 文件不存在，但路径格式合法
                // 在严格模式下拒绝，宽松模式下允许（用于创建新文件）
                if self.strict_mode {
                    NotebookAccessResult::InvalidPath {
                        reason: "File does not exist".to_string(),
                    }
                } else {
                    NotebookAccessResult::Allowed {
                        notebook: notebook.clone(),
                        full_path,
                    }
                }
            }
        }
    }
    
    /// 获取所有启用的笔记本列表
    pub fn list_enabled_notebooks(&self) -> Vec<&NotebookConfig> {
        self.notebooks
            .values()
            .filter(|nb| nb.enabled)
            .collect()
    }
    
    /// 获取所有笔记本 ID
    pub fn list_notebook_ids(&self) -> Vec<&str> {
        self.notebooks.keys().map(|s| s.as_str()).collect()
    }
    
    /// 设置默认笔记本
    pub fn set_default_notebook(&mut self, notebook_id: &str) -> Result<()> {
        if !self.notebooks.contains_key(notebook_id) {
            return Err(anyhow!("Notebook not found: {}", notebook_id));
        }
        self.default_notebook_id = Some(notebook_id.to_string());
        Ok(())
    }
    
    /// 获取默认笔记本 ID
    pub fn default_notebook_id(&self) -> Option<&str> {
        self.default_notebook_id.as_deref()
    }
}
```

### 2.4 文件加载函数

```rust
/// 从指定笔记本加载 JSON 节点文件
/// 
/// # Security
/// 此函数会进行完整的访问控制检查：
/// 1. 验证笔记本是否在白名单中
/// 2. 验证笔记本是否启用
/// 3. 验证路径不包含逃逸序列
/// 4. 验证最终路径在笔记本目录内
/// 
/// # Arguments
/// * `notebook_id` - 笔记本 ID
/// * `relative_path` - 相对于笔记本的路径（如 "folder/doc.sy"）
/// * `notebook_manager` - 笔记本管理器
/// 
/// # Returns
/// * `Ok(Node)` - 成功加载
/// * `Err` - 加载失败（包含详细错误信息）
/// 
/// # Example
/// ```rust,ignore
/// let manager = NotebookManager::from_config()?;
/// let node = load_json_node(
///     "20230620162729-abc123",
///     "folder/doc.sy",
///     &manager
/// )?;
/// ```
pub fn load_json_node(
    notebook_id: &str,
    relative_path: &str,
    notebook_manager: &NotebookManager,
) -> Result<lute::node::Node> {
    // 1. 验证并解析路径
    match notebook_manager.resolve_path(notebook_id, relative_path) {
        NotebookAccessResult::Allowed { full_path, .. } => {
            // 2. 加载文件
            load_json_node_from_path(&full_path)
        }
        NotebookAccessResult::NotebookNotFound { notebook_id } => {
            eprintln!(
                "❌ Notebook not found: {}\n   \
                 Available notebooks: {:?}",
                notebook_id,
                notebook_manager.list_notebook_ids()
            );
            Err(anyhow!("Notebook not found: {}", notebook_id))
        }
        NotebookAccessResult::NotebookDisabled { notebook_id } => {
            eprintln!(
                "❌ Notebook is disabled: {}\n   \
                 Enable it in config file to access",
                notebook_id
            );
            Err(anyhow!("Notebook is disabled: {}", notebook_id))
        }
        NotebookAccessResult::InvalidPath { reason } => {
            eprintln!(
                "❌ Invalid path: {}\n   \
                 Notebook: {}\n   \
                 Relative path: {}\n   \
                 Reason: {}",
                relative_path,
                notebook_id,
                relative_path,
                reason
            );
            Err(anyhow!("Invalid path: {}", reason))
        }
    }
}

/// 内部函数：从绝对路径加载文件
fn load_json_node_from_path(path: &Path) -> Result<lute::node::Node> {
    // 打开文件
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!(
                "❌ Failed to open file: {}\n   \
                 Error: {}",
                path.display(),
                e
            );
            return Err(e).with_context(|| format!("Failed to open file: {}", path.display()));
        }
    };

    // 创建带缓冲的读取器
    let reader = BufReader::with_capacity(1024 * 1024, file);

    // 反序列化
    match serde_json::from_reader(reader) {
        Ok(node) => Ok(node),
        Err(e) => {
            eprintln!(
                "❌ Failed to parse JSON: {}\n   \
                 Error: {}",
                path.display(),
                e
            );
            Err(anyhow::Error::new(e)
                .context(format!("Failed to parse JSON: {}", path.display())))
        }
    }
}

/// 便捷函数：使用默认笔记本加载
/// 
/// # Arguments
/// * `relative_path` - 相对于默认笔记本的路径
/// * `notebook_manager` - 笔记本管理器
/// 
/// # Returns
/// * `Ok(Node)` - 成功加载
/// * `Err` - 加载失败
pub fn load_json_node_default(
    relative_path: &str,
    notebook_manager: &NotebookManager,
) -> Result<lute::node::Node> {
    let notebook_id = notebook_manager.default_notebook_id()
        .ok_or_else(|| anyhow!("No default notebook configured"))?;
    
    load_json_node(notebook_id, relative_path, notebook_manager)
}
```

---

## 3. 配置管理工具

### 3.1 添加笔记本

```rust
/// 添加笔记本配置
/// 
/// # Arguments
/// * `notebook_id` - 笔记本 ID
/// * `path` - 笔记本路径（绝对路径）
/// * `name` - 笔记本名称（可选）
pub fn add_notebook_config(
    notebook_id: &str,
    path: &str,
    name: Option<&str>,
) -> Result<()> {
    let config_path = get_config_path()
        .ok_or_else(|| anyhow!("Config path not found"))?;
    
    // 验证路径
    let path = Path::new(path);
    if !path.is_absolute() {
        return Err(anyhow!("Notebook path must be absolute"));
    }
    
    if !path.exists() {
        return Err(anyhow!("Notebook path does not exist: {}", path.display()));
    }
    
    // 加载现有配置
    let mut manager = NotebookManager::from_config_file(&config_path)?;
    
    // 添加笔记本
    manager.notebooks.insert(
        notebook_id.to_string(),
        NotebookConfig {
            id: notebook_id.to_string(),
            name: name.unwrap_or(notebook_id).to_string(),
            path: path.to_string_lossy().to_string(),
            enabled: true,
            description: String::new(),
        },
    );
    
    // 保存配置
    save_config(&config_path, &manager)?;
    
    Ok(())
}
```

### 3.2 列出笔记本

```rust
/// 列出所有配置的笔记本
pub fn list_notebooks() -> Result<()> {
    let manager = NotebookManager::from_config()?;
    
    println!("📚 Configured Notebooks:");
    println!("{}", "=".repeat(60));
    
    for nb in manager.list_enabled_notebooks() {
        let default_marker = if manager.default_notebook_id() == Some(&nb.id) {
            " (default)"
        } else {
            ""
        };
        
        println!(
            "  ID:      {}\n  Name:    {}{}\n  Path:    {}\n  Status:  ✅ Enabled",
            nb.id,
            nb.name,
            default_marker,
            nb.path
        );
        println!("{}", "-".repeat(60));
    }
    
    // 显示禁用的笔记本
    let disabled: Vec<_> = manager.notebooks.values().filter(|nb| !nb.enabled).collect();
    if !disabled.is_empty() {
        println!("\n⏸️  Disabled Notebooks:");
        for nb in disabled {
            println!(
                "  ID:      {}\n  Name:    {}\n  Path:    {}\n  Status:  ❌ Disabled",
                nb.id, nb.name, nb.path
            );
        }
    }
    
    Ok(())
}
```

### 3.3 扫描思源数据目录

```rust
/// 自动扫描思源数据目录，发现所有笔记本
/// 
/// # Arguments
/// * `siyuan_data_dir` - 思源数据目录（如 ~/Notes/SiYuan）
pub fn scan_siyuan_notebooks(siyuan_data_dir: &str) -> Result<Vec<NotebookConfig>> {
    let notebooks_dir = Path::new(siyuan_data_dir).join("data/notebooks");
    
    if !notebooks_dir.exists() {
        return Err(anyhow!("Notebooks directory not found: {}", notebooks_dir.display()));
    }
    
    let mut notebooks = Vec::new();
    
    for entry in fs::read_dir(&notebooks_dir)? {
        let entry = entry?;
        let notebook_id = entry.file_name().to_string_lossy().to_string();
        
        // 尝试读取 notebook.json 获取名称
        let config_path = entry.path().join("notebook.json");
        let name = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: serde_json::Value = serde_json::from_str(&content)?;
            config["name"].as_str().map(String::from).unwrap_or_else(|| notebook_id.clone())
        } else {
            notebook_id.clone()
        };
        
        notebooks.push(NotebookConfig {
            id: notebook_id,
            name,
            path: entry.path().to_string_lossy().to_string(),
            enabled: true,
            description: String::new(),
        });
    }
    
    Ok(notebooks)
}
```

---

## 4. 使用示例

### 4.1 基础使用

```rust
use syservice::config::{Config, NotebookManager};
use syservice::file::{load_json_node, load_json_node_default};

fn main() -> Result<()> {
    // 1. 加载配置（包含笔记本白名单）
    let config = Config::load();
    let notebook_manager = NotebookManager::from_config()?;
    
    // 2. 方式 A：指定笔记本加载
    let node = load_json_node(
        "20230620162729-abc123",  // 笔记本 ID
        "folder/doc.sy",           // 相对路径
        &notebook_manager,
    )?;
    
    // 3. 方式 B：使用默认笔记本加载
    let node = load_json_node_default(
        "folder/doc.sy",
        &notebook_manager,
    )?;
    
    Ok(())
}
```

### 4.2 错误处理

```rust
match load_json_node("unknown-id", "doc.sy", &manager) {
    Ok(node) => {
        // 成功
    }
    Err(e) => {
        // 错误信息已打印到 stderr
        // 可以根据需要进一步处理
        eprintln!("Error: {}", e);
    }
}
```

### 4.3 在 view 模块中的集成

```rust
// view/src/document.rs

use syservice::file::{load_json_node, NotebookManager};

pub struct DocumentModel {
    // ...
    notebook_manager: Rc<NotebookManager>,  // 共享管理器
    current_notebook_id: String,
}

impl DocumentModel {
    pub fn open(
        notebook_id: &str,
        doc_path: &str,
        notebook_manager: Rc<NotebookManager>,
    ) -> Result<Self> {
        // 加载文档
        let node = load_json_node(notebook_id, doc_path, &notebook_manager)?;
        
        Ok(Self {
            notebook_manager,
            current_notebook_id: notebook_id.to_string(),
            // ...
        })
    }
}
```

---

## 5. 安全考虑

### 5.1 路径逃逸防护

```rust
// ❌ 拒绝：包含 ..
load_json_node("nb-001", "../../etc/passwd", &manager);
// Error: Invalid path: Relative path contains '..'

// ❌ 拒绝：解析后超出笔记本目录
load_json_node("nb-001", "subdir/../../../etc/passwd", &manager);
// Error: Invalid path: Resolved path is outside notebook directory

// ✅ 允许：合法路径
load_json_node("nb-001", "folder/doc.sy", &manager);
```

### 5.2 白名单机制

```rust
// ❌ 拒绝：未配置的笔记本
load_json_node("unknown-nb", "doc.sy", &manager);
// Error: Notebook not found: unknown-nb

// ✅ 允许：配置中的笔记本
load_json_node("nb-001", "doc.sy", &manager);
```

### 5.3 启用/禁用控制

```toml
# 配置中设置 enabled = false
[[notebooks]]
id = "nb-001"
path = "/path/to/notebook"
enabled = false  # 禁用
```

```rust
// ❌ 拒绝：禁用的笔记本
load_json_node("nb-001", "doc.sy", &manager);
// Error: Notebook is disabled: nb-001
```

---

## 6. 测试用例

### 6.1 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_notebook_manager_from_config() {
        let config = r#"
            [[notebooks]]
            id = "nb-001"
            name = "Test Notebook"
            path = "/tmp/test-nb"
            enabled = true
        "#;
        
        let manager = NotebookManager::from_toml(config).unwrap();
        assert_eq!(manager.notebooks.len(), 1);
        assert!(manager.notebooks.contains_key("nb-001"));
    }
    
    #[test]
    fn test_resolve_path_valid() {
        let config = r#"
            [[notebooks]]
            id = "nb-001"
            name = "Test"
            path = "/tmp/test-nb"
        "#;
        
        let manager = NotebookManager::from_toml(config).unwrap();
        
        // 创建测试目录
        fs::create_dir_all("/tmp/test-nb/folder").unwrap();
        
        let result = manager.resolve_path("nb-001", "folder/doc.sy");
        assert!(matches!(result, NotebookAccessResult::Allowed { .. }));
    }
    
    #[test]
    fn test_resolve_path_escape() {
        let config = r#"
            [[notebooks]]
            id = "nb-001"
            name = "Test"
            path = "/tmp/test-nb"
        "#;
        
        let manager = NotebookManager::from_toml(config).unwrap();
        
        let result = manager.resolve_path("nb-001", "../etc/passwd");
        assert!(matches!(result, NotebookAccessResult::InvalidPath { .. }));
    }
    
    #[test]
    fn test_resolve_path_not_found() {
        let config = r#"
            [[notebooks]]
            id = "nb-001"
            name = "Test"
            path = "/tmp/test-nb"
        "#;
        
        let manager = NotebookManager::from_toml(config).unwrap();
        
        let result = manager.resolve_path("unknown-nb", "doc.sy");
        assert!(matches!(result, NotebookAccessResult::NotebookNotFound { .. }));
    }
    
    #[test]
    fn test_resolve_path_disabled() {
        let config = r#"
            [[notebooks]]
            id = "nb-001"
            name = "Test"
            path = "/tmp/test-nb"
            enabled = false
        "#;
        
        let manager = NotebookManager::from_toml(config).unwrap();
        
        let result = manager.resolve_path("nb-001", "doc.sy");
        assert!(matches!(result, NotebookAccessResult::NotebookDisabled { .. }));
    }
}
```

---

## 7. 实现计划

### Phase 1: 核心功能（本周）
- [ ] 实现 `NotebookConfig` 和 `NotebookManager`
- [ ] 更新配置文件格式
- [ ] 实现 `resolve_path` 方法（含安全检查）
- [ ] 更新 `load_json_node` 函数
- [ ] 编写单元测试

### Phase 2: 配置管理工具（下周）
- [ ] 实现 `add_notebook_config`
- [ ] 实现 `list_notebooks`
- [ ] 实现 `scan_siyuan_notebooks`
- [ ] CLI 工具集成

### Phase 3: 集成测试（下周）
- [ ] 端到端测试
- [ ] 安全测试（路径逃逸、未授权访问）
- [ ] 性能测试（大量笔记本）

---

## 8. 配置示例

### 完整配置

```toml
# SiYuan API 配置
base_url = "http://127.0.0.1:6806"
token = "1g4rmbq473pv40jo"
timeout_secs = 30

# 默认笔记本
default_notebook_id = "20230620162729-abc123"

# 笔记本列表
[[notebooks]]
id = "20230620162729-abc123"
name = "主笔记本"
path = "/Users/crowds/Notes/SiYuan/data/notebooks/20230620162729-abc123"
enabled = true
description = "日常笔记"

[[notebooks]]
id = "20230620162729-def456"
name = "工作笔记"
path = "/Users/crowds/Notes/SiYuan/data/notebooks/20230620162729-def456"
enabled = true
description = "工作文档"
```

### 最小配置

```toml
# 仅 API 配置（无笔记本，向后兼容）
base_url = "http://127.0.0.1:6806"
token = "1g4rmbq473pv40jo"
```

---

## 9. 修订历史

| 日期 | 版本 | 作者 | 变更 |
|------|------|------|------|
| 2026-03-31 | 0.1 | Scribe Team | 初始完整设计 |
