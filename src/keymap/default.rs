use super::macros::hashmap;
use super::macros::keymap;
use super::{KeyTrie, Mode};
use std::collections::HashMap;
pub fn default_keymap() -> HashMap<Mode, KeyTrie> {
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

        "space" => { "Space"
            "f" => search_in_current_file,
            "F" => search_globally,
        },
        "q" => reset_terminal_and_exit,
        ":" => command_mode,
    });
    hashmap!(
        Mode::Normal => normal
    )
}
