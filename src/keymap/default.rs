use macros::hashmap;
use macros::keymap;
fn test() {
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
}