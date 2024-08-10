use common::move_cursor_left;
use crossterm::event::KeyEvent;

pub mod common;

pub struct Context {}
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
    pub const move_char_left: Self = Self::Static {
        name: "move_curosr_left",
        fun: move_cursor_left,
        doc: "Move left",
    };
    #[allow(non_upper_case_globals)]
    pub const insert_mode: Self = Self::Static {
        name: "insert_mode",
        fun: insert_mode,
        doc: "Insert mode",
    };
}
