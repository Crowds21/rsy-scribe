use std::{fs::File, io::Read};

use super::Mode;
use super::*;
use anyhow::Error;

pub struct Editor {
    pub mode: Mode,
    pub documents: String,
}

impl Editor {
    pub fn new() -> Self {
        let backend = CrosstermBackend::new(stdout());
        let terminal = MyTerminal::new(backend).unwrap();
        Self {
            mode: Mode::Normal,
            documents: editor_open().unwrap(),
        }
    }
}
/// TEMPORARY: This function is only for temporary use and will be removed in the future.
fn editor_open() -> Result<String, Error> {
    // self.text.insert(0, content);
    // TODO: Cannot recongise `~`. Need to parse manually
    let mut file = File::open("/Users/crowds/Scripts/orgmode/index.norg")?;
    // 创建一个字符串缓冲区来存储文件内容
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // 将字符串转换为 Rope
    Ok(contents)
}
