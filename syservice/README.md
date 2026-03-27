# SiYuan Service - SiYuan 笔记 API 客户端

本模块提供了对 SiYuan 笔记系统的完整 API 封装，支持所有常用操作。

## 📦 功能特性

### ✅ 已实现的功能

| 类别 | 功能 | 方法 |
|------|------|------|
| **笔记本** | 列出所有笔记本 | `list_notebooks()` |
| | 创建笔记本 | `create_notebook(name)` |
| | 删除笔记本 | `remove_notebook(id)` |
| **文档** | 创建文档（Markdown） | `create_doc_with_md(notebook, path, markdown)` |
| | 删除文档 | `remove_doc(doc_id)` |
| | 重命名文档 | `rename_doc(notebook, path, title)` |
| **块操作** | 插入块 | `insert_block(data_type, data, previous_id, next_id)` |
| | 追加块 | `append_block(parent_id, data_type, data)` |
| | 更新块 | `update_block(block_id, data_type, data)` |
| | 删除块 | `delete_block(block_id)` |
| | 移动块 | `move_block(block_id, previous_id, next_id)` |
| **查询** | SQL 查询 | `sql_query(sql)` |
| | 按标题搜索 | `search_by_title(title, limit)` |
| **属性** | 获取块属性 | `get_block_attrs(block_id)` |
| | 设置块属性 | `set_block_attrs(block_id, attrs)` |
| | 设置单个属性 | `set_block_attr(block_id, key, value)` |
| **系统** | 获取版本 | `get_version()` |
| | 检查连通性 | `ping()` |

---

## 🚀 快速开始

### 1. 基本使用

```rust
use syservice::service::SiYuanService;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建服务实例
    let service = SiYuanService::new(
        "http://127.0.0.1:6806",  // SiYuan API 地址
        "your_api_token"          // API Token
    );

    // 检查连通性
    if service.ping().await {
        println!("✓ SiYuan service is reachable");
    }

    // 列出所有笔记本
    let notebooks = service.list_notebooks().await?;
    for notebook in notebooks {
        println!("- {} ({})", notebook.name, notebook.id);
    }

    Ok(())
}
```

### 2. 创建文档

```rust
let service = SiYuanService::new("http://127.0.0.1:6806", "token");

// 在指定笔记本中创建文档
let doc_id = service
    .create_doc_with_md(
        "20260327132532-f145gxh",  // 笔记本 ID
        "/daily/2026-03-27",        // 文档路径
        "# 每日笔记\n\n今天是..."    // Markdown 内容
    )
    .await?;

println!("Created document: {}", doc_id);
```

### 3. 查询操作

```rust
// SQL 查询
let blocks = service
    .sql_query("SELECT * FROM blocks WHERE type='d' LIMIT 10")
    .await?;

// 按标题搜索
let docs = service.search_by_title("OpenClaw", 5).await?;
for doc in docs {
    println!("- {}", doc.content);
}
```

### 4. 块操作

```rust
// 追加块到文档
let block_ids = service
    .append_block(
        "parent_block_id",
        "markdown",
        "- 新的列表项"
    )
    .await?;

// 更新块内容
service
    .update_block("block_id", "markdown", "更新后的内容")
    .await?;

// 删除块
service.delete_block("block_id").await?;
```

### 5. 属性操作

```rust
use std::collections::HashMap;

let block_id = "20260327141216-y6pfwyk";

// 获取属性
let attrs = service.get_block_attrs(block_id).await?;

// 设置单个属性（自定义属性需以 custom- 为前缀）
service
    .set_block_attr(block_id, "custom-status", "completed")
    .await?;

// 设置多个属性
let mut attrs = HashMap::new();
attrs.insert("custom-priority".to_string(), "high".to_string());
attrs.insert("custom-tags".to_string(), "work,urgent".to_string());
service.set_block_attrs(block_id, attrs).await?;
```

---

## 🧪 运行测试

### 单元测试（无需 SiYuan 实例）

```bash
cd syservice
cargo test --lib service::tests::test_service_creation
cargo test --lib service::tests::test_notebook_serialization
```

### 集成测试（需要 SiYuan 实例）

```bash
# 运行所有集成测试
cargo test --test integration -- --ignored

# 运行单个测试
cargo test --test integration test_full_workflow -- --ignored --nocapture
```

---

## 📝 配置说明

### API Token 获取

1. 打开 SiYuan 笔记
2. 进入 **设置 > 关于**
3. 复制 **API Token**

### 环境变量（可选）

可以在 `~/.openclaw/.env` 中配置：

```bash
SIYUAN_API_TOKEN=your_token_here
SIYUAN_API_URL=http://127.0.0.1:6806
```

---

## 🔧 错误处理

所有方法返回 `anyhow::Result<T>`，可以使用 `?` 运算符传播错误：

```rust
async fn example() -> anyhow::Result<()> {
    let service = SiYuanService::new("http://127.0.0.1:6806", "token");
    
    // 错误会自动传播
    let notebooks = service.list_notebooks().await?;
    
    Ok(())
}
```

常见错误：
- 网络连接失败
- API Token 无效
- 笔记本/文档/块 ID 不存在
- SQL 语法错误

---

## 📚 示例代码

完整示例请参考：
- `tests/integration.rs` - 集成测试示例
- `src/service.rs` - 单元测试示例

---

## ⚠️ 注意事项

1. **自定义属性前缀**：设置块属性时，自定义属性必须以 `custom-` 为前缀
2. **路径格式**：文档路径使用正斜杠 `/`，例如 `/folder/doc`
3. **块 ID 格式**：SiYuan 块 ID 是 15 位时间戳字符串
4. **并发限制**：避免同时大量写入操作，可能导致性能问题

---

## 📖 相关文档

- [SiYuan 官方 API 文档](https://github.com/siyuan-note/siyuan/blob/master/API_zh.md)
- [ClawHub Rust 技能](https://clawhub.ai/ivangdavila/rust)

---

**版本：** 0.1.0  
**最后更新：** 2026-03-27
