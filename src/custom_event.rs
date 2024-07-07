use crate::keymap::keyboard::{KeyCode, KeyModifiers};
use serde::de::{self, Deserialize, Deserializer};
use std::fmt;
use unicode_width::UnicodeWidthStr;

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
// We use a newtype here because we want to customize Deserialize and Display.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
    // TODO: crossterm now supports kind & state if terminal supports kitty's extended protocol
}

impl KeyEvent {
    /// If a character was pressed, return it.
    pub fn char(&self) -> Option<char> {
        match self.code {
            KeyCode::Char(ch) => Some(ch),
            _ => None,
        }
    }

    /// Format the key in such a way that a concatenated sequence
    /// of keys can be read easily.
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use helix_view::input::KeyEvent;
    ///
    /// let k = KeyEvent::from_str("w").unwrap().key_sequence_format();
    /// assert_eq!(k, "w");
    ///
    /// let k = KeyEvent::from_str("C-w").unwrap().key_sequence_format();
    /// assert_eq!(k, "<C-w>");
    ///
    /// let k = KeyEvent::from_str(" ").unwrap().key_sequence_format();
    /// assert_eq!(k, "<space>");
    /// ```
    pub fn key_sequence_format(&self) -> String {
        let s = self.to_string();
        if s.graphemes(true).count() > 1 {
            format!("<{}>", s)
        } else {
            s
        }
    }
}

impl fmt::Display for KeyEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}{}{}",
            if self.modifiers.contains(KeyModifiers::SHIFT) {
                "S-"
            } else {
                ""
            },
            if self.modifiers.contains(KeyModifiers::ALT) {
                "A-"
            } else {
                ""
            },
            if self.modifiers.contains(KeyModifiers::CONTROL) {
                "C-"
            } else {
                ""
            },
        ))?;
        match self.code {
            KeyCode::Backspace => f.write_str(keys::BACKSPACE)?,
            KeyCode::Enter => f.write_str(keys::ENTER)?,
            KeyCode::Left => f.write_str(keys::LEFT)?,
            KeyCode::Right => f.write_str(keys::RIGHT)?,
            KeyCode::Up => f.write_str(keys::UP)?,
            KeyCode::Down => f.write_str(keys::DOWN)?,
            KeyCode::Home => f.write_str(keys::HOME)?,
            KeyCode::End => f.write_str(keys::END)?,
            KeyCode::PageUp => f.write_str(keys::PAGEUP)?,
            KeyCode::PageDown => f.write_str(keys::PAGEDOWN)?,
            KeyCode::Tab => f.write_str(keys::TAB)?,
            KeyCode::Delete => f.write_str(keys::DELETE)?,
            KeyCode::Insert => f.write_str(keys::INSERT)?,
            KeyCode::Null => f.write_str(keys::NULL)?,
            KeyCode::Esc => f.write_str(keys::ESC)?,
            KeyCode::Char(' ') => f.write_str(keys::SPACE)?,
            KeyCode::Char('-') => f.write_str(keys::MINUS)?,
            KeyCode::Char('<') => f.write_str(keys::LESS_THAN)?,
            KeyCode::Char('>') => f.write_str(keys::GREATER_THAN)?,
            KeyCode::F(i) => f.write_fmt(format_args!("F{}", i))?,
            KeyCode::Char(c) => f.write_fmt(format_args!("{}", c))?,
            KeyCode::CapsLock => f.write_str(keys::CAPS_LOCK)?,
            KeyCode::ScrollLock => f.write_str(keys::SCROLL_LOCK)?,
            KeyCode::NumLock => f.write_str(keys::NUM_LOCK)?,
            KeyCode::PrintScreen => f.write_str(keys::PRINT_SCREEN)?,
            KeyCode::Pause => f.write_str(keys::PAUSE)?,
            KeyCode::Menu => f.write_str(keys::MENU)?,
            KeyCode::KeypadBegin => f.write_str(keys::KEYPAD_BEGIN)?,
            KeyCode::Media(MediaKeyCode::Play) => f.write_str(keys::PLAY)?,
            KeyCode::Media(MediaKeyCode::Pause) => f.write_str(keys::PAUSE_MEDIA)?,
            KeyCode::Media(MediaKeyCode::PlayPause) => f.write_str(keys::PLAY_PAUSE)?,
            KeyCode::Media(MediaKeyCode::Stop) => f.write_str(keys::STOP)?,
            KeyCode::Media(MediaKeyCode::Reverse) => f.write_str(keys::REVERSE)?,
            KeyCode::Media(MediaKeyCode::FastForward) => f.write_str(keys::FAST_FORWARD)?,
            KeyCode::Media(MediaKeyCode::Rewind) => f.write_str(keys::REWIND)?,
            KeyCode::Media(MediaKeyCode::TrackNext) => f.write_str(keys::TRACK_NEXT)?,
            KeyCode::Media(MediaKeyCode::TrackPrevious) => f.write_str(keys::TRACK_PREVIOUS)?,
            KeyCode::Media(MediaKeyCode::Record) => f.write_str(keys::RECORD)?,
            KeyCode::Media(MediaKeyCode::LowerVolume) => f.write_str(keys::LOWER_VOLUME)?,
            KeyCode::Media(MediaKeyCode::RaiseVolume) => f.write_str(keys::RAISE_VOLUME)?,
            KeyCode::Media(MediaKeyCode::MuteVolume) => f.write_str(keys::MUTE_VOLUME)?,
            KeyCode::Modifier(ModifierKeyCode::LeftShift) => f.write_str(keys::LEFT_SHIFT)?,
            KeyCode::Modifier(ModifierKeyCode::LeftControl) => f.write_str(keys::LEFT_CONTROL)?,
            KeyCode::Modifier(ModifierKeyCode::LeftAlt) => f.write_str(keys::LEFT_ALT)?,
            KeyCode::Modifier(ModifierKeyCode::LeftSuper) => f.write_str(keys::LEFT_SUPER)?,
            KeyCode::Modifier(ModifierKeyCode::LeftHyper) => f.write_str(keys::LEFT_HYPER)?,
            KeyCode::Modifier(ModifierKeyCode::LeftMeta) => f.write_str(keys::LEFT_META)?,
            KeyCode::Modifier(ModifierKeyCode::RightShift) => f.write_str(keys::RIGHT_SHIFT)?,
            KeyCode::Modifier(ModifierKeyCode::RightControl) => f.write_str(keys::RIGHT_CONTROL)?,
            KeyCode::Modifier(ModifierKeyCode::RightAlt) => f.write_str(keys::RIGHT_ALT)?,
            KeyCode::Modifier(ModifierKeyCode::RightSuper) => f.write_str(keys::RIGHT_SUPER)?,
            KeyCode::Modifier(ModifierKeyCode::RightHyper) => f.write_str(keys::RIGHT_HYPER)?,
            KeyCode::Modifier(ModifierKeyCode::RightMeta) => f.write_str(keys::RIGHT_META)?,
            KeyCode::Modifier(ModifierKeyCode::IsoLevel3Shift) => {
                f.write_str(keys::ISO_LEVEL_3_SHIFT)?
            }
            KeyCode::Modifier(ModifierKeyCode::IsoLevel5Shift) => {
                f.write_str(keys::ISO_LEVEL_5_SHIFT)?
            }
        };
        Ok(())
    }
}

