use crate::lute;
use anyhow::{Context, Result};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::{fs::File, io::BufReader};

fn load_json_node(file_path: &Path) -> Result<lute::node::Node> {
    // 打开文件并添加错误上下文
    let file = File::open(file_path)
        .with_context(|| format!("Failed to open file: {}", file_path.display()))?;

    // 创建带缓冲的读取器（1MB缓冲区）
    let reader = BufReader::with_capacity(1024 * 1024, file);

    // 反序列化并添加详细错误上下文
     serde_json::from_reader(reader).map_err(|e| {
        // 将原始错误转换为 anyhow::Error 并添加上下文
        anyhow::Error::new(e).context(format!(
            "Failed to parse JSON from: {}",
            file_path.display()
        ))
    })

  
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn test_load_node() {
        let note_dir = Path::new("/Users/crowds/Notes/SiYuanKnowledgeBase/data");
        let doc_path =
            Path::new("20230620162729-levf2as/20230629142416-fk29t9w/20230629142458-ffxtme3/20240107160843-8f02mqs.sy");
        let path = note_dir.join(doc_path);
        let json_data = load_json_node(path.as_path()).unwrap();
        
        let serialized = serde_json::to_string_pretty(&json_data).unwrap();
        println!("{}", serialized);
        assert!(json_data.id.is_some());
    }
}
