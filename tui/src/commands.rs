use std::num::NonZeroUsize;
use view::editor::EditorModel;

///  执行命令时,所涉及到的上下文
pub struct CommandContext<'a> {
    pub register: Option<char>,
    /// 命令执行的次数
    pub count: Option<NonZeroUsize>,
    /// 与 Compositor 使用同一个对象
    pub editor: &'a mut EditorModel,
    
    pub callback: Vec<crate::compositor::Callback>,
    // pub on_next_key_callback: Option<(OnKeyCallback, OnKeyCallbackKind)>,
    // pub jobs: &'a mut Jobs,
}