impl UnicodeWidthStr for KeyEvent {
    fn width(&self) -> usize {
        use helix_core::unicode::width::UnicodeWidthChar;
        let mut width = match self.code {
            KeyCode::Backspace => keys::BACKSPACE.len(),
            KeyCode::Enter => keys::ENTER.len(),
            KeyCode::Left => keys::LEFT.len(),
            KeyCode::Right => keys::RIGHT.len(),
            KeyCode::Up => keys::UP.len(),
            KeyCode::Down => keys::DOWN.len(),
            KeyCode::Home => keys::HOME.len(),
            KeyCode::End => keys::END.len(),
            KeyCode::PageUp => keys::PAGEUP.len(),
            KeyCode::PageDown => keys::PAGEDOWN.len(),
            KeyCode::Tab => keys::TAB.len(),
            KeyCode::Delete => keys::DELETE.len(),
            KeyCode::Insert => keys::INSERT.len(),
            KeyCode::Null => keys::NULL.len(),
            KeyCode::Esc => keys::ESC.len(),
            KeyCode::Char(' ') => keys::SPACE.len(),
            KeyCode::Char('-') => keys::MINUS.len(),
            KeyCode::F(1..=9) => 2,
            KeyCode::F(_) => 3,
            KeyCode::Char(c) => c.width().unwrap_or(0),
            KeyCode::CapsLock => keys::CAPS_LOCK.len(),
            KeyCode::ScrollLock => keys::SCROLL_LOCK.len(),
            KeyCode::NumLock => keys::NUM_LOCK.len(),
            KeyCode::PrintScreen => keys::PRINT_SCREEN.len(),
            KeyCode::Pause => keys::PAUSE.len(),
            KeyCode::Menu => keys::MENU.len(),
            KeyCode::KeypadBegin => keys::KEYPAD_BEGIN.len(),
            KeyCode::Media(MediaKeyCode::Play) => keys::PLAY.len(),
            KeyCode::Media(MediaKeyCode::Pause) => keys::PAUSE_MEDIA.len(),
            KeyCode::Media(MediaKeyCode::PlayPause) => keys::PLAY_PAUSE.len(),
            KeyCode::Media(MediaKeyCode::Stop) => keys::STOP.len(),
            KeyCode::Media(MediaKeyCode::Reverse) => keys::REVERSE.len(),
            KeyCode::Media(MediaKeyCode::FastForward) => keys::FAST_FORWARD.len(),
            KeyCode::Media(MediaKeyCode::Rewind) => keys::REWIND.len(),
            KeyCode::Media(MediaKeyCode::TrackNext) => keys::TRACK_NEXT.len(),
            KeyCode::Media(MediaKeyCode::TrackPrevious) => keys::TRACK_PREVIOUS.len(),
            KeyCode::Media(MediaKeyCode::Record) => keys::RECORD.len(),
            KeyCode::Media(MediaKeyCode::LowerVolume) => keys::LOWER_VOLUME.len(),
            KeyCode::Media(MediaKeyCode::RaiseVolume) => keys::RAISE_VOLUME.len(),
            KeyCode::Media(MediaKeyCode::MuteVolume) => keys::MUTE_VOLUME.len(),
            KeyCode::Modifier(ModifierKeyCode::LeftShift) => keys::LEFT_SHIFT.len(),
            KeyCode::Modifier(ModifierKeyCode::LeftControl) => keys::LEFT_CONTROL.len(),
            KeyCode::Modifier(ModifierKeyCode::LeftAlt) => keys::LEFT_ALT.len(),
            KeyCode::Modifier(ModifierKeyCode::LeftSuper) => keys::LEFT_SUPER.len(),
            KeyCode::Modifier(ModifierKeyCode::LeftHyper) => keys::LEFT_HYPER.len(),
            KeyCode::Modifier(ModifierKeyCode::LeftMeta) => keys::LEFT_META.len(),
            KeyCode::Modifier(ModifierKeyCode::RightShift) => keys::RIGHT_SHIFT.len(),
            KeyCode::Modifier(ModifierKeyCode::RightControl) => keys::RIGHT_CONTROL.len(),
            KeyCode::Modifier(ModifierKeyCode::RightAlt) => keys::RIGHT_ALT.len(),
            KeyCode::Modifier(ModifierKeyCode::RightSuper) => keys::RIGHT_SUPER.len(),
            KeyCode::Modifier(ModifierKeyCode::RightHyper) => keys::RIGHT_HYPER.len(),
            KeyCode::Modifier(ModifierKeyCode::RightMeta) => keys::RIGHT_META.len(),
            KeyCode::Modifier(ModifierKeyCode::IsoLevel3Shift) => keys::ISO_LEVEL_3_SHIFT.len(),
            KeyCode::Modifier(ModifierKeyCode::IsoLevel5Shift) => keys::ISO_LEVEL_5_SHIFT.len(),
        };
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            width += 2;
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            width += 2;
        }
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            width += 2;
        }
        width
    }

    fn width_cjk(&self) -> usize {
        self.width()
    }
}

