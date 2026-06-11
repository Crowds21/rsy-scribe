//! 配置管理模块
//!
//! 支持多种配置来源（优先级从高到低）：
//! 1. 环境变量
//! 2. 配置文件（跨平台路径）
//! 3. 默认值
//!
//! # 配置文件路径
//!
//! - **Linux/macOS**: `~/.config/scribe/config.toml`
//! - **Windows**: `%APPDATA%\scribe\config.toml`
//!
//! # 配置文件格式
//!
//! ```toml
//! base_url = "http://127.0.0.1:6806"
//! token = "your-api-token"
//! timeout_secs = 30
//! ```
//!
//! # 环境变量
//!
//! - `SIYUAN_API_TOKEN` - API 令牌（可选）
//! - `SIYUAN_API_URL` - API 基础 URL（可选，默认：http://127.0.0.1:6806）
//! - `SIYUAN_TIMEOUT_SECS` - 超时时间（可选，默认：30）
//! - `SIYUAN_WORKSPACE_DIR` - 思源笔记工作空间目录（可选）

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use infrastructure::log_info;
use thiserror::Error;
use crate::api::ConfigProvider;

static GLOBAL_CONFIG: OnceLock<Config> = OnceLock::new();

fn config_summary(config: &Config) -> String {
    format!(
        "base_url={}, timeout_secs={}, workspace_dir={:?}, token_present={}",
        config.base_url,
        config.timeout_secs,
        config.workspace_dir,
        !config.token.is_empty()
    )
}

/// SiYuan 服务配置
#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub token: String,
    pub timeout_secs: u64,
    /// 思源笔记工作空间目录（如：/Users/crowds/Notes/SiYuanKnowledgeBase/data）
    pub workspace_dir: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "http://127.0.0.1:6806".to_string(),
            token: String::new(),
            timeout_secs: 30,
            workspace_dir: None,
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
    
    pub fn workspace_dir(mut self, dir: &str) -> Self {
        self.config.workspace_dir = Some(dir.trim_end_matches('/').to_string());
        self
    }
    
    pub fn build(self) -> Config { self.config }
}

/// 配置错误类型
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Invalid API URL: {0}")]
    InvalidUrl(String),
    #[error("Failed to read config file: {0}")]
    FileReadError(String),
    #[error("Failed to parse config file: {0}")]
    ParseError(String),
}

/// 获取所有可能的配置文件路径（按优先级排序）
///
/// 返回多个路径，按优先级从高到低排序：
///
/// **Linux/macOS:**
/// 1. `$XDG_CONFIG_HOME/scribe/config.toml`
/// 2. `~/.config/scribe/config.toml`
///
/// **Windows:**
/// 1. `%APPDATA%\scribe\config.toml`
///
pub fn get_config_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    
    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = env::var("APPDATA") {
            paths.push(PathBuf::from(appdata).join("scribe").join("config.toml"));
        }
    }
    
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        // 优先使用 XDG_CONFIG_HOME
        if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
            paths.push(PathBuf::from(xdg_config).join("scribe").join("config.toml"));
        }
        // 回退到 ~/.config
        if let Ok(home) = env::var("HOME") {
            paths.push(PathBuf::from(&home).join(".config").join("scribe").join("config.toml"));
        }
    }

    #[cfg(target_os = "macos")]
    {
        // 兼容 macOS 常见配置路径
        if let Ok(home) = env::var("HOME") {
            paths.push(
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("scribe")
                    .join("config.toml"),
            );
        }
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        // 其他系统回退到 ~/.config
        if let Ok(home) = env::var("HOME") {
            paths.push(PathBuf::from(&home).join(".config").join("scribe").join("config.toml"));
        }
    }
    
    paths
}

/// 获取配置文件路径（跨平台）
///
/// 返回第一个可能的配置文件路径（用于保存配置）
/// 
/// 遵循 XDG Base Directory 规范：
/// - Linux: `$XDG_CONFIG_HOME/scribe/config.toml` 或 `~/.config/scribe/config.toml`
/// - macOS: `~/Library/Application Support/scribe/config.toml`
/// - Windows: `%APPDATA%\scribe\config.toml`
pub fn get_config_path() -> Option<PathBuf> {
    get_config_paths().into_iter().next()
}

