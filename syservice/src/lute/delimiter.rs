use crate::lute::node::Node;

pub struct Delimiter {
    node:Node,          // 对应的AST节点
    typ: u8,                           // 分隔符类型 [*_~
    num: usize,                        // 当前有效分隔符数量
    original_num: usize,               // 原始分隔符数量
    can_open: bool,                    // 可作为开分隔符
    can_close: bool,                   // 可作为闭分隔符

    // 双向链表结构（使用Weak避免循环引用）
    previous:Box<Delimiter>,
    next: Box<Delimiter>,

    // 状态标志
    active: bool,
    is_image: bool,                    // 避免与标准库image冲突
    bracket_after: bool,
    index: usize,

    // 前一个分隔符引用
    previous_delimiter:Box<Delimiter>,
}