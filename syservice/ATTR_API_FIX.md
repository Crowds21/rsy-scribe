# 块属性 API 解析错误修复

## 🐛 问题

**错误信息：**
```
Failed to get attributes: Internal("JSON parse error: error decoding response body")
```

**问题代码：**
```rust
async fn get_block_attrs(&self, block_id: &str) -> Result<HashMap<String, String>> {
    #[derive(Debug, Default, serde::Deserialize)]
    struct AttrData { attrs: HashMap<String, String> }
    let response: ApiResponse<AttrData> = self.post("/api/attr/getBlockAttrs", &body).await?;
    Ok(response.data.attrs)  // ❌ 错误：期望 data.attrs
}
```

---

## 🔍 根本原因

根据 [SiYuan 官方 API 文档](https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#获取块属性)：

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "custom-attr1": "line1\nline2",
    "id": "20210912214605-uhi5gco",
    "title": "PDF 标注双链演示",
    "type": "doc",
    "updated": "20210916120715"
  }
}
```

**关键点：**
- ✅ `data` **直接就是属性对象**（HashMap）
- ❌ **不是** `{ attrs: {...} }` 的包装格式

---

## ✅ 修复方案

### 修改前

```rust
async fn get_block_attrs(&self, block_id: &str) -> Result<HashMap<String, String>> {
    #[derive(Debug, Default, serde::Deserialize)]
    struct AttrData { attrs: HashMap<String, String> }
    
    let body = json!({ "id": block_id });
    let response: ApiResponse<AttrData> = self.post("/api/attr/getBlockAttrs", &body).await?;
    Ok(response.data.attrs)  // ❌ 错误
}
```

### 修改后

```rust
async fn get_block_attrs(&self, block_id: &str) -> Result<HashMap<String, String>> {
    // 官方 API: data 直接就是属性对象 { "custom-attr1": "value", "id": "..." }
    // https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#获取块属性
    let body = json!({ "id": block_id });
    let response: ApiResponse<HashMap<String, String>> = 
        self.post("/api/attr/getBlockAttrs", &body).await?;
    Ok(response.data)  // ✅ 正确
}
```

---

## 📊 API 返回值对比

### 错误的期望格式

```json
{
  "code": 0,
  "msg": "",
  "data": {
    "attrs": {  // ❌ 不存在的包装层
      "custom-attr1": "value",
      "id": "..."
    }
  }
}
```

### 正确的实际格式

```json
{
  "code": 0,
  "msg": "",
  "data": {  // ✅ data 直接就是属性对象
    "custom-attr1": "value",
    "id": "20210912214605-uhi5gco",
    "title": "PDF 标注双链演示",
    "type": "doc",
    "updated": "20210916120715"
  }
}
```

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
cargo test --test integration test_block_attributes -- --ignored --nocapture
```

**预期输出：**
```
✓ Block has X existing attributes
✓ Set attribute: custom-integration-test = test-value-123
✓ Cleaned up test attribute
✅ Block attributes test passed!
```

---

## 📝 使用示例

### 获取块属性

```rust
let block_id = "20210912214605-uhi5gco";
let attrs = client.get_block_attrs(block_id).await?;

println!("Block {} has {} attributes", block_id, attrs.len());

// 访问特定属性
if let Some(title) = attrs.get("title") {
    println!("Title: {}", title);
}

if let Some(custom_attr) = attrs.get("custom-attr1") {
    println!("Custom attr: {}", custom_attr);
}
```

### 设置块属性

```rust
let mut attrs = HashMap::new();
attrs.insert("custom-status".to_string(), "completed".to_string());
attrs.insert("custom-priority".to_string(), "high".to_string());

client.set_block_attrs(block_id, attrs).await?;
```

### 设置单个属性

```rust
client.set_block_attr(block_id, "custom-note", "Important note").await?;
```

---

## ⚠️ 注意事项

### 1. 属性命名规范

- 自定义属性必须以 `custom-` 为前缀
- 系统属性（如 `id`, `title`, `type` 等）是只读的

### 2. 属性值类型

所有属性值都是字符串类型。如果需要存储复杂数据，可以：
- 使用 JSON 字符串
- 使用换行符分隔多行文本

### 3. 特殊字符处理

属性值中的换行符会被保留：

```rust
let mut attrs = HashMap::new();
attrs.insert("custom-notes".to_string(), "line1\nline2\nline3".to_string());
client.set_block_attrs(block_id, attrs).await?;
```

---

## 📚 相关 API

### 获取块属性

```rust
async fn get_block_attrs(&self, block_id: &str) -> Result<HashMap<String, String>>;
```

**端点：** `/api/attr/getBlockAttrs`

### 设置块属性

```rust
async fn set_block_attrs(
    &self,
    block_id: &str,
    attrs: HashMap<String, String>
) -> Result<()>;
```

**端点：** `/api/attr/setBlockAttrs`

### 设置单个属性（辅助方法）

```rust
async fn set_block_attr(
    &self,
    block_id: &str,
    key: &str,
    value: &str
) -> Result<()>;
```

---

## 🔗 参考文档

- [SiYuan API - 获取块属性](https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#获取块属性)
- [SiYuan API - 设置块属性](https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#设置块属性)
- [本地技能文档](/Users/crowds/.openclaw/workspace/skills/siyuan-api/references/api-zh.md)

---

**修复时间：** 2026-03-27  
**状态：** ✅ 完成  
**测试：** ✅ 通过
