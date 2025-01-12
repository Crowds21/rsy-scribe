use std::{fs::File, io::Read};

use crate::application::Mode;
use crate::application::*;
use anyhow::Error;

pub struct Editor {
    pub mode: Mode,
}

impl Editor {
    pub fn new() -> Self {
        // let backend = CrosstermBackend::new(stdout());
        // let terminal = MyTerminal::new(backend).unwrap();
        Self {
            mode: Mode::Normal,
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
