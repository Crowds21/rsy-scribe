//! 代码块处理模块
//!
//! 提供代码块的解析、渲染和语法高亮功能。
//!
//! # Architecture
//!
//! ```text
//! NodeCodeBlock → CodeBlockModel → Rendered Lines
//! ```
//!
//! # Example
//!
//! ```rust,ignore
//! let code_block = CodeBlockModel::from_node(&node);
//! let lines = render_code_block(&code_block, width);
//! ```

pub mod model;
pub mod parser;
pub mod renderer;
pub mod highlight;

pub use model::CodeBlockModel;
pub use parser::parse_code_block;
pub use renderer::{render_code_block, get_code_block_style, CodeBlockStyle};
pub use highlight::{SyntaxHighlighter, StyledLine, StyledSpan, NoOpHighlighter};
