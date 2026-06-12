# SiYuan 配置指南

## 配置方式

Scribe 从用户目录下的 **`config.toml`** 加载配置（见 [`config.rs`](src/config.rs) 中的路径探测逻辑）。若文件不存在，则使用内置默认值（空 token、默认 URL）。

**当前未支持**通过 `SIYUAN_API_*` 环境变量覆盖配置。

### 配置文件路径

| 操作系统 | 路径（按优先级） |
|----------|------------------|
| Linux / macOS | `$XDG_CONFIG_HOME/scribe/config.toml`，`~/.config/scribe/config.toml` |
| macOS | 另可：`~/Library/Application Support/scribe/config.toml` |
| Windows | `%APPDATA%\scribe\config.toml` |

### 配置文件格式

```toml
base_url = "http://127.0.0.1:6806"
token = "your-api-token-here"
timeout_secs = 30
workspace_dir = "/path/to/SiYuanKnowledgeBase/data"
```

完整示例：[`config.example.toml`](config.example.toml)

快速创建：

```bash
./syservice/scripts/init-config.sh
```

## 获取 API Token

1. 打开思源笔记
2. **设置 → 关于 → API token**
3. 复制并写入本地 `config.toml`（勿提交到 git）

## Rust 代码

```rust
use syservice::prelude::*;

fn main() -> anyhow::Result<()> {
    Config::init_global();
    let config = Config::global();
    let client = HttpClient::new(config)?;
    Ok(())
}
```

## 配置项

| 字段 | 默认值 | 说明 |
|------|--------|------|
| `base_url` | `http://127.0.0.1:6806` | API 地址 |
| `token` | `""` | API 令牌 |
| `timeout_secs` | `30` | 超时（秒） |
| `workspace_dir` | 无 | 工作空间 `data` 目录，打开 `.sy` 必需 |

## 集成测试

```bash
# 需本地 config.toml 含有效 token 与 workspace_dir
cargo test -p syservice --test integration -- --ignored
```

## 安全提示

- 勿将真实 token 写入仓库或示例文件
- 配置文件建议权限 `600`
- Token 泄露后请在思源中轮换

## 相关文档

- [docs/design/003-config-system.md](../docs/design/003-config-system.md)
- [README.md](README.md)
