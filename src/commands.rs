use crossterm::event::KeyEvent;

pub mod common;
pub(crate) mod typed;
use common::*;
use typed::*;

pub struct Context {}

#[derive(Debug, Clone, PartialEq)]
pub enum MappableCommand {
    Typable {
        name: String,
        args: Vec<String>,
        doc: String,
    },
    Static {
        name: &'static str,
        fun: fn(cx: &mut Context),
        doc: &'static str,
    },
    Macro {
        name: String,
        keys: Vec<KeyEvent>,
    },
}

/// 创建 Static_Commands
macro_rules! static_commands {
    ( $($name:ident, $doc:literal,)* ) => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $name: Self = Self::Static {
                name: stringify!($name),
                fun: $name,
                doc: $doc
            };
        )*

        pub const STATIC_COMMAND_LIST: &'static [Self] = &[
            $( Self::$name, )*
        ];
    }
}

impl MappableCommand {
    #[rustfmt::skip]
    static_commands!(
        move_cursor_left, "Move left",
        move_cursor_right, "Move right",
        move_cursor_down, "Move down",
        move_cursor_up, "Move up",
        goto_line, "Goto line",
        goto_word_end, "Goto word end",
        goto_file_start, "Goto file start",
        command_mode, "Enter command mode",
        select_mode,  "Select mode",
    );
}
