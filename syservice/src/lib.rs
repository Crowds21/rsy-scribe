//! SiYuan Service - 可扩展的 SiYuan 笔记系统 API 客户端
//!
//! 采用 Trait 驱动设计，提供高度扩展性。

// 模块声明
pub mod api;
pub mod client;
pub mod config;
pub mod document;
pub mod domain;
pub mod error;
pub mod file;
pub mod lute;
pub mod test_utils;

// 预导出模块
pub mod prelude {
    pub use crate::api::{ConfigProvider, ConsoleLogger, Logger, LogLevel, MiddlewareContext, Notebook, RequestMiddleware, SiYuanClient};
    pub use crate::client::HttpClient;
    pub use crate::config::Config;
    pub use crate::error::{Result, SiYuanError};
    pub use crate::domain::{SyBlock, SyResponse};
    pub use anyhow;
    pub use async_trait::async_trait;
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_prelude_imports() {
        let _config = Config::builder().token("test").base_url("http://localhost").build();
    }

    #[test]
    fn test_http_client_creation() {
        let config = Config::builder().token("test").base_url("http://localhost:6806").build();
        let client = HttpClient::new(&config);
        assert!(client.is_ok());
    }

    #[test]
    fn test_log_level_ordering() {
        use crate::api::LogLevel;
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Warn < LogLevel::Error);
    }
}
