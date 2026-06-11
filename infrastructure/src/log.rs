//! 基础设施日志模块。
//!
//! 设计目标：
//! - 全局单例日志线程：任意时刻只有一个后台线程执行实际日志写入。
//! - 异步不阻塞：业务线程仅负责通过 channel 投递日志事件。
//! - 可配置目录：支持在初始化时指定日志目录。
//! - 自动清理：日志文件达到 10MB 后自动截断清理，避免无限增长。
//!
//! 使用方式：
//! 1. 可选：应用启动早期调用 `init_logger_with_directory(...)` 配置日志落盘目录。
//! 2. 业务代码调用 `log_info/log_warn/log_error/...` 输出日志。
//! 3. 若未配置目录，日志默认输出到 stdout/stderr。
//!
//! 注意：`init_logger_with_directory` 与日志线程初始化均是“首次生效”，后续重复调用不会覆盖已初始化状态。

use chrono::Local;
use std::fs::{self, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc::{self, Sender};
use std::sync::OnceLock;
use std::thread;

#[derive(Clone, Copy, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// 写入线程消费的日志事件。
struct LogEvent {
    level: LogLevel,
    scope: String,
    message: String,
}

/// 日志线程命令。
///
/// 生产环境只使用 `Event`；测试环境会启用 `Flush`，
/// 用于等待后台队列处理完成，避免测试读取文件时出现竞态。
enum LoggerCommand {
    Event(LogEvent),
    #[cfg(test)]
    Flush(Sender<()>),
}

/// 全局日志发送端（单例）。
///
/// 通过 `OnceLock::get_or_init` 确保 `init_logger` 最多执行一次，
/// 从而只创建一条 mpsc 通道和一个日志线程。
static LOG_SENDER: OnceLock<Sender<LoggerCommand>> = OnceLock::new();
/// 可选日志文件路径（单例）。
static LOG_FILE_PATH: OnceLock<PathBuf> = OnceLock::new();
/// 单文件最大大小：10MB，达到后触发清理（截断）。
const MAX_LOG_FILE_SIZE_BYTES: u64 = 10 * 1024 * 1024;
/// 默认日志文件名。
const DEFAULT_LOG_FILE_NAME: &str = "infrastructure.log";

/// 初始化后台日志线程并返回全局发送端。
///
/// 该函数本身不做幂等控制，必须通过 `LOG_SENDER.get_or_init(init_logger)` 调用。
/// 这保证了并发场景下只会有一次实际初始化。
fn init_logger() -> Sender<LoggerCommand> {
    let (command_tx, command_rx) = mpsc::channel::<LoggerCommand>();
    thread::Builder::new()
        .name("infrastructure-logger".to_string())
        .spawn(move || {
            while let Ok(command) = command_rx.recv() {
                match command {
                    LoggerCommand::Event(event) => write_log_line(event),
                    #[cfg(test)]
                    LoggerCommand::Flush(tx) => {
                        let _ = tx.send(());
                    }
                }
            }
        })
        .expect("failed to spawn infrastructure logger thread");
    command_tx
}

/// 投递一条日志事件到后台线程。
///
/// 该函数只做事件发送，不做 I/O，因此不会因磁盘/终端写入而阻塞调用方。
pub fn log(level: LogLevel, scope: &str, message: impl AsRef<str>) {
    let sender = LOG_SENDER.get_or_init(init_logger);
    let _ = sender.send(LoggerCommand::Event(LogEvent {
        level,
        scope: scope.to_string(),
        message: message.as_ref().to_string(),
    }));
}

/// 初始化文件日志目录（只生效一次，需在首次 `log` 前调用）。
///
/// - 日志文件固定为 `<log_dir>/infrastructure.log`
/// - 达到 10MB 后会自动清理（截断）并继续写入
pub fn init_logger_with_directory(log_dir: impl AsRef<Path>) -> io::Result<()> {
    let dir = log_dir.as_ref();
    fs::create_dir_all(dir)?;
    let file_path = dir.join(DEFAULT_LOG_FILE_NAME);
    let _ = LOG_FILE_PATH.set(file_path);
    let _ = LOG_SENDER.get_or_init(init_logger);
    Ok(())
}

/// 在后台线程中执行实际日志写入。
///
/// - 若配置了日志文件：写入文件（先检查是否需要清理）。
/// - 若未配置文件：回退到控制台输出。
fn write_log_line(event: LogEvent) {
    let line = format!(
        "[{}] [{:?}] [{}] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        event.level,
        event.scope,
        event.message
    );

    if let Some(path) = LOG_FILE_PATH.get() {
        cleanup_if_oversized(path);
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(file, "{}", line);
        }
        return;
    }

    match event.level {
        LogLevel::Error => eprintln!("{}", line),
        _ => println!("{}", line),
    }
}

