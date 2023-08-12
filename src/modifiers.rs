//! Modifier key data.
//!
//! Modifier keys like Shift and Control alter the character value
//! and are used in keyboard shortcuts.
//!
//! Use the constants to match for combinations of the modifier keys.

bitflags! {
    /// Pressed modifier keys.
    ///
    /// Specification:
    /// <https://w3c.github.io/uievents-key/#keys-modifier>
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Modifiers: u32 {
        const ALT = 0x01;
        const ALT_GRAPH = 0x2;
        const CAPS_LOCK = 0x4;
        const CONTROL = 0x8;
        const FN = 0x10;
        const FN_LOCK = 0x20;
        const META = 0x40;
        const NUM_LOCK = 0x80;
        const SCROLL_LOCK = 0x100;
        const SHIFT = 0x200;
        const SYMBOL = 0x400;
        const SYMBOL_LOCK = 0x800;
        const HYPER = 0x1000;
        const SUPER = 0x2000;
    }
}
