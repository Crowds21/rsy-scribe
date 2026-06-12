use crate::config::Config;
use crate::lute;
use anyhow::{Context, Result};
use infrastructure::log_error;
use std::path::Path;
use std::{fs::File, io::BufReader};

/// 从思源笔记工作空间加载 JSON 节点文件
/// 
/// # Arguments
/// * `relative_path` - 相对于工作空间的路径（如 "notebooks/20230620162729-abc123/doc.sy"）
/// * `config` - 配置对象（包含工作空间目录）
/// 
/// # Returns
/// * `Ok(Node)` - 成功加载的节点
/// * `Err` - 加载失败，错误信息已打印到 stderr
/// 
/// # Example
/// 
/// ```rust,ignore
/// let config = Config::load();
/// let node = load_json_node_from_workspace(
///     "20230620162729-abc123/doc.sy",
///     &config
/// )?;
/// ```
pub fn load_json_node_from_workspace(
    relative_path: &str,
    config: &Config,
) -> Result<lute::node::Node> {
    // 获取工作空间目录
    let workspace_dir = config.workspace_dir.as_ref().ok_or_else(|| {
        anyhow::anyhow!(
            "Workspace directory not configured.\n   \
             Add 'workspace_dir' to config file"
        )
    })?;
    
    // 清理路径：移除开头和结尾的斜杠，并将中间连续的斜杠替换为单个斜杠
    let relative_path = relative_path
        .trim_start_matches('/')
        .trim_end_matches('/')
        .replace("//", "/");
    
    // 清理工作空间目录：移除结尾的斜杠
    let workspace_dir = workspace_dir.trim_end_matches('/');
    
    // 拼接完整路径（使用 Path API 自动处理不同系统的路径分隔符）
    let full_path = Path::new(workspace_dir).join(&relative_path);
    
    // 加载文件
    load_json_node_from_path(&full_path)
}

/// 从指定路径加载 JSON 节点文件（内部函数）
/// 
/// # Arguments
/// * `path` - 文件的完整绝对路径
/// 
/// # Returns
/// * `Ok(Node)` - 成功加载的节点
/// * `Err` - 加载失败，错误信息已打印到 stderr
fn load_json_node_from_path(path: &Path) -> Result<lute::node::Node> {
    // 打开文件并添加错误上下文
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            log_error(
                "syservice.file",
                format!(
                    "Failed to open file: {}; error: {}",
                    path.display(),
                    e
                ),
            );
            return Err(e).with_context(|| format!("Failed to open file: {}", path.display()));
        }
    };

    // 创建带缓冲的读取器（1MB 缓冲区）
    let reader = BufReader::with_capacity(1024 * 1024, file);

    // 反序列化并添加详细错误上下文
    match serde_json::from_reader(reader) {
        Ok(node) => Ok(node),
        Err(e) => {
            log_error(
                "syservice.file",
                format!("Failed to parse JSON from: {}; error: {}", path.display(), e),
            );
            Err(anyhow::Error::new(e).context(format!(
                "Failed to parse JSON from: {}",
                path.display()
            )))
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[test]
    #[ignore]
    fn test_load_node_from_workspace() {
        // 需要配置工作空间目录
        let config = Config::builder()
            .workspace_dir("/Users/crowds/Notes/SiYuanKnowledgeBase/data")
            .build();
        
        let relative_path = "notebooks/20230620162729-levf2as/20230629142416-fk29t9w/20230629142458-ffxtme3/20240107160843-8f02mqs.sy";
        let json_data = load_json_node_from_workspace(relative_path, &config).unwrap();
        
        let serialized = serde_json::to_string_pretty(&json_data).unwrap();
        infrastructure::log_info("syservice.file.test", serialized);
        assert!(json_data.id.is_some());
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = load_json_node_from_path(Path::new("/nonexistent/path/file.sy"));
        assert!(result.is_err());
    }

    #[test]
    fn test_load_without_workspace_config() {
        let config = Config::default();
        let result = load_json_node_from_workspace("test.sy", &config);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Workspace directory not configured"));
    }

    #[test]
    fn test_path_normalization() {
        let config = Config::builder()
            .workspace_dir("/tmp/test-workspace/")  // 结尾有斜杠
            .build();
        
        // 测试各种斜杠组合
        let test_cases = vec![
            "notebooks/test.sy",           // 正常
            "/notebooks/test.sy",          // 开头有斜杠
            "notebooks/test.sy/",          // 结尾有斜杠
            "/notebooks/test.sy/",         // 两头有斜杠
            "notebooks//test.sy",          // 中间双斜杠（Path::join 会处理）
        ];
        
        for path in test_cases {
            let result = load_json_node_from_workspace(path, &config);
            // 文件不存在是正常的，关键是路径拼接不能出错
            assert!(result.is_err());
            let err_msg = result.unwrap_err().to_string();
            // 确保错误信息中路径不包含双斜杠
            assert!(!err_msg.contains("//"), "Path should not contain double slashes: {}", err_msg);
        }
    }
}
