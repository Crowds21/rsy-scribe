/// Macro for defining a `KeyTrie`. Example:
///
/// ```
/// # use helix_core::hashmap;
/// # use helix_term::keymap;
/// let normal_mode = keymap!({ "Normal mode"
///     "i" => insert_mode,
///     "g" => { "Goto"
///         "g" => goto_file_start,
///         "e" => goto_file_end,
///     },
///     "j" | "down" => move_line_down,
/// });
/// let keymap = normal_mode;
/// ```
/// 每个宏都有一个明成分
#[macro_export]
macro_rules! keymap {
    (@trie $cmd:ident) => {
        $crate::keymap::KeyTrie::MappableCommand($crate::commands::MappableCommand::$cmd)
    };

    (@trie
        { $label:literal $(sticky=$sticky:literal)? $($($key:literal)|+ => $value:tt,)+ }
    ) => {
        keymap!({ $label $(sticky=$sticky)? $($($key)|+ => $value,)+ })
    };

    (@trie [$($cmd:ident),* $(,)?]) => {
        $crate::keymap::KeyTrie::Sequence(vec![$($crate::commands::Command::$cmd),*])
    };

    (
        { $label:literal $(sticky=$sticky:literal)? $($($key:literal)|+ => $value:tt,)+ }
    ) => {
        // modified from the hashmap! macro
        {
            let _cap = hashmap!(@count $($($key),+),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            let mut _order = ::std::vec::Vec::with_capacity(_cap);
            $(
                $(
                    let _key = $key.parse::<::helix_view::input::KeyEvent>().unwrap();
                    let _duplicate = _map.insert(
                        _key,
                        keymap!(@trie $value)
                    );
                    assert!(_duplicate.is_none(), "Duplicate key found: {:?}", _duplicate.unwrap());
                    _order.push(_key);
                )+
            )*
            let mut _node = $crate::keymap::KeyTrieNode::new($label, _map, _order);
            $( _node.is_sticky = $sticky; )?
            $crate::keymap::KeyTrie::Node(_node)
        }
    };
}
