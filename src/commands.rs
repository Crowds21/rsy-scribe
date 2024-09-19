use common::*;
use crossterm::event::KeyEvent;

pub mod common;

pub struct Context {}

#[derive(Debug)]
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

impl MappableCommand {
    /// Static的一个实例,通过注解允许不遵循rust命名规范
    #[allow(non_upper_case_globals)]
    pub const insert_mode: Self = Self::Static {
        name: "insert_mode",
        fun: insert_mode,
        doc: "Insert mode",
    };
    #[allow(non_upper_case_globals)]
    pub const select_mode: Self = Self::Static {
        name: "select_mode",
        fun: select_mode,
        doc: "Insert mode",
    };
    #[allow(non_upper_case_globals)]
    pub const move_cursor_left: Self = Self::Static {
        name: "move_cursor_left",
        fun: move_cursor_left,
        doc: "Move left",
    };
    #[allow(non_upper_case_globals)]
    pub const move_cursor_right: Self = Self::Static {
        name: "move_cursor_right",
        fun: move_cursor_right,
        doc: "Move right",
    };
    #[allow(non_upper_case_globals)]
    pub const move_cursor_down: Self = Self::Static {
        name: "move_cursor_down",
        fun: move_cursor_down,
        doc: "Move down",
    };
    #[allow(non_upper_case_globals)]
    pub const move_cursor_up: Self = Self::Static {
        name: "move_cursor_up",
        fun: move_cursor_up,
        doc: "Move up",
    };
    #[allow(non_upper_case_globals)]
    pub const goto_file_start: Self = Self::Static {
        name: "goto_file_start",
        fun: goto_file_start,
        doc: "Goto file start",
    };
    #[allow(non_upper_case_globals)]
    pub const goto_line: Self = Self::Static {
        name: "Goto line",
        fun: goto_line,
        doc: "Goto line",
    };
    #[allow(non_upper_case_globals)]
    pub const goto_file_end: Self = Self::Static {
        name: "goto_file_end",
        fun: goto_file_end,
        doc: "Goto file end",
    };
    #[allow(non_upper_case_globals)]
    pub const goto_word_end: Self = Self::Static {
        name: "goto_word_end",
        fun: goto_word_end,
        doc: "Goto word end",
    };
}
