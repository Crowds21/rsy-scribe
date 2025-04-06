use std::borrow::Cow;
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Severity {
    Hint,
    Info,
    Warning,
    Error,
}
pub struct StatusMessage {
    pub severity: Severity,
    pub message: Cow<'static, str>,
}