/// 从配置文件加载配置
pub fn load_from_file(path: &Path) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::FileReadError(format!("{}: {}", path.display(), e)))?;
    
    parse_config_toml(&content)
}

/// 解析 TOML 格式的配置内容
fn parse_config_toml(content: &str) -> Result<Config, ConfigError> {
    // 简单的 TOML 解析（避免额外依赖）
    let mut config = Config::default();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            
            match key {
                "base_url" => config.base_url = value.trim_end_matches('/').to_string(),
                "token" => config.token = value.to_string(),
                "timeout_secs" => config.timeout_secs = value.parse().unwrap_or(30),
                "workspace_dir" => config.workspace_dir = Some(value.trim_end_matches('/').to_string()),
                _ => {} // 忽略未知字段
            }
        }
    }
    
    Ok(config)
}

/// 从默认配置文件路径加载配置
///
/// 按优先级检查所有可能的配置文件路径，返回第一个存在的配置文件
pub fn load_from_default_path() -> Option<Config> {
    let config_paths = get_config_paths();
    let tried_paths = config_paths
        .iter()
        .map(|p| p.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    log_info("config", format!("probing config paths: {}", tried_paths));

    for path in config_paths {
        if path.exists() {
            if let Ok(config) = load_from_file(&path) {
                log_info("config", format!("config file hit: {}", path.display()));
                return Some(config);
            }
        }
    }
    None
}

/// 保存配置到文件
///
/// 如果未指定路径，则使用平台默认的首选路径
pub fn save_to_file(config: &Config, path: Option<&Path>) -> Result<(), ConfigError> {
    let config_path = match path {
        Some(p) => p.to_path_buf(),
        None => get_config_path().ok_or_else(|| {
            ConfigError::FileReadError("Failed to determine config path".to_string())
        })?,
    };
    
    // 创建目录（如果不存在）
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            ConfigError::FileReadError(format!("Failed to create config dir: {}", e))
        })?;
    }
    
    // 生成 TOML 内容
    let workspace_line = match &config.workspace_dir {
        Some(dir) => format!("workspace_dir = \"{}\"\n", dir),
        None => String::new(),
    };
    
    let content = format!(
        r#"# SiYuan 配置文件
# 生成时间：{}
#
# 注意：macOS 用户也可以使用 ~/.config/scribe/config.toml 路径
# 这样方便与其他平台共享配置

base_url = "{}"
token = "{}"
timeout_secs = {}
{}"#,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        config.base_url,
        config.token,
        config.timeout_secs,
        workspace_line
    );
    
    fs::write(&config_path, content).map_err(|e| {
        ConfigError::FileReadError(format!("Failed to write config file: {}", e))
    })?;
    
    Ok(())
}

impl Config {
    pub fn builder() -> ConfigBuilder { ConfigBuilder::new() }

    /// 从环境变量加载配置
    ///
    /// 所有配置项都是可选的，未设置时使用默认值
    pub fn from_env() -> Self {
        let mut config = Config::default();
        
        if let Ok(token) = env::var("SIYUAN_API_TOKEN") {
            config.token = token;
        }
        if let Ok(url) = env::var("SIYUAN_API_URL") {
            config.base_url = url.trim_end_matches('/').to_string();
        }
        if let Ok(timeout) = env::var("SIYUAN_TIMEOUT_SECS") {
            config.timeout_secs = timeout.parse().unwrap_or(30);
        }
        if let Ok(workspace) = env::var("SIYUAN_WORKSPACE_DIR") {
            config.workspace_dir = Some(workspace.trim_end_matches('/').to_string());
        }
        
        config
    }

    /// 从配置文件加载配置（如果存在）
    pub fn from_file() -> Option<Self> {
        load_from_default_path()
    }

