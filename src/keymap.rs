use anyhow::{anyhow, Error};
use arc_swap::{
    access::{DynAccess, DynGuard},
    ArcSwap,
};
use crossterm::event::{
    KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, ModifierKeyCode,
};
use keyboard::keys;
use macros::key;
use std::{
    borrow::Cow,
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{application::Mode, commands::MappableCommand};
pub mod keyboard;
pub mod macros;

pub struct Context {}
/// KeyBindgTree的节点

#[derive(Debug, Clone)]
pub struct KeyTrieNode {
    name: String,
    map: HashMap<KeyEvent, KeyTrie>,
    order: Vec<KeyEvent>,
    pub is_sticky: bool,
}

/// KeyBindTree
#[derive(Debug, Clone, PartialEq)]
pub enum KeyTrie {
    MappableCommand(MappableCommand),
    Sequence(Vec<MappableCommand>),
    Node(KeyTrieNode),
}

impl KeyTrie {
    /// [KeyEvent]一个 KeyEvent 类型元素的切片
    pub fn search(&self, keys: &[KeyEvent]) -> Option<&KeyTrie> {
        let mut trie = self;
        for key in keys {
            trie = match trie {
                KeyTrie::Node(map) => map.get(key),
                KeyTrie::MappableCommand(_) | KeyTrie::Sequence(_) => None,
            }?
        }
        Some(trie)
    }
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

impl PartialEq for KeyTrieNode {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

impl Deref for KeyTrieNode {
    type Target = HashMap<KeyEvent, KeyTrie>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for KeyTrieNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum KeymapResult {
    /// Needs more keys to execute a command. Contains valid keys for next keystroke.
    Pending(KeyTrieNode),
    Matched(MappableCommand),
    /// Matched a sequence of commands to execute.
    MatchedSequence(Vec<MappableCommand>),
    /// Key was not found in the root keymap
    NotFound,
    /// Key is invalid in combination with previous keys. Contains keys leading upto
    /// and including current (invalid) key.
    Cancelled(Vec<KeyEvent>),
}
pub struct Keymaps {
    pub map: Box<dyn DynAccess<HashMap<Mode, KeyTrie>>>,
    /// Stores pending keys waiting for the next key. This is relative to a
    /// sticky node if one is in use.
    state: Vec<KeyEvent>,
    /// Stores the sticky node if one is activated.
    pub sticky: Option<KeyTrieNode>,
}
impl Keymaps {
    pub fn new(map: Box<dyn DynAccess<HashMap<Mode, KeyTrie>>>) -> Keymaps {
        return Self {
            map,
            state: Vec::new(),
            sticky: None,
        };
    }
    /// 通过load函数获取DynAccess中的值
    pub fn map(&self) -> DynGuard<HashMap<Mode, KeyTrie>> {
        self.map.load()
    }

    /// Lookup `key` in the keymap to try and find a command to execute. Escape
    /// key cancels pending keystrokes. If there are no pending keystrokes but a
    /// sticky node is in use, it will be cleared.
    pub fn get(&mut self, mode: Mode, key: KeyEvent) -> KeymapResult {
        // TODO: remove the sticky part and look up manually
        let keymaps = &*self.map();
        let keymap = &keymaps[&mode];
        if key!(Esc) == key {
            if !self.state.is_empty() {
                // Note that Esc is not included here
                return KeymapResult::Cancelled(self.state.drain(..).collect());
            }
            self.sticky = None;
        }

        let first = self.state.first().unwrap_or(&key);
        let trie_node = match self.sticky {
            Some(ref trie) => Cow::Owned(KeyTrie::Node(trie.clone())),
            None => Cow::Borrowed(keymap),
        };

        let trie = match trie_node.search(&[*first]) {
            Some(KeyTrie::MappableCommand(ref cmd)) => {
                return KeymapResult::Matched(cmd.clone());
            }
            Some(KeyTrie::Sequence(ref cmds)) => {
                return KeymapResult::MatchedSequence(cmds.clone());
            }
            None => return KeymapResult::NotFound,
            Some(t) => t,
        };

        self.state.push(key);
        match trie.search(&self.state[1..]) {
            Some(KeyTrie::Node(map)) => {
                if map.is_sticky {
                    self.state.clear();
                    self.sticky = Some(map.clone());
                }
                KeymapResult::Pending(map.clone())
            }
            Some(KeyTrie::MappableCommand(cmd)) => {
                self.state.clear();
                KeymapResult::Matched(cmd.clone())
            }
            Some(KeyTrie::Sequence(cmds)) => {
                self.state.clear();
                KeymapResult::MatchedSequence(cmds.clone())
            }
            None => KeymapResult::Cancelled(self.state.drain(..).collect()),
        }
    }
}

pub mod default;
impl Default for Keymaps {
    fn default() -> Self {
        Self::new(Box::new(ArcSwap::new(Arc::new(default::default_keymap()))))
    }
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
