# SiYuan API 更新总结

根据官方 SiYuan API 文档更新了 `syservice` 模块的接口实现。

---

## 📋 主要变更

### 1. 返回值格式修正

#### `create_notebook`

**旧格式：**
```rust
async fn create_notebook(&self, name: &str) -> Result<String>;
// 返回 notebook ID
```

**新格式（符合官方 API）：**
```rust
async fn create_notebook(&self, name: &str) -> Result<Notebook>;
// 返回完整的 Notebook 对象
```

**API 响应：**
```json
{
  "code": 0,
  "msg": "",
  "data": {
    "notebook": {
      "id": "20220126215949-r1wvoch",
      "name": "笔记本的名称",
      "icon": "",
      "sort": 0,
      "closed": false
    }
  }
}
```

---

### 2. 新增数据类型

#### `BlockOperation`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockOperation {
    pub action: String,          // "insert", "update", "delete", "move"
    pub data: Option<String>,    // 生成的 DOM
    pub id: String,              // 块 ID
    pub parent_id: String,       // 父块 ID
    pub previous_id: String,     // 前一个块 ID
}
```

用于表示块操作的结果，包含操作类型、生成的 DOM、受影响的块 ID 等。

---

### 3. 接口签名更新

#### 块操作接口

| 接口 | 旧签名 | 新签名 |
|------|--------|--------|
| `insert_block` | ❌ 不存在 | ✅ 新增 |
| `prepend_block` | ❌ 不存在 | ✅ 新增 |
| `append_block` | `Result<Vec<String>>` | `Result<Vec<BlockOperation>>` |
| `update_block` | `Result<()>` | `Result<Vec<BlockOperation>>` |
| `delete_block` | `Result<()>` | `Result<Vec<BlockOperation>>` |

#### 文档操作接口

| 接口 | 旧签名 | 新签名 |
|------|--------|--------|
| `remove_doc` | `Result<()>` | `Result<()>` (通过 path) |
| `remove_doc_by_id` | ❌ 不存在 | ✅ 新增 (通过 ID) |
| `rename_doc` | `Result<()>` | `Result<()>` (通过 path) |
| `rename_doc_by_id` | ❌ 不存在 | ✅ 新增 (通过 ID) |

---

## 🆕 新增接口

### 块操作

#### `insert_block`

```rust
async fn insert_block(
    &self,
    data_type: &str,
    data: &str,
    previous_id: Option<&str>,
    next_id: Option<&str>,
    parent_id: Option<&str>,
) -> Result<Vec<BlockOperation>>;
```

**端点：** `/api/block/insertBlock`

**说明：** 插入块，支持通过 `previousID`、`nextID`、`parentID` 定位插入位置。

---

#### `prepend_block`

```rust
async fn prepend_block(
    &self,
    parent_id: &str,
    data_type: &str,
    data: &str,
) -> Result<Vec<BlockOperation>>;
```

**端点：** `/api/block/prependBlock`

**说明：** 插入为父块的第一个子块。

---

#### `append_block`

```rust
async fn append_block(
    &self,
    parent_id: &str,
    data_type: &str,
    data: &str,
) -> Result<Vec<BlockOperation>>;
```

**端点：** `/api/block/appendBlock`

**说明：** 追加为父块的最后一个子块。

---

### 文档操作

#### `remove_doc_by_id`

```rust
async fn remove_doc_by_id(&self, id: &str) -> Result<()>;
```

**端点：** `/api/filetree/removeDocByID`

**说明：** 通过块 ID 删除文档（而不是 path）。

---

#### `rename_doc_by_id`

```rust
async fn rename_doc_by_id(&self, id: &str, title: &str) -> Result<()>;
```

**端点：** `/api/filetree/renameDocByID`

**说明：** 通过块 ID 重命名文档（而不是 path）。

---

## 📊 返回值对比

### 块操作返回值

**旧版本：**
```rust
Result<Vec<String>>  // 仅返回块 ID 列表
```

**新版本：**
```rust
Result<Vec<BlockOperation>>  // 返回完整的操作信息
```

**示例响应：**
```json
{
  "code": 0,
  "msg": "",
  "data": [
    {
      "doOperations": [
        {
          "action": "insert",
          "data": "<div data-node-id=\"20220108003710-hm0x9sc\"...>",
          "id": "20220108003710-hm0x9sc",
          "parentID": "20220107173950-7f9m1nb",
          "previousID": ""
        }
      ]
    }
  ]
}
```

---

## 🔧 迁移指南

### 从旧版本迁移

#### 1. 更新 `create_notebook` 调用

**旧代码：**
```rust
let notebook_id = client.create_notebook("My Notebook").await?;
println!("Created: {}", notebook_id);
```

**新代码：**
```rust
let notebook = client.create_notebook("My Notebook").await?;
println!("Created: {} ({})", notebook.id, notebook.name);
```

---

#### 2. 更新块操作调用

**旧代码：**
```rust
let block_ids = client.append_block(parent_id, "markdown", content).await?;
for id in block_ids {
    println!("Created block: {}", id);
}
```

**新代码：**
```rust
let operations = client.append_block(parent_id, "markdown", content).await?;
for op in operations {
    println!("Created block: {} (action: {})", op.id, op.action);
}
```

---

## ✅ 测试状态

```
running 15 tests
test result: ok. 13 passed; 0 failed; 2 ignored
```

所有单元测试通过！

---

## 📚 参考文档

- [SiYuan API 中文文档](https://github.com/siyuan-note/siyuan/blob/master/API_zh.md)
- [SiYuan API English Documentation](https://github.com/siyuan-note/siyuan/blob/master/API.md)
- [本地技能文档](/Users/crowds/.openclaw/workspace/skills/siyuan-api/references/api-zh.md)

---

**更新时间：** 2026-03-27  
**版本：** 0.2.0