/// 文件大小达到阈值时执行清理（截断为空文件）。
///
/// 清理策略为“超限即清空并继续写”，实现简单且稳定；
/// 后续如需保留历史可扩展为按时间或索引轮转。
fn cleanup_if_oversized(path: &Path) {
    let needs_cleanup = fs::metadata(path)
        .map(|meta| meta.len() >= MAX_LOG_FILE_SIZE_BYTES)
        .unwrap_or(false);
    if needs_cleanup {
        let _ = fs::write(path, "");
    }
}

/// 统一基础设施日志输出，便于后续替换为 tracing/file logger。
pub fn log_info(scope: &str, message: impl AsRef<str>) {
    log(LogLevel::Info, scope, message);
}

/// 输出 Debug 级别日志。
pub fn log_debug(scope: &str, message: impl AsRef<str>) {
    log(LogLevel::Debug, scope, message);
}

/// 输出 Warn 级别日志。
pub fn log_warn(scope: &str, message: impl AsRef<str>) {
    log(LogLevel::Warn, scope, message);
}

/// 输出 Error 级别日志。
pub fn log_error(scope: &str, message: impl AsRef<str>) {
    log(LogLevel::Error, scope, message);
}

#[cfg(test)]
fn set_log_file_path_for_test(path: PathBuf) {
    let _ = LOG_FILE_PATH.set(path);
}

#[cfg(test)]
fn flush_for_test() {
    let sender = LOG_SENDER.get_or_init(init_logger);
    let (ack_tx, ack_rx) = mpsc::channel();
    let _ = sender.send(LoggerCommand::Flush(ack_tx));
    let _ = ack_rx.recv_timeout(std::time::Duration::from_secs(1));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{Duration, Instant};

    #[test]
    fn logger_supports_directory_and_cleans_when_10mb() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!(
            "infrastructure-log-test-dir-{}-{}",
            std::process::id(),
            nanos
        ));
        let _ = fs::remove_dir_all(&temp_dir);
        init_logger_with_directory(&temp_dir).expect("logger dir init should work");
        let temp_path = temp_dir.join(DEFAULT_LOG_FILE_NAME);
        set_log_file_path_for_test(temp_path.clone());

        let start = Instant::now();
        for i in 0..500 {
            log_info("test.scope", format!("message-{}", i));
        }
        let elapsed = start.elapsed();

        // 日志通过 channel 异步投递，主线程不应被明显阻塞。
        assert!(elapsed < Duration::from_millis(50));

        flush_for_test();
        let content = fs::read_to_string(&temp_path).expect("log file should exist");
        assert!(content.contains("[Info] [test.scope] message-0"));
        assert!(content.contains("[Info] [test.scope] message-499"));

        // 预填充超过 10MB，验证下一次写入会触发清理。
        let oversized = vec![b'x'; (MAX_LOG_FILE_SIZE_BYTES as usize) + 1024];
        fs::write(&temp_path, oversized).expect("should create oversized log file");
        log_info("test.scope", "post-cleanup-message");
        flush_for_test();
        let after_cleanup_size = fs::metadata(&temp_path)
            .expect("log file should exist")
            .len();
        assert!(after_cleanup_size < MAX_LOG_FILE_SIZE_BYTES);
        let after_cleanup_content =
            fs::read_to_string(&temp_path).expect("log file should be readable after cleanup");
        assert!(after_cleanup_content.contains("post-cleanup-message"));

        fs::write(&temp_path, "").expect("should clear log file after test");
        let after_clear = fs::read_to_string(&temp_path).expect("file should still exist");
        assert!(after_clear.is_empty());
        let _ = fs::remove_file(&temp_path);
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
