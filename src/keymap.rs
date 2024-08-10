use arc_swap::access::DynAccess;
use crossterm::event::KeyEvent;
use std::collections::{BTreeSet, HashMap};
pub mod keyboard;
pub mod macros;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal = 0,
    Select = 1,
    Insert = 2,
    Command = 3,
}
pub struct KeyTrieNode {
    name: String,
    pub is_sticky: bool,
    map: HashMap<KeyEvent, KeyTrie>,
    pub command: Box<fn()>,
}
/// 键盘映射
pub struct LocalKeymaps {
    /// 不同的 Mode
    pub map: Box<dyn DynAccess<HashMap<Mode, KeyTrie>>>,
    /// 用户输入待处理的事件
    state: Vec<KeyEvent>,
}
pub struct KeyTrie {}
impl KeyTrieNode {
    pub fn new(
        name: &str,
        map: HashMap<KeyEvent, KeyTrie>,
        order: Vec<KeyEvent>,
        command: Box<fn()>,
    ) -> Self {
        Self {
            name: name.to_string(),
            is_sticky: false,
            map,
            command,
        }
    }
}

pub fn create_keymap() {
    let _cap = 3;
    let mut _map: HashMap<&str, fn()> = HashMap::new();
    _map.insert("q", crate::commands::reset_terminal_and_exit);
    _map.insert("h", crate::commands::move_curosr_left);
    // BTreeSet
    // _map.insert("l", crate::commands::move_curosr_right(1));

    // let mut _map = ::std::collections::HashMap::with_capacity(_cap);
    // let mut _node = KeyTrieNode::new("Normal mode",_map,_order);
    // let mut _order = ::std::vec::Vec::with_capacity(_cap);
}
