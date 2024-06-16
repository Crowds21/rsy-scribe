#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub enum Event {
    FocusGained,
    FocusLost,
    Key(KeyEvent),
    // Mouse(MouseEvent),
    // Paste(String),
    Resize(u16, u16),
    IdleTimeout,
}

/// Represents a key event.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, Clone, Copy)]
pub struct KeyEvent {
    /// The key itself.
    pub code: KeyCode,
    /// Additional key modifiers.
    pub modifiers: KeyModifiers,
    /// Kind of event.
    ///
    /// Only set if:
    /// - Unix: [`KeyboardEnhancementFlags::REPORT_EVENT_TYPES`] has been enabled with [`PushKeyboardEnhancementFlags`].
    /// - Windows: always
    pub kind: KeyEventKind,
    /// Keyboard state.
    ///
    /// Only set if [`KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES`] has been enabled with
    /// [`PushKeyboardEnhancementFlags`].
    pub state: KeyEventState,
}

/// Represents a keyboard event kind.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum KeyEventKind {
    Press,
    Repeat,
    Release,
}
