use crate::lute;
use anyhow::{Context, Result};
use std::path::Path;
use std::{fs::File, io::BufReader};

/// 从指定路径加载 JSON 节点文件
/// 
/// # Arguments
/// * `file_path` - 文件的完整路径（绝对路径或相对于当前工作目录）
pub fn load_json_node(file_path: &str) -> Result<lute::node::Node> {
    let path = Path::new(file_path);
    
    // 打开文件并添加错误上下文
    let file = File::open(path)
        .with_context(|| format!("Failed to open file: {}", path.display()))?;

    // 创建带缓冲的读取器（1MB 缓冲区）
    let reader = BufReader::with_capacity(1024 * 1024, file);

    // 反序列化并添加详细错误上下文
    serde_json::from_reader(reader).map_err(|e| {
        anyhow::Error::new(e).context(format!(
            "Failed to parse JSON from: {}",
            file_path
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_load_node() {
        let path = "20230620162729-levf2as/20230629142416-fk29t9w/20230629142458-ffxtme3/20240107160843-8f02mqs.sy";
        let json_data = load_json_node(path).unwrap();
        
        let serialized = serde_json::to_string_pretty(&json_data).unwrap();
        println!("{}", serialized);
        assert!(json_data.id.is_some());
    }
}
