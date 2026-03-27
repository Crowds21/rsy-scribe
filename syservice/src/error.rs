//! 错误处理模块

use thiserror::Error;

/// SiYuan 服务错误类型
#[derive(Debug, Error)]
pub enum SiYuanError {
    #[error("Configuration error: {0}")]
    Config(#[from] crate::config::ConfigError),

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("API error (code {code}): {message}")]
    Api { code: i32, message: String },

    #[error("Resource not found: {resource_type} - {identifier}")]
    NotFound { resource_type: String, identifier: String },

    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Connection timeout after {timeout_secs}s")]
    Timeout { timeout_secs: u64 },

    #[error("Failed to connect to SiYuan at {url}")]
    ConnectionFailed { url: String },

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, SiYuanError>;

impl SiYuanError {
    pub fn is_retryable(&self) -> bool {
        matches!(self, SiYuanError::Http(_) | SiYuanError::Timeout { .. } | SiYuanError::ConnectionFailed { .. })
    }

    pub fn is_auth_error(&self) -> bool {
        matches!(self, SiYuanError::AuthFailed(_))
    }

    pub fn user_message(&self) -> String {
        match self {
            SiYuanError::Config(e) => format!("配置错误：{}", e),
            SiYuanError::Http(e) => format!("网络请求失败：{}", e),
            SiYuanError::Api { code, message } => format!("SiYuan API 错误 ({}): {}", code, message),
            SiYuanError::NotFound { resource_type, identifier } => format!("未找到{}: {}", resource_type, identifier),
            SiYuanError::AuthFailed(_) => "认证失败，请检查 API Token".to_string(),
            SiYuanError::Timeout { timeout_secs } => format!("请求超时（{}秒），请检查网络连接", timeout_secs),
            SiYuanError::ConnectionFailed { url } => format!("无法连接到 SiYuan: {}", url),
            _ => self.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_type() {
        let err = SiYuanError::AuthFailed("Invalid token".to_string());
        assert!(err.is_auth_error());
        assert!(!err.is_retryable());
    }

    #[test]
    fn test_user_message() {
        let err = SiYuanError::Timeout { timeout_secs: 30 };
        assert!(err.user_message().contains("超时"));
    }
}
