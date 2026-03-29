# syservice 扩展性指南

本文档介绍如何扩展 `syservice` 模块，添加自定义功能。

---

## 📐 架构概览

```
┌─────────────────────────────────────────────────────────┐
│                   你的应用代码                           │
├─────────────────────────────────────────────────────────┤
│  SiYuanClient (Trait) - 核心扩展点                      │
│  ├─ HttpClient (HTTP 实现)                              │
│  ├─ MockClient (测试实现)                               │
│  └─ CachedClient (缓存实现)                             │
├─────────────────────────────────────────────────────────┤
│  中间件层                                                 │
│  ├─ LoggingMiddleware                                   │
│  ├─ CacheMiddleware                                     │
│  ├─ RateLimitMiddleware                                 │
│  └─ 自定义中间件                                         │
├─────────────────────────────────────────────────────────┤
│  配置层                                                   │
│  ├─ Config (环境变量)                                   │
│  ├─ FileConfig (配置文件)                               │
│  └─ 自定义配置源                                         │
└─────────────────────────────────────────────────────────┘
```

---

## 🔌 扩展点 1：实现自定义客户端

### 场景

- 单元测试需要 Mock 数据
- 添加缓存层减少 API 调用
- 实现离线模式

### 示例：Mock 客户端

```rust
use async_trait::async_trait;
use syservice::api::{SiYuanClient, Notebook};
use syservice::error::Result;
use std::collections::HashMap;

pub struct MockClient {
    notebooks: Vec<Notebook>,
}

impl MockClient {
    pub fn new() -> Self {
        Self {
            notebooks: vec![
                Notebook {
                    id: "1".to_string(),
                    name: "Test Notebook".to_string(),
                    icon: String::new(),
                    sort: 0,
                    closed: false,
                },
            ],
        }
    }
}

#[async_trait]
impl SiYuanClient for MockClient {
    async fn list_notebooks(&self) -> Result<Vec<Notebook>> {
        Ok(self.notebooks.clone())
    }

    async fn create_notebook(&self, name: &str) -> Result<String> {
        Ok("mock_id".to_string())
    }

    async fn remove_notebook(&self, notebook_id: &str) -> Result<()> {
        Ok(())
    }

    // 实现其他方法...
}

// 使用
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_with_mock() {
        let client = MockClient::new();
        let notebooks = client.list_notebooks().await.unwrap();
        assert_eq!(notebooks.len(), 1);
    }
}
```

### 示例：缓存客户端

```rust
use async_trait::async_trait;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use syservice::api::{SiYuanClient, Notebook};
use syservice::error::Result;

pub struct CachedClient<C: SiYuanClient> {
    inner: C,
    cache: Arc<RwLock<HashMap<String, CacheEntry<Vec<Notebook>>>>>,
    ttl: Duration,
}

struct CacheEntry<T> {
    data: T,
    expires_at: Instant,
}

impl<C: SiYuanClient> CachedClient<C> {
    pub fn new(inner: C, ttl: Duration) -> Self {
        Self {
            inner,
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }
}

#[async_trait]
impl<C: SiYuanClient + Send + Sync> SiYuanClient for CachedClient<C> {
    async fn list_notebooks(&self) -> Result<Vec<Notebook>> {
        // 检查缓存
        {
            let cache = self.cache.read().unwrap();
            if let Some(entry) = cache.get("list_notebooks") {
                if entry.expires_at > Instant::now() {
                    return Ok(entry.data.clone());
                }
            }
        }

        // 缓存未命中，调用底层客户端
        let notebooks = self.inner.list_notebooks().await?;

        // 更新缓存
        {
            let mut cache = self.cache.write().unwrap();
            cache.insert(
                "list_notebooks".to_string(),
                CacheEntry {
                    data: notebooks.clone(),
                    expires_at: Instant::now() + self.ttl,
                },
            );
        }

        Ok(notebooks)
    }

    // 其他方法委托给 inner...
}
```

---

## 🔌 扩展点 2：添加中间件

### 场景

- 记录所有请求日志
- 实现请求限流
- 添加性能监控
- 自动重试特定错误

### 示例：日志中间件

```rust
use async_trait::async_trait;
use syservice::api::{RequestMiddleware, MiddlewareContext};
use syservice::error::Result;

pub struct LoggingMiddleware;

#[async_trait]
impl RequestMiddleware for LoggingMiddleware {
    async fn before_request(&self, ctx: &mut MiddlewareContext) -> Result<()> {
        println!(
            "[HTTP] {} {} | Body: {:?}",
            ctx.method, ctx.endpoint, ctx.body
        );
        Ok(())
    }

    async fn after_request(&self, ctx: &MiddlewareContext) -> Result<()> {
        println!(
            "[HTTP] {} {} | Status: {:?} | Duration: {:?}",
            ctx.method, ctx.endpoint, ctx.status_code, ctx.duration
        );
        Ok(())
    }
}

// 使用
let client = HttpClient::new(&config)?
    .with_middleware(Box::new(LoggingMiddleware));
```

### 示例：限流中间件

