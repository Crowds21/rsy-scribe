//! 配置管理模块

use std::env;
use thiserror::Error;
use crate::api::ConfigProvider;

/// SiYuan 服务配置
#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub token: String,
    pub timeout_secs: u64,
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:6806".to_string(),
            token: String::new(),
            timeout_secs: 30,
            max_retries: 3,
            retry_delay_ms: 100,
        }
    }
}

/// 配置构建器
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self { Self::default() }
    pub fn base_url(mut self, url: &str) -> Self {
        self.config.base_url = url.trim_end_matches('/').to_string();
        self
    }
    pub fn token(mut self, token: &str) -> Self {
        self.config.token = token.to_string();
        self
    }
    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.config.timeout_secs = secs;
        self
    }
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }
    pub fn retry_delay_ms(mut self, ms: u64) -> Self {
        self.config.retry_delay_ms = ms;
        self
    }
    pub fn build(self) -> Config { self.config }
}

/// 配置错误类型
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required SIYUAN_API_TOKEN environment variable")]
    MissingToken,
    #[error("Invalid API URL: {0}")]
    InvalidUrl(String),
}

impl Config {
    pub fn builder() -> ConfigBuilder { ConfigBuilder::new() }

    pub fn from_env() -> Result<Self, ConfigError> {
        let mut config = Config::default();
        config.token = env::var("SIYUAN_API_TOKEN").map_err(|_| ConfigError::MissingToken)?;
        if let Ok(url) = env::var("SIYUAN_API_URL") {
            config.base_url = url.trim_end_matches('/').to_string();
        }
        if let Ok(timeout) = env::var("SIYUAN_TIMEOUT_SECS") {
            config.timeout_secs = timeout.parse().unwrap_or(30);
        }
        if let Ok(retries) = env::var("SIYUAN_MAX_RETRIES") {
            config.max_retries = retries.parse().unwrap_or(3);
        }
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.token.is_empty() {
            return Err(ConfigError::MissingToken);
        }
        if !self.base_url.starts_with("http") {
            return Err(ConfigError::InvalidUrl(self.base_url.clone()));
        }
        Ok(())
    }
}

impl ConfigProvider for Config {
    fn base_url(&self) -> &str { &self.base_url }
    fn token(&self) -> &str { &self.token }
    fn timeout_secs(&self) -> u64 { self.timeout_secs }
    fn max_retries(&self) -> u32 { self.max_retries }
    fn retry_delay_ms(&self) -> u64 { self.retry_delay_ms }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = Config::builder()
            .base_url("http://localhost:6806")
            .token("test_token")
            .timeout_secs(60)
            .build();
        assert_eq!(config.base_url, "http://localhost:6806");
        assert_eq!(config.token, "test_token");
    }

    #[test]
    fn test_config_validate() {
        let config = Config::default();
        assert!(config.validate().is_err());
        
        let config = Config::builder()
            .token("test")
            .base_url("http://localhost:6806")
            .build();
        assert!(config.validate().is_ok());
    }
}
