use anyhow::{anyhow, Error};
use arc_swap::{
    access::{DynAccess, DynGuard},
    ArcSwap,
};
use crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, ModifierKeyCode,
};
use keyboard::keys;
use std::{collections::HashMap, sync::Arc};

use crate::commands::MappableCommand;
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

#[derive(Debug)]
pub struct KeyTrieNode {
    name: String,
    map: HashMap<KeyEvent, KeyTrie>,
    order: Vec<KeyEvent>,
    pub is_sticky: bool,
}

/// KeyBindTree
#[derive(Debug)]
pub enum KeyTrie {
    MappableCommand(MappableCommand),
    Sequence(Vec<MappableCommand>),
    Node(KeyTrieNode),
}

impl KeyTrieNode {
    pub fn new(name: &str, map: HashMap<KeyEvent, KeyTrie>, order: Vec<KeyEvent>) -> Self {
        Self {
            name: name.to_string(),
            is_sticky: false,
            map,
            order,
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
    let keymap: HashMap<Mode, KeyTrie> = HashMap::new();

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
                KeyTrie::MappableCommand(MappableCommand::insert_mode),
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
                            KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
                            KeyTrie::MappableCommand(MappableCommand::goto_file_end),
                        );
                        n
                    },
                    order: vec![
                        KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE),
                        KeyEvent::new(KeyCode::Char('e'), KeyModifiers::NONE),
                    ],
                }),
            );
            // 绑定 "j" 或 "down" 到 move_line_down
            m.insert(
                KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
                KeyTrie::MappableCommand(MappableCommand::move_cursor_down),
            );
            // 假设有一个处理 "|" 操作的方式，可能需要额外的宏逻辑
            // 这里省略了 "down" 的绑定，因为它需要特殊处理
            m
        },
        order: vec![
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE),
        ],
    });

    let mut keymap = HashMap::new();
    keymap.insert(Mode::Insert, normal_mode);
    return keymap;
}

