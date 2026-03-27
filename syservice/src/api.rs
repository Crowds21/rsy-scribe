//! SiYuan API 核心 Trait 定义
//!
//! 本模块定义了 SiYuan API 的核心接口，所有实现都必须遵循这些 Trait。

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;

use crate::domain::SyBlock;
use crate::error::Result;

// ==================== 基础类型定义 ====================

/// API 响应包装器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    #[serde(default)]
    pub data: T,
}

/// 笔记本信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Notebook {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub sort: i32,
    #[serde(default)]
    pub closed: bool,
}

/// 块操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockOperation {
    #[serde(default)]
    pub action: String,
    #[serde(default)]
    pub data: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default, rename = "parentID")]
    pub parent_id: String,
    #[serde(default, rename = "previousID")]
    pub previous_id: String,
}

// ==================== 核心 Client Trait ====================

/// SiYuan API 客户端核心 Trait
#[async_trait]
pub trait SiYuanClient: Send + Sync {
    // ========== 笔记本操作 ==========
    async fn list_notebooks(&self) -> Result<Vec<Notebook>>;
    async fn create_notebook(&self, name: &str) -> Result<Notebook>;
    async fn remove_notebook(&self, notebook_id: &str) -> Result<()>;

    // ========== 文档操作 ==========
    async fn create_doc_with_md(
        &self,
        notebook_id: &str,
        path: &str,
        markdown: &str,
    ) -> Result<String>;
    async fn remove_doc(&self, doc_id: &str) -> Result<()>;
    async fn remove_doc_by_id(&self, id: &str) -> Result<()>;
    async fn rename_doc(&self, notebook_id: &str, path: &str, title: &str) -> Result<()>;
    async fn rename_doc_by_id(&self, id: &str, title: &str) -> Result<()>;

    // ========== 块操作 ==========
    async fn insert_block(
        &self,
        data_type: &str,
        data: &str,
        previous_id: Option<&str>,
        next_id: Option<&str>,
        parent_id: Option<&str>,
    ) -> Result<Vec<BlockOperation>>;
    async fn prepend_block(
        &self,
        parent_id: &str,
        data_type: &str,
        data: &str,
    ) -> Result<Vec<BlockOperation>>;
    async fn append_block(
        &self,
        parent_id: &str,
        data_type: &str,
        data: &str,
    ) -> Result<Vec<BlockOperation>>;
    async fn update_block(&self, block_id: &str, data_type: &str, data: &str) -> Result<Vec<BlockOperation>>;
    async fn delete_block(&self, block_id: &str) -> Result<Vec<BlockOperation>>;

    // ========== 查询操作 ==========
    async fn sql_query(&self, sql: &str) -> Result<Vec<SyBlock>>;
    async fn search_by_title(&self, title: &str, limit: usize) -> Result<Vec<SyBlock>>;

    // ========== 属性操作 ==========
    async fn get_block_attrs(&self, block_id: &str) -> Result<HashMap<String, String>>;
    async fn set_block_attrs(&self, block_id: &str, attrs: HashMap<String, String>) -> Result<()>;
    async fn set_block_attr(&self, block_id: &str, key: &str, value: &str) -> Result<()>;

    // ========== 系统操作 ==========
    async fn get_version(&self) -> Result<String>;
    async fn ping(&self) -> bool;
}

// ==================== 配置 Trait ====================

/// 配置提供者 Trait
pub trait ConfigProvider: Send + Sync {
    fn base_url(&self) -> &str;
    fn token(&self) -> &str;
    fn timeout_secs(&self) -> u64 { 30 }
    fn max_retries(&self) -> u32 { 3 }
    fn retry_delay_ms(&self) -> u64 { 100 }
}

// ==================== 日志 Trait ====================

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// 日志记录器 Trait
pub trait Logger: Send + Sync {
    fn log(&self, level: LogLevel, msg: &str);
    fn debug(&self, msg: &str) { self.log(LogLevel::Debug, msg); }
    fn info(&self, msg: &str) { self.log(LogLevel::Info, msg); }
    fn warn(&self, msg: &str) { self.log(LogLevel::Warn, msg); }
    fn error(&self, msg: &str) { self.log(LogLevel::Error, msg); }
}

/// 空操作日志实现
pub struct NoOpLogger;
impl Logger for NoOpLogger {
    fn log(&self, _level: LogLevel, _msg: &str) {}
}

/// 控制台日志实现
pub struct ConsoleLogger {
    pub min_level: LogLevel,
}

impl ConsoleLogger {
    pub fn new(min_level: LogLevel) -> Self { Self { min_level } }
}

impl Default for ConsoleLogger {
    fn default() -> Self { Self::new(LogLevel::Info) }
}

impl Logger for ConsoleLogger {
    fn log(&self, level: LogLevel, msg: &str) {
        if level >= self.min_level {
            println!("[{:?}] {}", level, msg);
        }
    }
}

// ==================== 中间件 Trait ====================

/// 中间件上下文
#[derive(Debug)]
pub struct MiddlewareContext {
    pub endpoint: String,
    pub method: String,
    pub body: Option<String>,
    pub status_code: Option<u16>,
    pub duration: std::time::Duration,
    pub error: Option<String>,
}

impl MiddlewareContext {
    pub fn new(endpoint: &str, method: &str) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            method: method.to_string(),
            body: None,
            status_code: None,
            duration: std::time::Duration::from_millis(0),
            error: None,
        }
    }
}

/// 请求中间件 Trait
#[async_trait]
pub trait RequestMiddleware: Send + Sync {
    async fn before_request(&self, ctx: &mut MiddlewareContext) -> Result<()>;
    async fn after_request(&self, ctx: &MiddlewareContext) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_op_logger() {
        let logger = NoOpLogger;
        logger.debug("This should not print");
    }

    #[test]
    fn test_console_logger() {
        let logger = ConsoleLogger::new(LogLevel::Debug);
        logger.info("Test message");
    }

    #[test]
    fn test_middleware_context() {
        let ctx = MiddlewareContext::new("/api/test", "POST");
        assert_eq!(ctx.endpoint, "/api/test");
        assert_eq!(ctx.method, "POST");
    }
}