    /// 从环境变量或配置文件加载配置（环境变量优先）
    ///
    /// 如果环境变量和配置文件都不存在，返回默认配置
    pub fn load() -> Self {
        // 优先使用环境变量
        if env::var("SIYUAN_API_TOKEN").is_ok()
            || env::var("SIYUAN_API_URL").is_ok()
            || env::var("SIYUAN_TIMEOUT_SECS").is_ok()
            || env::var("SIYUAN_WORKSPACE_DIR").is_ok()
        {
            let config = Self::from_env();
            log_info(
                "config",
                format!("loaded from env: {}", config_summary(&config)),
            );
            return config;
        }
        
        // 回退到配置文件
        if let Some(config) = Self::from_file() {
            log_info(
                "config",
                format!("loaded from file: {}", config_summary(&config)),
            );
            return config;
        }
        
        // 使用默认配置
        let config = Config::default();
        log_info(
            "config",
            format!("loaded from default: {}", config_summary(&config)),
        );
        config
    }

    /// 从环境变量或配置文件加载配置（同 `load()`）
    pub fn load_or_default() -> Self {
        Self::load()
    }

    /// 在应用启动阶段初始化全局配置，只加载一次。
    pub fn init_global() -> &'static Self {
        GLOBAL_CONFIG.get_or_init(Self::load)
    }

    /// 获取全局配置；若尚未初始化则惰性加载一次。
    pub fn global() -> &'static Self {
        GLOBAL_CONFIG.get_or_init(Self::load)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

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
        // 默认配置应该有效（token 不是必需的）
        let config = Config::default();
        assert!(config.validate().is_ok());
        
        // 无效的 URL 应该失败
        let config = Config::builder()
            .base_url("invalid-url")
            .build();
        assert!(config.validate().is_err());
        
        // 有效的配置
        let config = Config::builder()
            .token("test")
            .base_url("http://localhost:6806")
            .build();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_parse_config_toml() {
        let content = r#"
base_url = "http://192.168.1.100:6806"
token = "test_token_123"
timeout_secs = 60
"#;
        let config = parse_config_toml(content).unwrap();
        assert_eq!(config.base_url, "http://192.168.1.100:6806");
        assert_eq!(config.token, "test_token_123");
        assert_eq!(config.timeout_secs, 60);
    }

    #[test]
    fn test_parse_config_with_comments() {
        let content = r#"
# 这是注释
base_url = "http://localhost:6806"
# token 注释
token = "my_token"
"#;
        let config = parse_config_toml(content).unwrap();
        assert_eq!(config.base_url, "http://localhost:6806");
        assert_eq!(config.token, "my_token");
    }

    #[test]
    fn test_parse_config_partial() {
        let content = r#"
token = "only_token"
"#;
        let config = parse_config_toml(content).unwrap();
        assert_eq!(config.token, "only_token");
        assert_eq!(config.base_url, "http://127.0.0.1:6806"); // default
        assert_eq!(config.timeout_secs, 30); // default
    }

    #[test]
    fn test_config_path_exists() {
        let path = get_config_path();
        assert!(path.is_some(), "Should return a config path");
    }

    #[test]
    fn test_config_paths_returns_multiple() {
        let paths = get_config_paths();
        assert!(!paths.is_empty(), "Should return at least one config path");
        
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            // Linux/macOS 应该返回 1-2 个路径（取决于 XDG_CONFIG_HOME 是否设置）
            assert!(paths.len() >= 1, "Linux/macOS should have at least 1 config path");
            // 所有路径都应该包含 .config
            for path in &paths {
                assert!(path.to_string_lossy().contains(".config"), 
                    "Config path should use .config directory");
            }
        }
    }

    #[test]
    fn test_load_from_temp_file() {
        let content = r#"
base_url = "http://test:6806"
token = "temp_token"
"#;
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(content.as_bytes()).unwrap();
        
        let config = load_from_file(temp_file.path()).unwrap();
        assert_eq!(config.base_url, "http://test:6806");
        assert_eq!(config.token, "temp_token");
    }
}
