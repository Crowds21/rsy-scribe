//! Vim-like 模式系统

use serde::{Deserialize, Serialize};

/// 编辑器模式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Mode {
    /// 正常模式 - 用于导航和命令
    #[default]
    Normal,
    /// 插入模式 - 用于编辑文本
    Insert,
    /// 命令模式 - 用于输入 : 命令
    Command,
    /// 搜索模式 - 用于搜索
    Search,
}

impl Mode {
    /// 获取模式的显示名称
    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Command => "COMMAND",
            Mode::Search => "SEARCH",
        }
    }

    /// 获取模式的简写
    pub fn as_short_str(&self) -> &'static str {
        match self {
            Mode::Normal => "NOR",
            Mode::Insert => "INS",
            Mode::Command => "CMD",
            Mode::Search => "SRC",
        }
    }
}

/// Leader 键配置
#[derive(Debug, Clone)]
pub struct LeaderConfig {
    /// Leader 键，默认为空格
    pub key: char,
    /// Leader 键超时（毫秒）
    pub timeout: u64,
}

impl Default for LeaderConfig {
    fn default() -> Self {
        Self {
            key: ' ',
            timeout: 1000,
        }
    }
}

/// 命令类型
#[derive(Debug, Clone)]
pub enum Command {
    /// 退出应用
    Exit,
    /// 强制退出（不保存）
    Quit,
    /// 保存
    Write,
    /// 保存并退出
    WriteQuit,
    /// 搜索文件
    SearchFile,
    /// 搜索内容
    SearchContent,
    /// 未知命令
    Unknown(String),
}

impl Command {
    /// 解析命令字符串
    pub fn parse(cmd: &str) -> Self {
        let cmd = cmd.trim();
        match cmd {
            "exit" | "quit" | "q" => Command::Exit,
            "q!" => Command::Quit,
            "write" | "w" => Command::Write,
            "wq" | "x" => Command::WriteQuit,
            "ff" | "find" => Command::SearchFile,
            "fg" | "grep" => Command::SearchContent,
            _ => Command::Unknown(cmd.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode_as_str() {
        assert_eq!(Mode::Normal.as_str(), "NORMAL");
        assert_eq!(Mode::Insert.as_str(), "INSERT");
    }

    #[test]
    fn test_command_parse() {
        assert!(matches!(Command::parse("exit"), Command::Exit));
        assert!(matches!(Command::parse("q"), Command::Exit));
        assert!(matches!(Command::parse("ff"), Command::SearchFile));
        assert!(matches!(Command::parse("unknown_cmd"), Command::Unknown(_)));
    }
}
