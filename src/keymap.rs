use arc_swap::access::DynAccess;

use crate::custom_event::KeyEvent;
use core::str;
use std::collections::HashMap;
pub mod keyboard;
pub mod macros;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal = 0,
    Select = 1,
    Insert = 2,
}
/// 按键字典树中的节点
#[derive(Debug, Clone, Default)]
pub struct KeyTrieNode {
    /// A label for keys coming under this node, like "Goto mode"
    name: String,
    map: HashMap<KeyEvent, KeyTrie>,
    order: Vec<KeyEvent>,
    pub is_sticky: bool,
}

/// 按键字典树中的种类
#[derive(Debug, Clone, PartialEq)]
pub enum KeyTrie {
    // MappableCommand(MappableCommand),
    // Sequence(Vec<MappableCommand>),
    Node(KeyTrieNode),
}
impl PartialEq for KeyTrieNode {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

/// 键盘映射
pub struct Keymaps {
    /// 不同的 Mode
    pub map: Box<dyn DynAccess<HashMap<Mode, KeyTrie>>>,
    /// Stores pending keys waiting for the next key. This is relative to a
    /// sticky node if one is in use.
    state: Vec<KeyEvent>,
    /// Stores the sticky node if one is activated.
    pub sticky: Option<KeyTrieNode>,
}

pub fn create_keymap() {
    let _cap = 3;
    let mut _map = HashMap::new();
    _map.insert("q", crate::commands::reset_terminal_and_exit);
    // let mut _map = ::std::collections::HashMap::with_capacity(_cap);
    // let mut _order = ::std::vec::Vec::with_capacity(_cap);
}
