//! Contains types to define keyboard related events.
//!
//! The naming and conventions follow the UI Events specification
//! but this crate should be useful for anyone implementing keyboard
//! input in a cross-platform way.

#![warn(clippy::doc_markdown)]

pub use crate::code::{Code, UnrecognizedCodeError};
pub use crate::key::{Key, UnrecognizedKeyError};
pub use crate::location::Location;
pub use crate::modifiers::Modifiers;
pub use crate::shortcuts::ShortcutMatcher;

mod code;
mod key;
mod location;
mod modifiers;
mod shortcuts;
#[cfg(feature = "webdriver")]
pub mod webdriver;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Describes the state the key is in.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KeyState {
    /// Key is pressed.
    ///
    /// In JS: "keydown" event firing.
    Down,
    /// Key is released.
    ///
    /// In JS: "keyup event".
    Up,
}

impl KeyState {
    /// The [type] name of the corresponding key event.
    ///
    /// This is either `"keydown"` or `"keyup"`.
    ///
    /// [type]: https://w3c.github.io/uievents/#events-keyboard-types
    pub const fn event_type(self) -> &'static str {
        match self {
            Self::Down => "keydown",
            Self::Up => "keyup",
        }
    }
}

/// Keyboard events are issued for all pressed and released keys.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyboardEvent {
    /// Whether the key is pressed or released.
    pub state: KeyState,
    /// Logical key value.
    pub key: Key,
    /// Physical key position.
    pub code: Code,
    /// Location for keys with multiple instances on common keyboards.
    pub location: Location,
    /// Flags for pressed modifier keys.
    pub modifiers: Modifiers,
    /// True if the key is currently auto-repeated.
    pub repeat: bool,
    /// Events with this flag should be ignored in a text editor
    /// and instead [composition events](CompositionEvent) should be used.
    pub is_composing: bool,
}

/// Describes the state of a composition session.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CompositionState {
    /// The [compositionstart] event.
    ///
    /// See also [the MDN documentation][mdn].
    ///
    /// [compositionstart]: https://w3c.github.io/uievents/#event-type-compositionstart
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event
    Start,
    /// The [compositionupdate] event.
    ///
    /// See also [the MDN documentation][mdn].
    ///
    /// [compositionupdate]: https://w3c.github.io/uievents/#event-type-compositionupdate
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event
    Update,
    /// The [compositionend] event.
    ///
    /// In a text editor, in this state the data should be added to the input.
    ///
    /// See also [the MDN documentation][mdn].
    ///
    /// [compositionend]: https://w3c.github.io/uievents/#event-type-compositionend
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event
    End,
}

impl CompositionState {
    /// The [type] name of the corresponding composition event.
    ///
    /// This is either `"compositionstart"`, `"compositionupdate"` or `"compositionend"`.
    ///
    /// [type]: https://w3c.github.io/uievents/#events-composition-types
    pub const fn event_type(self) -> &'static str {
        match self {
            Self::Start => "compositionstart",
            Self::Update => "compositionupdate",
            Self::End => "compositionend",
        }
    }
}

/// Event to expose input methods to program logic.
///
/// Provides information about entered sequences from
/// dead key combinations and IMEs.
///
/// A composition session is always started by a [`CompositionState::Start`]
/// event followed my zero or more [`CompositionState::Update`] events
/// and terminated by a single [`CompositionState::End`] event.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompositionEvent {
    /// Describes the event kind.
    pub state: CompositionState,
    /// Current composition data. May be empty.
    pub data: String,
}

impl Key {
    /// Determine a *charCode* value for a key with a character value.
    ///
    /// For all other keys the value is zero.
    /// The *charCode* is an implementation specific legacy property of DOM keyboard events.
    ///
    /// Specification: <https://w3c.github.io/uievents/#dom-keyboardevent-charcode>
    pub fn legacy_charcode(&self) -> u32 {
        // Spec: event.charCode = event.key.charCodeAt(0)
        // otherwise 0
        match self {
            Key::Character(ref c) => c.chars().next().unwrap_or('\0') as u32,
            _ => 0,
        }
    }

    /// Determine a *keyCode* value for a key.
    ///
    /// The *keyCode* is an implementation specific legacy property of DOM keyboard events.
    ///
    /// Specification: <https://w3c.github.io/uievents/#dom-keyboardevent-keycode>
    pub fn legacy_keycode(&self) -> u32 {
        match self {
            // See: https://w3c.github.io/uievents/#fixed-virtual-key-codes
            Key::Backspace => 8,
            Key::Tab => 9,
            Key::Enter => 13,
            Key::Shift => 16,
            Key::Control => 17,
            Key::Alt => 18,
            Key::CapsLock => 20,
            Key::Escape => 27,
            Key::PageUp => 33,
            Key::PageDown => 34,
            Key::End => 35,
            Key::Home => 36,
            Key::ArrowLeft => 37,
            Key::ArrowUp => 38,
            Key::ArrowRight => 39,
            Key::ArrowDown => 40,
            Key::Delete => 46,
            Key::Character(ref c) if c.len() == 1 => match first_char(c) {
                ' ' => 32,
                x @ '0'..='9' => x as u32,
                x @ 'a'..='z' => x.to_ascii_uppercase() as u32,
                x @ 'A'..='Z' => x as u32,
                // See: https://w3c.github.io/uievents/#optionally-fixed-virtual-key-codes
                ';' | ':' => 186,
                '=' | '+' => 187,
                ',' | '<' => 188,
                '-' | '_' => 189,
                '.' | '>' => 190,
                '/' | '?' => 191,
                '`' | '~' => 192,
                '[' | '{' => 219,
                '\\' | '|' => 220,
                ']' | '}' => 221,
                '\'' | '\"' => 222,
                _ => 0,
            },
            _ => 0,
        }
    }
}

impl Default for KeyState {
    fn default() -> KeyState {
        KeyState::Down
    }
}

impl Default for Key {
    fn default() -> Key {
        Key::Unidentified
    }
}

impl Default for Code {
    fn default() -> Code {
        Code::Unidentified
    }
}

impl Default for Location {
    fn default() -> Location {
        Location::Standard
    }
}

/// Return the first codepoint of a string.
///
/// # Panics
/// Panics if the string is empty.
fn first_char(s: &str) -> char {
    s.chars().next().expect("empty string")
}
