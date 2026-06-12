# syservice — SiYuan API 与工作空间集成

Trait 驱动的思源笔记 API 客户端，负责配置加载、HTTP 请求、`.sy` 文件读取与 Lute AST。

## 功能概览

| 类别 | 能力 |
|------|------|
| 配置 | 跨平台 `config.toml`、`Config::global()` |
| HTTP | `HttpClient` 实现 `SiYuanClient` trait |
| 查询 | SQL、`search_by_title` |
| 文件 | `load_json_node_from_workspace` |
| 扩展 | 中间件、自定义 Logger（见 EXTENSIBILITY_GUIDE） |

## 快速开始

```rust
use syservice::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::builder()
        .base_url("http://127.0.0.1:6806")
        .token("your-api-token-here")
        .build();
    let client = HttpClient::new(&config)?;

    if client.ping().await {
        println!("SiYuan reachable");
    }

    let notebooks = client.list_notebooks().await?;
    for nb in notebooks {
        println!("{} ({})", nb.name, nb.id);
    }
    Ok(())
}
```

## 配置

复制 [`config.example.toml`](config.example.toml) 到用户配置目录，或使用 [`scripts/init-config.sh`](scripts/init-config.sh)。

详见 [CONFIG.md](CONFIG.md) 与 [docs/design/003-config-system.md](../docs/design/003-config-system.md)。

## 测试

```bash
# 库单元测试
cargo test -p syservice

# 集成测试（需运行中的思源 + 本地 config.toml）
cargo test -p syservice --test integration -- --ignored
```

见 [tests/README.md](tests/README.md)。

## 模块文档

- [008 — syservice 模块设计](../docs/design/008-syservice-module.md)
- [EXTENSIBILITY_GUIDE.md](EXTENSIBILITY_GUIDE.md)
- [SiYuan 官方 API](https://github.com/siyuan-note/siyuan/blob/master/API_zh.md)