impl std::str::FromStr for KeyEvent {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens: Vec<_> = s.split('-').collect();
        let mut code = match tokens.pop().ok_or_else(|| anyhow!("Missing key code"))? {
            keys::BACKSPACE => KeyCode::Backspace,
            keys::ENTER => KeyCode::Enter,
            keys::LEFT => KeyCode::Left,
            keys::RIGHT => KeyCode::Right,
            keys::UP => KeyCode::Up,
            keys::DOWN => KeyCode::Down,
            keys::HOME => KeyCode::Home,
            keys::END => KeyCode::End,
            keys::PAGEUP => KeyCode::PageUp,
            keys::PAGEDOWN => KeyCode::PageDown,
            keys::TAB => KeyCode::Tab,
            keys::DELETE => KeyCode::Delete,
            keys::INSERT => KeyCode::Insert,
            keys::NULL => KeyCode::Null,
            keys::ESC => KeyCode::Esc,
            keys::SPACE => KeyCode::Char(' '),
            keys::MINUS => KeyCode::Char('-'),
            keys::LESS_THAN => KeyCode::Char('<'),
            keys::GREATER_THAN => KeyCode::Char('>'),
            keys::CAPS_LOCK => KeyCode::CapsLock,
            keys::SCROLL_LOCK => KeyCode::ScrollLock,
            keys::NUM_LOCK => KeyCode::NumLock,
            keys::PRINT_SCREEN => KeyCode::PrintScreen,
            keys::PAUSE => KeyCode::Pause,
            keys::MENU => KeyCode::Menu,
            keys::KEYPAD_BEGIN => KeyCode::KeypadBegin,
            keys::PLAY => KeyCode::Media(MediaKeyCode::Play),
            keys::PAUSE_MEDIA => KeyCode::Media(MediaKeyCode::Pause),
            keys::PLAY_PAUSE => KeyCode::Media(MediaKeyCode::PlayPause),
            keys::STOP => KeyCode::Media(MediaKeyCode::Stop),
            keys::REVERSE => KeyCode::Media(MediaKeyCode::Reverse),
            keys::FAST_FORWARD => KeyCode::Media(MediaKeyCode::FastForward),
            keys::REWIND => KeyCode::Media(MediaKeyCode::Rewind),
            keys::TRACK_NEXT => KeyCode::Media(MediaKeyCode::TrackNext),
            keys::TRACK_PREVIOUS => KeyCode::Media(MediaKeyCode::TrackPrevious),
            keys::RECORD => KeyCode::Media(MediaKeyCode::Record),
            keys::LOWER_VOLUME => KeyCode::Media(MediaKeyCode::LowerVolume),
            keys::RAISE_VOLUME => KeyCode::Media(MediaKeyCode::RaiseVolume),
            keys::MUTE_VOLUME => KeyCode::Media(MediaKeyCode::MuteVolume),
            keys::LEFT_SHIFT => KeyCode::Modifier(ModifierKeyCode::LeftShift),
            keys::LEFT_CONTROL => KeyCode::Modifier(ModifierKeyCode::LeftControl),
            keys::LEFT_ALT => KeyCode::Modifier(ModifierKeyCode::LeftAlt),
            keys::LEFT_SUPER => KeyCode::Modifier(ModifierKeyCode::LeftSuper),
            keys::LEFT_HYPER => KeyCode::Modifier(ModifierKeyCode::LeftHyper),
            keys::LEFT_META => KeyCode::Modifier(ModifierKeyCode::LeftMeta),
            keys::RIGHT_SHIFT => KeyCode::Modifier(ModifierKeyCode::RightShift),
            keys::RIGHT_CONTROL => KeyCode::Modifier(ModifierKeyCode::RightControl),
            keys::RIGHT_ALT => KeyCode::Modifier(ModifierKeyCode::RightAlt),
            keys::RIGHT_SUPER => KeyCode::Modifier(ModifierKeyCode::RightSuper),
            keys::RIGHT_HYPER => KeyCode::Modifier(ModifierKeyCode::RightHyper),
            keys::RIGHT_META => KeyCode::Modifier(ModifierKeyCode::RightMeta),
            keys::ISO_LEVEL_3_SHIFT => KeyCode::Modifier(ModifierKeyCode::IsoLevel3Shift),
            keys::ISO_LEVEL_5_SHIFT => KeyCode::Modifier(ModifierKeyCode::IsoLevel5Shift),
            single if single.chars().count() == 1 => KeyCode::Char(single.chars().next().unwrap()),
            function if function.len() > 1 && function.starts_with('F') => {
                let function: String = function.chars().skip(1).collect();
                let function = str::parse::<u8>(&function)?;
                (function > 0 && function < 25)
                    .then_some(KeyCode::F(function))
                    .ok_or_else(|| anyhow!("Invalid function key '{}'", function))?
            }
            invalid => return Err(anyhow!("Invalid key code '{}'", invalid)),
        };

