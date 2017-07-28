//! Contains types to define keyboard related events.
//!
//! The naming and conventions follow the UI Events specification
//! but this crate should be useful for anyone implementing keyboard
//! input in a cross-platform way.

pub use code::Code;
pub use key::Key;
pub use location::Location;
pub use modifiers::*;

#[macro_use]
extern crate bitflags;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

mod code;
mod key;
mod location;
mod modifiers;

/// Describes the state the key is in.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum State {
    /// Key is pressed.
    ///
    /// In JS: "keydown" event firing.
    Down,
    /// Key is released.
    ///
    /// In JS: "keyup event".
    Up,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyEvent {
    pub state: State,
    pub key: Key,
    pub code: Code,
    pub location: Location,
    pub modifiers: Modifiers,
    pub repeat: bool,
}
