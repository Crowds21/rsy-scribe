use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use anyhow::Context;
use syservice::lute;

fn get_resource_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/resources")
        .join(filename)
}
/// 从 tests/resources/ 目录下加载 示例sy 文件
pub fn load_sy_test_file() -> anyhow::Result<lute::node::Node> {
    let path = get_resource_path("template.sy");
    // 打开文件并添加错误上下文
    let file =
        File::open(&path).with_context(|| format!("Failed to open file: {}", path.display()))?;

    // 创建带缓冲的读取器（1MB缓冲区）
    let reader = BufReader::with_capacity(1024 * 1024, file);

    // 反序列化并添加详细错误上下文
    serde_json::from_reader(reader).map_err(|e| {
        // 将原始错误转换为 anyhow::Error 并添加上下文
        anyhow::Error::new(e).context(format!("Failed to parse JSON from: {}", path.display()))
    })
}
