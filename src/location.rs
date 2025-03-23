/// The location attribute contains an indication of the physical location of the key on the device.
///
/// Certain keys on the keyboard can have the same value, but are in different locations. For
/// instance, the <kbd>Shift</kbd> key can be on the left or right side of the keyboard, or the
/// number keys can be above the letters or on the numpad. This enum allows differentiating them.
///
/// See also [MDN's documentation](https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/location).
#[derive(Copy, Clone, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Location {
    /// The key is in its "normal" location on the keyboard.
    ///
    /// The key activation MUST NOT be distinguished as the left or right
    /// version of the key, and (other than the `NumLock` key) did not
    /// originate from the numeric keypad (or did not originate with a
    /// virtual key corresponding to the numeric keypad).
    ///
    /// This variant is the default, and is also used when the location of the key cannot be
    /// identified.
    ///
    /// # Example
    ///
    /// The <kbd>1</kbd> key above the <kbd>Q</kbd> key on a QWERTY keyboard will use this location.
    ///
    #[doc = include_str!("../docs/keyboard_standard_1_key.svg")]
    ///
    /// <sub>For image attribution, see the [ATTRIBUTION.md] file.</sub>
    ///
    #[doc = concat!(
        "[ATTRIBUTION.md]: https://docs.rs/crate/keyboard-types/",
        env!("CARGO_PKG_VERSION"),
        "/source/docs/ATTRIBUTION.md",
    )]
    #[default]
    Standard = 0x00,

    /// The key activated originated from the left key location (when there
    /// is more than one possible location for this key).
    ///
    /// # Example
    ///
    /// The left <kbd>Shift</kbd> key below the <kbd>Caps Lock</kbd> key on a QWERTY keyboard will
    /// use this location.
    ///
    #[doc = include_str!("../docs/keyboard_left_shift_key.svg")]
    ///
    /// <sub>For image attribution, see the [ATTRIBUTION.md] file.</sub>
    ///
    #[doc = concat!(
        "[ATTRIBUTION.md]: https://docs.rs/crate/keyboard-types/",
        env!("CARGO_PKG_VERSION"),
        "/source/docs/ATTRIBUTION.md",
    )]
    Left = 0x01,

    /// The key activation originated from the right key location (when
    /// there is more than one possible location for this key).
    ///
    /// # Example
    ///
    /// The right <kbd>Shift</kbd> key below the <kbd>Enter</kbd> key on a QWERTY keyboard will use
    /// this location.
    ///
    #[doc = include_str!("../docs/keyboard_right_shift_key.svg")]
    ///
    /// <sub>For image attribution, see the [ATTRIBUTION.md] file.</sub>
    ///
    #[doc = concat!(
        "[ATTRIBUTION.md]: https://docs.rs/crate/keyboard-types/",
        env!("CARGO_PKG_VERSION"),
        "/source/docs/ATTRIBUTION.md",
    )]
    Right = 0x02,

    /// The key activation originated on the numeric keypad or with a virtual
    /// key corresponding to the numeric keypad (when there is more than one
    /// possible location for this key). Note that the `NumLock` key should
    /// always be encoded with a location of `Location::Standard`.
    ///
    /// # Example
    ///
    /// The <kbd>1</kbd> key on the numpad will use this location.
    ///
    #[doc = include_str!("../docs/keyboard_numpad_1_key.svg")]
    ///
    /// <sub>For image attribution, see the [ATTRIBUTION.md] file.</sub>
    ///
    #[doc = concat!(
        "[ATTRIBUTION.md]: https://docs.rs/crate/keyboard-types/",
        env!("CARGO_PKG_VERSION"),
        "/source/docs/ATTRIBUTION.md",
    )]
    Numpad = 0x03,
}
