use anyhow::{Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use syservice::lute;
use tui::component::block::{doc, RenderedBlock};
use tui::compositor::CompositorContext;

#[test]
fn parse_node_test() {
    let sy_file = load_sy_test_file();
    assert!(sy_file.is_ok());
    let mut root = sy_file.unwrap();
    let cx = CompositorContext::new();
    let mut vec: Vec<RenderedBlock> = Vec::new();
    root.set_node_type_for_tree();
    doc::create_tui_element(&root, &cx, &mut vec);
    assert!(vec.len() > 0)
}

fn get_resource_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/resources")
        .join(filename)
}
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
