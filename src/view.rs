#[derive(Debug)]
/// Info box used in editor. Rendering logic will be in other crate.
pub struct Info {
    /// Title shown at top.
    pub title: String,
    /// Text body, should contain newlines.
    pub text: String,
    /// Body width.
    pub width: u16,
    /// Body height.
    pub height: u16,
}
