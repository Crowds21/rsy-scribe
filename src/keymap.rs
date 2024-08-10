use arc_swap::{
    access::{DynAccess, DynGuard},
    ArcSwap,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{collections::HashMap, sync::Arc};

use crate::commands;
pub mod keyboard;
pub mod macros;
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Mode {
    Normal = 0,
    Select = 1,
    Insert = 2,
    Command = 3,
}
pub struct Context {}
/// KeyBindgTree的节点
pub struct KeyTrieNode {
    name: String,
    map: HashMap<KeyEvent, KeyTrie>,
    order: Vec<KeyEvent>,
    pub is_sticky: bool,
}

/// KeyBindTree
pub enum KeyTrie {
    MappableCommand(commands::MappableCommand),
    Sequence(Vec<commands::MappableCommand>),
    Node(KeyTrieNode),
}

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
pub struct Keymaps {
    pub map: Box<dyn DynAccess<HashMap<Mode, KeyTrie>>>,
    /// 用户输入待处理的事件
    state: Vec<KeyEvent>,
}
impl Keymaps {
    pub fn new(map: Box<dyn DynAccess<HashMap<Mode, KeyTrie>>>) -> Keymaps {
        return Self {
            map,
            state: Vec::new(),
        };
    }
    /// 通过load函数获取DynAccess中的值
    pub fn map(&self) -> DynGuard<HashMap<Mode, KeyTrie>> {
        self.map.load()
    }

    /// 根据mode获取该类别下的所有快捷键,在通过传入的KeyEvnet过滤
    pub fn resolver_key_event(&mut self, mode: Mode, key: KeyEvent) {
        let keymaps = &*self.map();
        let keymap = &keymaps[&mode];
        if KeyCode::Esc == key.code {
            //TODO: 返回一个KeymapResult::Cancelled
            return;
        }
    }
}

impl Default for Keymaps {
    fn default() -> Self {
        Self::new(Box::new(ArcSwap::new(Arc::new(default()))))
    }
}

pub fn default() -> HashMap<Mode, KeyTrie> {
    let _cap = 3;
    let mut _map: HashMap<&str, fn()> = HashMap::new();
    // _map.insert("q", crate::commands::common::reset_terminal_and_exit);
    // _map.insert("h", crate::commands::common::move_cursor_left);
    let mut keymap: HashMap<Mode, KeyTrie> = HashMap::new();

    // BTreeSet
    // _map.insert("l", crate::commands::move_curosr_right(1));

    // let mut _map = ::std::collections::HashMap::with_capacity(_cap);
    // let mut _node = KeyTrieNode::new("Normal mode",_map,_order);
    // let mut _order = ::std::vec::Vec::with_capacity(_cap);
    let normal_mode = KeyTrie::Node(KeyTrieNode {
        name: "Normal mode".to_owned(),
        is_sticky: false, // 除非在示例中指定了 sticky 属性
        map: {
            let mut m = HashMap::new();
            m.insert(
                KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE), // 假设这是将字符 'i' 转换为 KeyEvent 的方式
                KeyTrie::MappableCommand(commands::MappableCommand::insert_mode),
            );
            // 绑定 "g" 到一个子 KeyTrie
            m.insert(
                KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
                KeyTrie::Node(KeyTrieNode {
                    name: "Goto".to_string(),
                    is_sticky: false,
                    map: {
                        let mut n = HashMap::new();
                        n.insert(
                            KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
                            KeyTrie::MappableCommand(MappableCommand::goto_file_start),
                        );
                        n.insert(
                            KeyEvent::new('e'),
                            KeyTrie::MappableCommand(MappableCommand::goto_file_end),
                        );
                        n
                    },
                    order: vec![KeyEvent::new('g'), KeyEvent::new('e')],
                }),
            );
            // 绑定 "j" 或 "down" 到 move_line_down
            m.insert(
                KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
                KeyTrie::MappableCommand(commands::MappableCommand::move_line_down),
            );
            // 假设有一个处理 "|" 操作的方式，可能需要额外的宏逻辑
            // 这里省略了 "down" 的绑定，因为它需要特殊处理
            m
        },
        order: vec![KeyEvent::new('i'), KeyEvent::new('g'), KeyEvent::new('j')],
    });

    let keymap = normal_mode;
    return keymap;
}
