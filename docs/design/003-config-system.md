# 003 — 配置系统设计

**状态：** 已实现  
**最后更新：** 2026-06-11

---

## 1. 背景

早期版本曾在代码中硬编码 API 地址与 Token，存在泄露风险且不便多环境部署。当前通过 [`syservice/src/config.rs`](../../syservice/src/config.rs) 从用户目录的 `config.toml` 加载配置。

---

## 2. Config 结构

```rust
pub struct Config {
    pub base_url: String,
    pub token: String,
    pub timeout_secs: u64,
    pub workspace_dir: Option<String>,
}
```

| 字段 | 说明 |
|------|------|
| `base_url` | 思源 API 根地址，默认 `http://127.0.0.1:6806` |
| `token` | API 令牌（搜索等需认证接口） |
| `timeout_secs` | HTTP 超时（秒） |
| `workspace_dir` | 工作空间 `data` 目录，用于读取 `.sy` 文件 |

---

## 3. 配置文件路径

按优先级探测（首个存在的文件生效）：

| 平台 | 路径 |
|------|------|
| Linux / macOS | `$XDG_CONFIG_HOME/scribe/config.toml`，`~/.config/scribe/config.toml` |
| macOS（追加） | `~/Library/Application Support/scribe/config.toml` |
| Windows | `%APPDATA%/scribe/config.toml` |

示例文件：[`syservice/config.example.toml`](../../syservice/config.example.toml)

---

## 4. 加载行为

**当前实现：仅从配置文件加载；无配置文件时使用 `Config::default()`。**

```rust
Config::init_global();  // app 启动时
let config = Config::global();

// 或
let config = Config::load();
```

> **注意：** 未实现 `SIYUAN_API_*` 环境变量覆盖。HTTP 重试次数在 `HttpClient` 内硬编码（3 次 / 100ms），**不在** `config.toml` 中配置。

---

## 5. 示例 config.toml

```toml
base_url = "http://127.0.0.1:6806"
token = "your-api-token-here"
timeout_secs = 30
workspace_dir = "/path/to/SiYuanKnowledgeBase/data"
```

初始化脚本：[`syservice/scripts/init-config.sh`](../../syservice/scripts/init-config.sh)

用户指南：[`syservice/CONFIG.md`](../../syservice/CONFIG.md)

---

## 6. 安全

1. 勿将含真实 `token` 的 `config.toml` 提交到 git
2. `init-config.sh` 写入 `chmod 600`
3. 若 Token 曾出现在历史提交中，请在思源设置中**轮换 API Token**

---

## 7. 测试

```bash
cargo test -p syservice config
cargo test -p syservice --test integration -- --ignored
```

---

## 8. 未来改进

- 可选环境变量覆盖
- `[[notebooks]]` 多笔记本白名单（见 [010 — 笔记本访问控制](010-notebook-access-control.md)）
- 配置热重载

---

## 9. 关联文档

- [008 — syservice 模块](008-syservice-module.md)
