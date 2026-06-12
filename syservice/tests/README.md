# 集成测试文档

## 测试文件

- **tests/integration.rs** - SiYuan API 集成测试（根据官方 API 更新）

---

## 运行测试

### 运行所有测试

```bash
# 运行单元测试
cargo test --package syservice --lib

# 运行集成测试（需要 SiYuan 实例）
cargo test --test integration -- --ignored
```

### 运行单个测试

```bash
# 完整工作流测试
cargo test --test integration test_full_workflow -- --ignored --nocapture

# 笔记本 CRUD 测试
cargo test --test integration test_notebook_crud -- --ignored --nocapture

# 文档 CRUD 测试
cargo test --test integration test_document_crud -- --ignored --nocapture

# 块属性测试
cargo test --test integration test_block_attributes -- --ignored --nocapture

# 块操作测试（新增）
cargo test --test integration test_block_operations -- --ignored --nocapture

# 错误处理测试
cargo test --test integration test_error_handling -- --ignored --nocapture

# 中间件测试
cargo test --test integration test_with_middleware -- --ignored --nocapture

# 文档重命名测试（新增）
cargo test --test integration test_rename_doc_by_id -- --ignored --nocapture
```

---

## 测试用例

### 1. `test_full_workflow` - 完整工作流测试

**目的：** 验证完整的 SiYuan API 工作流

**测试步骤：**
1. 检查服务连通性（ping）
2. 获取 SiYuan 版本
3. 列出所有笔记本
4. 搜索文档（标题包含 "OpenClaw"）
5. 获取块属性
6. 追加块到文档（验证 `BlockOperation` 返回值）
7. 执行 SQL 查询

**预期结果：** 所有操作成功完成

---

### 2. `test_notebook_crud` - 笔记本 CRUD 测试

**目的：** 验证笔记本的创建、读取、删除操作

**测试步骤：**
1. 创建测试笔记本（验证返回 `Notebook` 对象）
2. 验证笔记本存在
3. 删除笔记本
4. 验证笔记本已删除

**预期结果：** 笔记本成功创建和删除

**更新点：** 使用新的返回值类型 `Notebook` 而不是 `String`

---

### 3. `test_document_crud` - 文档 CRUD 测试

**目的：** 验证文档的创建、读取、更新、删除操作

**测试步骤：**
1. 获取一个现有笔记本
2. 创建测试文档
3. 通过 SQL 查询验证文档存在
4. 更新文档内容（验证 `BlockOperation` 返回值）
5. 通过 ID 删除文档
6. 验证文档已删除

**预期结果：** 文档成功创建、更新和删除

**更新点：** 使用 `remove_doc_by_id()` 和新的返回值类型

---

### 4. `test_block_attributes` - 块属性测试

**目的：** 验证块属性的读取和设置操作

**测试步骤：**
1. 搜索测试块
2. 获取现有属性
3. 设置自定义属性（`custom-integration-test`）
4. 验证属性值
5. 清理测试属性

**预期结果：** 属性成功设置和验证

---

### 5. `test_block_operations` - 块操作测试（新增）

**目的：** 验证新的块操作 API 和 `BlockOperation` 返回值

**测试步骤：**
1. 测试 `prepend_block()` - 插入前置子块
2. 测试 `append_block()` - 插入后置子块
3. 测试 `insert_block()` - 插入块（通过 previousID）
4. 测试 `delete_block()` - 删除块

**预期结果：**
- 所有操作返回 `Vec<BlockOperation>`
- 每个操作包含正确的 `action`、`id` 等字段

**新增验证：**
```rust
for op in &operations {
    assert!(!op.id.is_empty(), "Operation should have an ID");
    assert_eq!(op.action, "insert", "Action should be 'insert'");
}
```

---

### 6. `test_error_handling` - 错误处理测试

**目的：** 验证错误处理机制

**测试步骤：**
1. 使用无效的 Token 创建客户端
2. 尝试列出笔记本
3. 验证返回认证错误

**预期结果：** 正确检测到认证错误

---

### 7. `test_with_middleware` - 中间件测试

**目的：** 验证中间件功能

**测试步骤：**
1. 创建日志中间件
2. 将中间件添加到客户端
3. 执行请求
4. 验证中间件记录了请求

**预期结果：** 中间件正确记录请求信息

---

### 8. `test_rename_doc_by_id` - 文档重命名测试（新增）

**目的：** 验证通过 ID 重命名文档的新 API

**测试步骤：**
1. 创建测试文档
2. 使用 `rename_doc_by_id()` 重命名
3. 清理测试文档