/// Keymap 宏会调用该函数来将 "字符串"映射到对应的`KeyCode`;
#[allow(dead_code)]
fn str_to_keycode(s: &str) -> Result<KeyEvent, Error> {
    let mut tokens: Vec<_> = s.split('-').collect();
    let mut code = match tokens.pop().ok_or_else(|| anyhow!("Missing key code"))? {
        keys::BACKSPACE => KeyCode::Backspace,
        keys::ENTER => KeyCode::Enter,
        keys::LEFT => KeyCode::Left,
        keys::RIGHT => KeyCode::Right,
        keys::UP => KeyCode::Up,
        keys::DOWN => KeyCode::Down,
        keys::HOME => KeyCode::Home,
        keys::END => KeyCode::End,
        keys::PAGEUP => KeyCode::PageUp,
        keys::PAGEDOWN => KeyCode::PageDown,
        keys::TAB => KeyCode::Tab,
        keys::DELETE => KeyCode::Delete,
        keys::INSERT => KeyCode::Insert,
        keys::NULL => KeyCode::Null,
        keys::ESC => KeyCode::Esc,
        keys::SPACE => KeyCode::Char(' '),
        keys::MINUS => KeyCode::Char('-'),
        keys::LESS_THAN => KeyCode::Char('<'),
        keys::GREATER_THAN => KeyCode::Char('>'),
        keys::CAPS_LOCK => KeyCode::CapsLock,
        keys::SCROLL_LOCK => KeyCode::ScrollLock,
        keys::NUM_LOCK => KeyCode::NumLock,
        keys::PRINT_SCREEN => KeyCode::PrintScreen,
        keys::PAUSE => KeyCode::Pause,
        keys::MENU => KeyCode::Menu,
        keys::KEYPAD_BEGIN => KeyCode::KeypadBegin,
        keys::LEFT_SHIFT => KeyCode::Modifier(ModifierKeyCode::LeftShift),
        keys::LEFT_CONTROL => KeyCode::Modifier(ModifierKeyCode::LeftControl),
        keys::LEFT_ALT => KeyCode::Modifier(ModifierKeyCode::LeftAlt),
        keys::LEFT_SUPER => KeyCode::Modifier(ModifierKeyCode::LeftSuper),
        keys::LEFT_HYPER => KeyCode::Modifier(ModifierKeyCode::LeftHyper),
        keys::LEFT_META => KeyCode::Modifier(ModifierKeyCode::LeftMeta),
        keys::RIGHT_SHIFT => KeyCode::Modifier(ModifierKeyCode::RightShift),
        keys::RIGHT_CONTROL => KeyCode::Modifier(ModifierKeyCode::RightControl),
        keys::RIGHT_ALT => KeyCode::Modifier(ModifierKeyCode::RightAlt),
        keys::RIGHT_SUPER => KeyCode::Modifier(ModifierKeyCode::RightSuper),
        keys::RIGHT_HYPER => KeyCode::Modifier(ModifierKeyCode::RightHyper),
        keys::RIGHT_META => KeyCode::Modifier(ModifierKeyCode::RightMeta),
        keys::ISO_LEVEL_3_SHIFT => KeyCode::Modifier(ModifierKeyCode::IsoLevel3Shift),
        keys::ISO_LEVEL_5_SHIFT => KeyCode::Modifier(ModifierKeyCode::IsoLevel5Shift),
        single if single.chars().count() == 1 => KeyCode::Char(single.chars().next().unwrap()),
        function if function.len() > 1 && function.starts_with('F') => {
            let function: String = function.chars().skip(1).collect();
            let function = str::parse::<u8>(&function)?;
            (function > 0 && function < 25)
                .then_some(KeyCode::F(function))
                .ok_or_else(|| anyhow!("Invalid function key '{}'", function))?
        }
        invalid => return Err(anyhow!("Invalid key code '{}'", invalid)),
    };

    let mut modifiers = KeyModifiers::empty();
    for token in tokens {
        let flag = match token {
            "S" => KeyModifiers::SHIFT,
            "A" => KeyModifiers::ALT,
            "C" => KeyModifiers::CONTROL,
            _ => return Err(anyhow!("Invalid key modifier '{}-'", token)),
        };

        if modifiers.contains(flag) {
            return Err(anyhow!("Repeated key modifier '{}-'", token));
        }
        modifiers.insert(flag);
    }

    // Normalize character keys so that characters like C-S-r and C-R
    // are represented by equal KeyEvents.
    match code {
        KeyCode::Char(ch) if ch.is_ascii_lowercase() && modifiers.contains(KeyModifiers::SHIFT) => {
            code = KeyCode::Char(ch.to_ascii_uppercase());
            modifiers.remove(KeyModifiers::SHIFT);
        }
        _ => (),
    }
    let kind = KeyEventKind::Press;
    let state = KeyEventState::NONE;
    Ok(KeyEvent {
        code,
        modifiers,
        kind,
        state,
    })
}

#[cfg(test)]
mod tests {
    use super::macros::hashmap;
    use super::macros::keymap;
    use super::*;

    #[test]
    fn test_macro_keymap() {
        let normal = keymap!({"Normal mode"
            "h" | "left" => move_cursor_left,
            "j" | "down" => move_cursor_down,
            "k" | "up" => move_cursor_up,
            "l" | "right" => move_cursor_right,
            "v" => select_mode,
            "G" => goto_line,
            "g" => { "Goto"
                "g" => goto_file_start,
                "e" => goto_word_end,
            },
        });

        //println!("Normal-KeyTire{:#?}", normal);
        assert!(
            matches!(normal, KeyTrie::Node(_)),
            "Test NormalKeyTrie type"
        );
        let key_trie_node = match normal {
            KeyTrie::Node(n) => n,
            _ => panic!("Expected Node"),
        };
        assert_eq!(key_trie_node.name, "Normal mode", "Test NormalKeyTrie name");

        let key_event_j = key_trie_node
            .map
            .get(&KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
        assert!(key_event_j.is_some(), "Get key event `j`");
        let key_event_down = key_trie_node
            .map
            .get(&KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert!(key_event_down.is_some(), "Get key event `g`");
        let key_event_g = key_trie_node
            .map
            .get(&KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
        assert!(key_event_g.is_some(), "Get key event `g`");
    }
}
