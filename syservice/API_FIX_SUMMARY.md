# API URL 修正总结

## 🐛 发现的问题

### `get_version()` 函数

**错误的代码：**
```rust
async fn get_version(&self) -> Result<String> {
    #[derive(Debug, Default, serde::Deserialize)]
    struct VersionData { version: String }
    let response: ApiResponse<VersionData> = self.post("/api/system/getVersion", &json!({})).await?;
    Ok(response.data.version)
}
```

**问题：**
1. ❌ **URL 错误** - 使用了 `/api/system/getVersion`
2. ❌ **返回值解析错误** - 期望 `data` 是对象 `{ version: "..." }`

---

## ✅ 官方 API 文档

根据 [SiYuan API 文档](https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#获取系统版本)：

```
### 获取系统版本

* `/api/system/version`
* 不带参
* 返回值

  ```json
  {
    "code": 0,
    "msg": "",
    "data": "1.3.5"
  }
  ```
```

**正确格式：**
- ✅ URL: `/api/system/version`
- ✅ 返回值：`data` 直接是版本字符串

---

## 🔧 修正后的代码

```rust
async fn get_version(&self) -> Result<String> {
    // 官方 API: /api/system/version 直接返回版本字符串
    // https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#获取系统版本
    let response: ApiResponse<String> = self.post("/api/system/version", &json!({})).await?;
    Ok(response.data)
}
```

**修正点：**
1. ✅ URL 改为 `/api/system/version`
2. ✅ 直接解析 `ApiResponse<String>`
3. ✅ 添加官方文档链接注释

---

## 📊 其他系统 API 检查

根据官方文档，系统相关 API 如下：

| API | 端点 | 状态 |
|-----|------|------|
| 获取启动进度 | `/api/system/bootProgress` | ⏸️ 待实现 |
| **获取系统版本** | **`/api/system/version`** | ✅ **已修正** |
| 获取系统当前时间 | `/api/system/currentTime` | ⏸️ 待实现 |

---

## 🧪 测试结果

```
running 15 tests
test result: ok. 13 passed; 0 failed; 2 ignored
```

✅ 所有测试通过！

---

## 📝 建议

### 添加其他系统 API

可以考虑添加以下 API 方法：

#### 1. 获取启动进度

```rust
async fn get_boot_progress(&self) -> Result<BootProgress> {
    #[derive(Debug, Default, serde::Deserialize)]
    struct BootProgressData {
        details: String,
        progress: u8,
    }
    let response: ApiResponse<BootProgressData> = 
        self.post("/api/system/bootProgress", &json!({})).await?;
    Ok(response.data)
}
```

#### 2. 获取系统当前时间

```rust
async fn get_current_time(&self) -> Result<i64> {
    // 返回 Unix 时间戳（毫秒精度）
    let response: ApiResponse<i64> = 
        self.post("/api/system/currentTime", &json!({})).await?;
    Ok(response.data)
}
```

---

## 📚 参考文档

- [SiYuan API 中文文档 - 系统](https://github.com/siyuan-note/siyuan/blob/master/API_zh.md#系统)
- [SiYuan API English Documentation - System](https://github.com/siyuan-note/siyuan/blob/master/API.md#system)
- [本地技能文档](/Users/crowds/.openclaw/workspace/skills/siyuan-api/references/api-zh.md)

---

**修正时间：** 2026-03-27  
**状态：** ✅ 完成