        let mut modifiers = KeyModifiers::empty();
        for token in tokens {
            let flag = match token {
                "S" => KeyModifiers::SHIFT,
                "A" => KeyModifiers::ALT,
                "C" => KeyModifiers::CONTROL,
                _ => return Err(anyhow!("Invalid key modifier '{}-'", token)),
            };

            if modifiers.contains(flag) {
                return Err(anyhow!("Repeated key modifier '{}-'", token));
            }
            modifiers.insert(flag);
        }

        // Normalize character keys so that characters like C-S-r and C-R
        // are represented by equal KeyEvents.
        match code {
            KeyCode::Char(ch)
                if ch.is_ascii_lowercase() && modifiers.contains(KeyModifiers::SHIFT) =>
            {
                code = KeyCode::Char(ch.to_ascii_uppercase());
                modifiers.remove(KeyModifiers::SHIFT);
            }
            _ => (),
        }

        Ok(KeyEvent { code, modifiers })
    }
}

impl<'de> Deserialize<'de> for KeyEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}

#[cfg(feature = "term")]
impl From<crossterm::event::Event> for Event {
    fn from(event: crossterm::event::Event) -> Self {
        match event {
            crossterm::event::Event::Key(key) => Self::Key(key.into()),
            crossterm::event::Event::Mouse(mouse) => Self::Mouse(mouse.into()),
            crossterm::event::Event::Resize(w, h) => Self::Resize(w, h),
            crossterm::event::Event::FocusGained => Self::FocusGained,
            crossterm::event::Event::FocusLost => Self::FocusLost,
            crossterm::event::Event::Paste(s) => Self::Paste(s),
        }
    }
}