```rust
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use syservice::api::{RequestMiddleware, MiddlewareContext};
use syservice::error::Result;

pub struct RateLimitMiddleware {
    state: Arc<Mutex<RateLimitState>>,
    requests_per_second: u32,
}

struct RateLimitState {
    last_request: Option<Instant>,
    request_count: u32,
}

impl RateLimitMiddleware {
    pub fn new(requests_per_second: u32) -> Self {
        Self {
            state: Arc::new(Mutex::new(RateLimitState {
                last_request: None,
                request_count: 0,
            })),
            requests_per_second,
        }
    }
}

#[async_trait]
impl RequestMiddleware for RateLimitMiddleware {
    async fn before_request(&self, ctx: &mut MiddlewareContext) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        
        // 检查是否需要限流
        if let Some(last) = state.last_request {
            let elapsed = last.elapsed();
            if elapsed < Duration::from_secs(1) && state.request_count >= self.requests_per_second {
                // 需要等待
                let wait_time = Duration::from_secs(1) - elapsed;
                drop(state);
                tokio::time::sleep(wait_time).await;
            } else if elapsed >= Duration::from_secs(1) {
                // 重置计数器
                state.request_count = 0;
            }
        }

        state.last_request = Some(Instant::now());
        state.request_count += 1;

        Ok(())
    }

    async fn after_request(&self, ctx: &MiddlewareContext) -> Result<()> {
        // 可以在这里记录限流统计信息
        Ok(())
    }
}

// 使用
let client = HttpClient::new(&config)?
    .with_middleware(Box::new(RateLimitMiddleware::new(10))); // 10 requests/sec
```

---

## 🔌 扩展点 3：自定义配置源

### 场景

- 从数据库加载配置
- 从远程配置中心获取
- 支持多环境配置

### 示例：数据库配置

```rust
use syservice::api::ConfigProvider;

pub struct DatabaseConfig {
    url: String,
    token: String,
    timeout_secs: u64,
}

impl DatabaseConfig {
    pub async fn from_database(pool: &DbPool) -> sqlx::Result<Self> {
        let row = sqlx::query!(
            "SELECT api_url, api_token, timeout FROM siyuan_config WHERE active = true"
        )
        .fetch_one(pool)
        .await?;

        Ok(Self {
            url: row.api_url,
            token: row.api_token,
            timeout_secs: row.timeout as u64,
        })
    }
}

impl ConfigProvider for DatabaseConfig {
    fn base_url(&self) -> &str {
        &self.url
    }

    fn token(&self) -> &str {
        &self.token
    }

    fn timeout_secs(&self) -> u64 {
        self.timeout_secs
    }
}

// 使用
let db_config = DatabaseConfig::from_database(&pool).await?;
let client = HttpClient::new_with_config(&db_config)?;
```

---

## 🔌 扩展点 4：自定义日志系统

### 场景

- 集成 tracing
- 发送到远程日志服务
- 结构化日志

### 示例：tracing 集成

```rust
use syservice::api::{Logger, LogLevel};
use tracing::{debug, info, warn, error};

pub struct TracingLogger;

impl Logger for TracingLogger {
    fn log(&self, level: LogLevel, msg: &str) {
        match level {
            LogLevel::Debug => debug!("{}", msg),
            LogLevel::Info => info!("{}", msg),
            LogLevel::Warn => warn!("{}", msg),
            LogLevel::Error => error!("{}", msg),
        }
    }
}

// 使用
let logger = TracingLogger;
logger.info("SiYuan client initialized");
```

---

## 🎯 最佳实践

### 1. 依赖抽象

```rust
// ✅ 推荐：依赖 Trait
fn process(client: &dyn SiYuanClient) {
    // ...
}

// ❌ 避免：依赖具体类型
fn process(client: &HttpClient) {
    // ...
}
```

### 2. 组合优于继承

```rust
// ✅ 推荐：组合
let client = HttpClient::new(&config)?
    .with_middleware(Box::new(LoggingMiddleware))
    .with_middleware(Box::new(RateLimitMiddleware::new(10)));

// ❌ 避免：继承
struct LoggedHttpClient {
    client: HttpClient,
    // ...
}
```

### 3. 小专注的 Trait

```rust
// ✅ 推荐：小而专注
pub trait Logger {
    fn log(&self, level: LogLevel, msg: &str);
}

// ❌ 避免：大而全
pub trait Everything {
    fn log(&self, ...);
    fn configure(&self, ...);
    fn connect(&self, ...);
    // ... 50 个方法
}
```

---

## 📚 完整示例

### 生产环境配置

```rust
use syservice::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. 加载配置
    let config = Config::from_env()?;

    // 2. 创建中间件链
    let middlewares = vec![
        Box::new(LoggingMiddleware) as Box<dyn RequestMiddleware>,
        Box::new(RateLimitMiddleware::new(10)),
        Box::new(MetricsMiddleware::new()),
    ];

    // 3. 创建客户端
    let client = HttpClient::new(&config)?
        .with_middlewares(middlewares);

    // 4. 使用
    let notebooks = client.list_notebooks().await?;
    println!("Found {} notebooks", notebooks.len());

    Ok(())
}
```

---

**最后更新：** 2026-03-27  
**版本：** 0.2.0