**预期结果：** 文档成功重命名

**新增 API：**
```rust
async fn rename_doc_by_id(&self, id: &str, title: &str) -> Result<()>;
```

---

## 测试配置

集成测试通过 `Config::load()` 读取本地 `~/.config/scribe/config.toml`（或平台等价路径），**不在代码中硬编码 token**。

本地需配置：

```toml
base_url = "http://127.0.0.1:6806"
token = "your-api-token-here"
workspace_dir = "/path/to/SiYuanKnowledgeBase/data"
```

自定义客户端示例：

```rust
fn create_client() -> HttpClient {
    let config = Config::builder()
        .base_url("http://127.0.0.1:6806")
        .token("your-api-token-here")
        .timeout_secs(60)
        .build();
    HttpClient::new(&config).expect("Failed to create HTTP client")
}
```

---

## 前置条件

### 1. SiYuan 笔记运行中

确保 SiYuan 笔记在 `http://127.0.0.1:6806` 运行。

### 2. API Token 配置

确保使用有效的 API Token。可以在 SiYuan 设置 > 关于中获取。

### 3. 测试数据

某些测试需要预置数据（如标题包含 "OpenClaw" 的文档）。

---

## API 更新说明

### 主要变更

1. **`create_notebook` 返回值**
   - 旧：`Result<String>` (返回 notebook ID)
   - 新：`Result<Notebook>` (返回完整的 Notebook 对象)

2. **块操作返回值**
   - 旧：`Result<()>` 或 `Result<Vec<String>>`
   - 新：`Result<Vec<BlockOperation>>`

3. **新增接口**
   - `insert_block()` - 插入块
   - `prepend_block()` - 插入前置子块
   - `remove_doc_by_id()` - 通过 ID 删除文档
   - `rename_doc_by_id()` - 通过 ID 重命名文档

### 迁移示例

#### 笔记本创建

**旧代码：**
```rust
let notebook_id = client.create_notebook("My Notebook").await?;
println!("Created: {}", notebook_id);
```

**新代码：**
```rust
let notebook = client.create_notebook("My Notebook").await?;
println!("Created: {} ({})", notebook.name, notebook.id);
```

#### 块追加

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

## 常见问题

### Q: 测试被忽略（ignored）

**A:** 集成测试默认被标记为 `#[ignore]`，需要添加 `--ignored` 标志运行。

### Q: 认证失败

**A:** 检查 `TEST_TOKEN` 是否有效。可以在 SiYuan 设置 > 关于中获取 API Token。

### Q: 连接超时

**A:** 确保 SiYuan 服务正在运行，并且防火墙允许连接。

### Q: 测试失败

**A:** 使用 `--nocapture` 标志查看详细错误信息：

```bash
cargo test --test integration test_full_workflow -- --ignored --nocapture
```

---

## 添加新测试

### 模板

```rust
#[tokio::test]
#[ignore = "requires running SiYuan instance"]
async fn test_your_feature() {
    let client = create_client();
    
    // 你的测试代码...
    
    assert!(/* 断言 */);
}
```

### 最佳实践

1. **使用 `#[ignore]`** - 集成测试需要 SiYuan 实例
2. **清理测试数据** - 测试完成后删除创建的数据
3. **使用 `expect()`** - 提供清晰的错误信息
4. **添加打印输出** - 便于调试（使用 `println!`）
5. **验证返回值类型** - 确保使用新的 `BlockOperation` 类型

---

## 测试状态

| 测试 | 状态 | 需要 SiYuan | 说明 |
|------|------|-------------|------|
| `test_full_workflow` | ⏸️ 已忽略 | ✅ | 完整工作流 |
| `test_notebook_crud` | ⏸️ 已忽略 | ✅ | 笔记本 CRUD（更新） |
| `test_document_crud` | ⏸️ 已忽略 | ✅ | 文档 CRUD（更新） |
| `test_block_attributes` | ⏸️ 已忽略 | ✅ | 块属性 |
| `test_block_operations` | ⏸️ 已忽略 | ✅ | 块操作（新增） |
| `test_error_handling` | ⏸️ 已忽略 | ✅ | 错误处理 |
| `test_with_middleware` | ⏸️ 已忽略 | ✅ | 中间件 |
| `test_rename_doc_by_id` | ⏸️ 已忽略 | ✅ | 重命名文档（新增） |

**单元测试：** ✅ 13/13 通过  
**集成测试：** ⏸️ 8 个（需要 SiYuan 实例）

---

**最后更新：** 2026-03-27  
**版本：** 0.2.0（根据官方 API 更新）
