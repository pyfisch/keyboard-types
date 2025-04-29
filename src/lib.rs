//! Contains types to define keyboard related events.
//!
//! The naming and conventions follow the UI Events specification
//! but this crate should be useful for anyone implementing keyboard
//! input in a cross-platform way.

#![warn(clippy::doc_markdown)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

use alloc::string::{String, ToString};
use core::fmt;
use core::str::FromStr;

pub use crate::code::{Code, UnrecognizedCodeError};
pub use crate::location::Location;
pub use crate::modifiers::Modifiers;
pub use crate::named_key::{NamedKey, UnrecognizedNamedKeyError};
pub use crate::shortcuts::ShortcutMatcher;

mod code;
mod location;
mod modifiers;
mod named_key;
mod shortcuts;
#[cfg(feature = "webdriver")]
pub mod webdriver;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Describes the state a key is in.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum KeyState {
    /// The key is pressed down.
    ///
    /// Often emitted in a [keydown] event, see also [the MDN documentation][mdn] on that.
    ///
    /// [keydown]: https://w3c.github.io/uievents/#event-type-keydown
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
    Down,
    /// The key is not pressed / was just released.
    ///
    /// Often emitted in a [keyup] event, see also [the MDN documentation][mdn] on that.
    ///
    /// [keyup]: https://w3c.github.io/uievents/#event-type-keyup
    /// [mdn]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event
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

    /// True if the key is pressed down.
    pub const fn is_down(self) -> bool {
        matches!(self, Self::Down)
    }

    /// True if the key is released.
    pub const fn is_up(self) -> bool {
        matches!(self, Self::Up)
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
/// event followed by zero or more [`CompositionState::Update`] events
/// and terminated by a single [`CompositionState::End`] event.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompositionEvent {
    /// Describes the event kind.
    pub state: CompositionState,
    /// Current composition data. May be empty.
    pub data: String,
}

/// The value received from the keypress.
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Key {
    /// A key string that corresponds to the character typed by the user,
    /// taking into account the user’s current locale setting, modifier state,
    /// and any system-level keyboard mapping overrides that are in effect.
    Character(String),
    Named(NamedKey),
}

/// Parse from string error, returned when string does not match to any [`Key`] variant.
#[derive(Clone, Debug)]
pub struct UnrecognizedKeyError;

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Character(s) => f.write_str(s),
            Self::Named(k) => k.fmt(f),
        }
    }
}

impl FromStr for Key {
    type Err = UnrecognizedKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_key_string(s) {
            Ok(Self::Character(s.to_string()))
        } else {
            Ok(Self::Named(
                NamedKey::from_str(s).map_err(|_| UnrecognizedKeyError)?,
            ))
        }
    }
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
            Key::Named(_) => 0,
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
            Key::Named(NamedKey::Backspace) => 8,
            Key::Named(NamedKey::Tab) => 9,
            Key::Named(NamedKey::Enter) => 13,
            Key::Named(NamedKey::Shift) => 16,
            Key::Named(NamedKey::Control) => 17,
            Key::Named(NamedKey::Alt) => 18,
            Key::Named(NamedKey::CapsLock) => 20,
            Key::Named(NamedKey::Escape) => 27,
            Key::Named(NamedKey::PageUp) => 33,
            Key::Named(NamedKey::PageDown) => 34,
            Key::Named(NamedKey::End) => 35,
            Key::Named(NamedKey::Home) => 36,
            Key::Named(NamedKey::ArrowLeft) => 37,
            Key::Named(NamedKey::ArrowUp) => 38,
            Key::Named(NamedKey::ArrowRight) => 39,
            Key::Named(NamedKey::ArrowDown) => 40,
            Key::Named(NamedKey::Delete) => 46,
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
    fn default() -> Self {
        Self::Named(NamedKey::default())
    }
}

impl Default for NamedKey {
    fn default() -> Self {
        Self::Unidentified
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

/// Check if string can be used as a `Key::Character` _keystring_.
///
/// This check is simple and is meant to prevents common mistakes like mistyped keynames
/// (e.g. `Ennter`) from being recognized as characters.
fn is_key_string(s: &str) -> bool {
    s.chars().all(|c| !c.is_control()) && s.chars().skip(1).all(|c| !c.is_ascii())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_key_string() {
        assert!(is_key_string("A"));
        assert!(!is_key_string("AA"));
        assert!(!is_key_string("	"));
    }
}
