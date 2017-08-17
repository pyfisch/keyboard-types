//! Contains types to define keyboard related events.
//!
//! The naming and conventions follow the UI Events specification
//! but this crate should be useful for anyone implementing keyboard
//! input in a cross-platform way.

pub use code::Code;
pub use key::Key;
pub use location::Location;
pub use modifiers::Modifiers;

#[macro_use]
extern crate bitflags;
#[cfg(feature = "heap_size")]
#[macro_use]
extern crate heapsize;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

mod code;
#[cfg(feature = "heap_size")]
mod heap_size;
mod key;
mod location;
pub mod modifiers;

/// Describes the state the key is in.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

/// Keyboard events are issued for all pressed and released keys.
#[derive(Clone, Debug, Eq, PartialEq)]
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
    /// and instead composition events should be used.
    pub is_composing: bool,
}

/// Describes the state of a composition session.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CompositionState {
    /// In JS: "compositionstart" event.
    Start,
    /// In JS: "compositionupdate" event.
    Update,
    /// In JS: "compositionend" event.
    ///
    /// In a text editor in this state the data
    /// should be added to the input.
    End,
}

/// Event to expose input methods to program logic.
///
/// Provides information about entered sequences from
/// dead key combinations and IMEs.
///
/// A composition session is always started by a "compositionstart"
/// event followed my zero or more "compositionupdate" events
/// and terminated by a single "compositionend" event.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CompositionEvent {
    /// Describes the event kind.
    pub state: CompositionState,
    /// Current composition data. May be empty.
    pub data: String,
}
