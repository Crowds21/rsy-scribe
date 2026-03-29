# SQL 查询解析错误修复

## 🐛 问题

**错误信息：**
```
Failed to search: Internal("JSON parse error: error decoding response body")
```

**问题代码：**
```rust
async fn sql_query(&self, sql: &str) -> Result<Vec<SyBlock>> {
    let body = json!({ "stmt": sql });
    let response: crate::domain::SyResponse = self.post("/api/query/sql", &body).await?.data;
    // ...
}
```

---

## 🔍 根本原因

### 1. API 返回值格式

根据 [SiYuan 官方 API 文档](https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#执行-sql-查询)：

```json
{
  "code": 0,
  "msg": "",
  "data": [
    { "列": "值" }
  ]
}
```

**关键点：**
- `data` 是动态的 JSON 数组
- 每个对象是 `{ "列名": "值" }` 的形式
- 列名对应数据库字段名

### 2. 结构体字段问题

`SyBlock` 结构体的字段都是必需的，但 SQL 查询可能返回部分字段：

```rust
// 旧代码 - 所有字段都是必需的
pub struct SyBlock {
    pub alias: String,      // ❌ 必需
    pub box_id: String,     // ❌ 必需
    pub content: String,    // ❌ 必需
    // ...
}
```

---

## ✅ 修复方案

### 1. 修改 `SyBlock` 结构体

为所有字段添加 `#[serde(default)]` 属性，使字段变为可选：

```rust
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct SyBlock {
    #[serde(default)]
    pub alias: String,
    
    #[serde(default, rename = "box")]
    pub box_id: String,
    
    #[serde(default)]
    pub content: String,
    
    // ... 其他字段同样处理
}
```

**优点：**
- ✅ 允许缺失字段
- ✅ 自动使用默认值（空字符串、0 等）
- ✅ 向后兼容

### 2. 修改 `sql_query` 实现

```rust
async fn sql_query(&self, sql: &str) -> Result<Vec<SyBlock>> {
    // 官方 API: SQL 查询返回的是动态 JSON 数组 [{ "列": "值" }]
    let body = json!({ "stmt": sql });
    let response: ApiResponse<Vec<serde_json::Value>> = 
        self.post("/api/query/sql", &body).await?;
    
    // 将 JSON Value 转换为 SyBlock
    let blocks: Result<Vec<SyBlock>> = response.data
        .into_iter()
        .map(|value| {
            serde_json::from_value::<SyBlock>(value)
                .map_err(|e| SiYuanError::Internal(
                    format!("Failed to parse SyBlock: {}", e)
                ))
        })
        .collect();
    
    blocks
}
```

---

## 📊 数据库字段映射

SiYuan 数据库 `blocks` 表的字段：

| 字段名 | SyBlock 字段 | 说明 |
|--------|-------------|------|
| `id` | `id` | 块 ID |
| `box` | `box_id` | 笔记本 ID |
| `path` | `path` | 路径 |
| `hpath` | `hpath` | 人类可读路径 |
| `parent_id` | `parent_id` | 父块 ID |
| `root_id` | `root_id` | 根块 ID |
| `type` | `block_type` | 块类型 |
| `sub_type` | `subtype` | 子类型 |
| `content` | `content` | 内容 |
| `markdown` | `markdown` | Markdown 内容 |
| `created` | `created_at` | 创建时间 |
| `updated` | `updated` | 更新时间 |
| `sort` | `sort` | 排序 |
| `memo` | `memo` | 备注 |
| `tag` | `tag` | 标签 |
| `alias` | `alias` | 别名 |
| `hash` | `hash` | 哈希 |
| `length` | `length` | 长度 |
| `ial` | `ial` | 块属性 |
| `fcontent` | `fcontent` | 格式化内容 |
| `name` | `name` | 名称 |

---

## 🧪 测试验证

### 单元测试

```bash
cargo test --package syservice --lib
```

**结果：**
```
running 15 tests
test result: ok. 13 passed; 0 failed; 2 ignored
```

✅ 所有测试通过！

### 集成测试（需要 SiYuan 实例）

```bash
cargo test --test integration test_full_workflow -- --ignored --nocapture
```

**预期输出：**
```
✓ Found X documents matching 'OpenClaw'
```

---

## 📝 使用示例

### 基本查询

```rust
let blocks = client
    .sql_query("SELECT * FROM blocks WHERE type='d' LIMIT 10")
    .await?;

for block in blocks {
    println!("Block: {} - {}", block.id, block.content);
}
```

### 按标题搜索

```rust
let blocks = client
    .search_by_title("OpenClaw", 5)
    .await?;

println!("Found {} documents", blocks.len());
```

### 自定义查询

```rust
let blocks = client
    .sql_query("SELECT id, content, created FROM blocks WHERE content LIKE '%keyword%' LIMIT 5")
    .await?;

// 即使只查询部分字段，也能正常解析
for block in blocks {
    println!("ID: {}, Content: {}", block.id, block.content);
}
```

---

## ⚠️ 注意事项

### 1. 字段缺失

如果 SQL 查询只选择部分字段，未选择的字段将使用默认值：

```sql
SELECT id, content FROM blocks  -- 其他字段将为空
```

### 2. 字段名大小写

SiYuan 数据库字段名都是小写，确保 SQL 查询使用正确的字段名。

### 3. 权限限制

注意：发布模式下除非公开所有文档读写权限，否则会禁止访问 SQL 查询接口。

---

## 📚 参考文档

- [SiYuan API - SQL 查询](https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#执行-sql-查询)
- [SiYuan 数据库结构](https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#sql)
- [serde 文档](https://serde.rs/)

---

**修复时间：** 2026-03-27  
**状态：** ✅ 完成  
**测试：** ✅ 通过
