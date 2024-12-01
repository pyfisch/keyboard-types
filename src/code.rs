
// AUTO GENERATED CODE - DO NOT EDIT
#![cfg_attr(rustfmt, rustfmt_skip)]

use std::fmt::{self, Display};
use std::str::FromStr;
use std::error::Error;

/// Code is the physical position of a key.
///
/// The names are based on the US keyboard. If the key
/// is not present on US keyboards a name from another
/// layout is used.
///
/// Specification:
/// <https://w3c.github.io/uievents-code/>
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum Code {
    /// <kbd>`~</kbd> on a US keyboard. This is the <kbd>半角/全角/漢字</kbd> (<span class="unicode">hankaku/zenkaku/kanji</span>) key on Japanese keyboards
    Backquote,
    /// Used for both the US <kbd>\|</kbd> (on the 101-key layout) and also for the key
    /// located between the <kbd>"</kbd> and <kbd>Enter</kbd> keys on row C of the 102-,
    /// 104- and 106-key layouts.
    /// Labelled <kbd>#~</kbd> on a UK (102) keyboard.
    Backslash,
    /// <kbd>[{</kbd> on a US keyboard.
    BracketLeft,
    /// <kbd>]}</kbd> on a US keyboard.
    BracketRight,
    /// <kbd>,&lt;</kbd> on a US keyboard.
    Comma,
    /// <kbd>0)</kbd> on a US keyboard.
    Digit0,
    /// <kbd>1!</kbd> on a US keyboard.
    Digit1,
    /// <kbd>2@</kbd> on a US keyboard.
    Digit2,
    /// <kbd>3#</kbd> on a US keyboard.
    Digit3,
    /// <kbd>4$</kbd> on a US keyboard.
    Digit4,
    /// <kbd>5%</kbd> on a US keyboard.
    Digit5,
    /// <kbd>6^</kbd> on a US keyboard.
    Digit6,
    /// <kbd>7&amp;</kbd> on a US keyboard.
    Digit7,
    /// <kbd>8*</kbd> on a US keyboard.
    Digit8,
    /// <kbd>9(</kbd> on a US keyboard.
    Digit9,
    /// <kbd>=+</kbd> on a US keyboard.
    Equal,
    /// Located between the left <kbd>Shift</kbd> and <kbd>Z</kbd> keys.
    /// Labelled <kbd>\|</kbd> on a UK keyboard.
    IntlBackslash,
    /// Located between the <kbd>/</kbd> and right <kbd>Shift</kbd> keys.
    /// Labelled <kbd>\ろ</kbd> (<span class="unicode">ro</span>) on a Japanese keyboard.
    IntlRo,
    /// Located between the <kbd>=</kbd> and <kbd>Backspace</kbd> keys.
    /// Labelled <kbd>¥</kbd> (<span class="unicode">yen</span>) on a Japanese keyboard. <kbd>\/</kbd> on a
    /// Russian keyboard.
    IntlYen,
    /// <kbd>a</kbd> on a US keyboard.
    /// Labelled <kbd>q</kbd> on an AZERTY (e.g., French) keyboard.
    KeyA,
    /// <kbd>b</kbd> on a US keyboard.
    KeyB,
    /// <kbd>c</kbd> on a US keyboard.
    KeyC,
    /// <kbd>d</kbd> on a US keyboard.
    KeyD,
    /// <kbd>e</kbd> on a US keyboard.
    KeyE,
    /// <kbd>f</kbd> on a US keyboard.
    KeyF,
    /// <kbd>g</kbd> on a US keyboard.
    KeyG,
    /// <kbd>h</kbd> on a US keyboard.
    KeyH,
    /// <kbd>i</kbd> on a US keyboard.
    KeyI,
    /// <kbd>j</kbd> on a US keyboard.
    KeyJ,
    /// <kbd>k</kbd> on a US keyboard.
    KeyK,
    /// <kbd>l</kbd> on a US keyboard.
    KeyL,
    /// <kbd>m</kbd> on a US keyboard.
    KeyM,
    /// <kbd>n</kbd> on a US keyboard.
    KeyN,
    /// <kbd>o</kbd> on a US keyboard.
    KeyO,
    /// <kbd>p</kbd> on a US keyboard.
    KeyP,
    /// <kbd>q</kbd> on a US keyboard.
    /// Labelled <kbd>a</kbd> on an AZERTY (e.g., French) keyboard.
    KeyQ,
    /// <kbd>r</kbd> on a US keyboard.
    KeyR,
    /// <kbd>s</kbd> on a US keyboard.
    KeyS,
    /// <kbd>t</kbd> on a US keyboard.
    KeyT,
    /// <kbd>u</kbd> on a US keyboard.
    KeyU,
    /// <kbd>v</kbd> on a US keyboard.
    KeyV,
    /// <kbd>w</kbd> on a US keyboard.
    /// Labelled <kbd>z</kbd> on an AZERTY (e.g., French) keyboard.
    KeyW,
    /// <kbd>x</kbd> on a US keyboard.
    KeyX,
    /// <kbd>y</kbd> on a US keyboard.
    /// Labelled <kbd>z</kbd> on a QWERTZ (e.g., German) keyboard.
    KeyY,
    /// <kbd>z</kbd> on a US keyboard.
    /// Labelled <kbd>w</kbd> on an AZERTY (e.g., French) keyboard, and <kbd>y</kbd> on a
    /// QWERTZ (e.g., German) keyboard.
    KeyZ,
    /// <kbd>-_</kbd> on a US keyboard.
    Minus,
    /// <kbd>.&gt;</kbd> on a US keyboard.
    Period,
    /// <kbd>'"</kbd> on a US keyboard.
    Quote,
    /// <kbd>;:</kbd> on a US keyboard.
    Semicolon,
    /// <kbd>/?</kbd> on a US keyboard.
    Slash,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd> or <kbd>⌥</kbd>.
    AltLeft,
    /// <kbd>Alt</kbd>, <kbd>Option</kbd> or <kbd>⌥</kbd>.
    /// This is labelled <kbd>AltGr</kbd> key on many keyboard layouts.
    AltRight,
    /// <kbd>Backspace</kbd> or <kbd>⌫</kbd>.
    /// Labelled <kbd>Delete</kbd> on Apple keyboards.
    Backspace,
    /// <kbd>CapsLock</kbd> or <kbd>⇪</kbd>
    CapsLock,
    /// The application context menu key, which is typically found between the right <kbd>Meta</kbd> key and the right <kbd>Control</kbd> key.
    ContextMenu,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlLeft,
    /// <kbd>Control</kbd> or <kbd>⌃</kbd>
    ControlRight,
    /// <kbd>Enter</kbd> or <kbd>↵</kbd>. Labelled <kbd>Return</kbd> on Apple keyboards.
    Enter,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd> or other OS symbol key.
    MetaLeft,
    /// The Windows, <kbd>⌘</kbd>, <kbd>Command</kbd> or other OS symbol key.
    MetaRight,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftLeft,
    /// <kbd>Shift</kbd> or <kbd>⇧</kbd>
    ShiftRight,
    /// <kbd> </kbd> (space)
    Space,
    /// <kbd>Tab</kbd> or <kbd>⇥</kbd>
    Tab,
    /// Japanese: <kbd>変換</kbd> (<span class="unicode">henkan</span>)
    Convert,
    /// Japanese: <kbd>カタカナ/ひらがな/ローマ字</kbd> (<span class="unicode">katakana/hiragana/romaji</span>)
    KanaMode,
    /// Korean: HangulMode <kbd>한/영</kbd> (<span class="unicode">han/yeong</span>)<br/>Japanese (Mac keyboard): <kbd>かな</kbd> (<span class="unicode">kana</span>)
    Lang1,
    /// Korean: Hanja <kbd>한자</kbd> (<span class="unicode">hanja</span>)<br/>Japanese (Mac keyboard): <kbd>英数</kbd> (<span class="unicode">eisu</span>)
    Lang2,
    /// Japanese (word-processing keyboard): Katakana
    Lang3,
    /// Japanese (word-processing keyboard): Hiragana
    Lang4,
    /// Japanese (word-processing keyboard): Zenkaku/Hankaku
    Lang5,
    /// Japanese: <kbd>無変換</kbd> (<span class="unicode">muhenkan</span>)
    NonConvert,
    /// <kbd>⌦</kbd>. The forward delete key.
    /// Note that on Apple keyboards, the key labelled <kbd>Delete</kbd> on the main part of
    /// the keyboard should be encoded as [`Backspace`][Code::Backspace].
    Delete,
    /// <kbd>End</kbd> or <kbd>↘</kbd>
    End,
    /// <kbd>Help</kbd>. Not present on standard PC keyboards.
    Help,
    /// <kbd>Home</kbd> or <kbd>↖</kbd>
    Home,
    /// <kbd>Insert</kbd> or <kbd>Ins</kbd>. Not present on Apple keyboards.
    Insert,
    /// <kbd>Page Down</kbd>, <kbd>PgDn</kbd> or <kbd>⇟</kbd>
    PageDown,
    /// <kbd>Page Up</kbd>, <kbd>PgUp</kbd> or <kbd>⇞</kbd>
    PageUp,
    /// <kbd>↓</kbd>
    ArrowDown,
    /// <kbd>←</kbd>
    ArrowLeft,
    /// <kbd>→</kbd>
    ArrowRight,
    /// <kbd>↑</kbd>
    ArrowUp,
    /// On the Mac, the [`NumLock`][Code::NumLock] code should be used for the numpad <kbd>Clear</kbd> key.
    NumLock,
    /// <kbd>0 Ins</kbd> on a keyboard<br/><kbd>0</kbd> on a phone or remote control
    Numpad0,
    /// <kbd>1 End</kbd> on a keyboard<br/><kbd>1</kbd> or <kbd>1 QZ</kbd> on a phone or
    /// remote control
    Numpad1,
    /// <kbd>2 ↓</kbd> on a keyboard<br/><kbd>2 ABC</kbd> on a phone or remote control
    Numpad2,
    /// <kbd>3 PgDn</kbd> on a keyboard<br/><kbd>3 DEF</kbd> on a phone or remote control
    Numpad3,
    /// <kbd>4 ←</kbd> on a keyboard<br/><kbd>4 GHI</kbd> on a phone or remote control
    Numpad4,
    /// <kbd>5</kbd> on a keyboard<br/><kbd>5 JKL</kbd> on a phone or remote control
    Numpad5,
    /// <kbd>6 →</kbd> on a keyboard<br/><kbd>6 MNO</kbd> on a phone or remote control
    Numpad6,
    /// <kbd>7 Home</kbd> on a keyboard<br/><kbd>7 PQRS</kbd> or <kbd>7 PRS</kbd> on a phone
    /// or remote control
    Numpad7,
    /// <kbd>8 ↑</kbd> on a keyboard<br/><kbd>8 TUV</kbd> on a phone or remote control
    Numpad8,
    /// <kbd>9 PgUp</kbd> on a keyboard<br/><kbd>9 WXYZ</kbd> or <kbd>9 WXY</kbd> on a phone
    /// or remote control
    Numpad9,
    /// <kbd>+</kbd>
    NumpadAdd,
    /// Found on the Microsoft Natural Keyboard.
    NumpadBackspace,
    /// <kbd>C</kbd> or <kbd>AC</kbd> (All Clear). Also for use with numpads that have a <kbd>Clear</kbd> key that is separate from the <kbd>NumLock</kbd> key. On the Mac, the numpad <kbd>Clear</kbd> key should always
    /// be encoded as [`NumLock`][Code::NumLock].
    NumpadClear,
    /// <kbd>CE</kbd> (Clear Entry)
    NumpadClearEntry,
    /// <kbd>,</kbd> (thousands separator). For locales where the thousands separator
    /// is a "." (e.g., Brazil), this key may generate a <kbd>.</kbd>.
    NumpadComma,
    /// <kbd>. Del</kbd>. For locales where the decimal separator is "," (e.g.,
    /// Brazil), this key may generate a <kbd>,</kbd>.
    NumpadDecimal,
    /// <kbd>/</kbd>
    NumpadDivide,
    NumpadEnter,
    /// <kbd>=</kbd>
    NumpadEqual,
    /// <kbd>#</kbd> on a phone or remote control device. This key is typically found
    /// below the <kbd>9</kbd> key and to the right of the <kbd>0</kbd> key.
    NumpadHash,
    /// <kbd>M+</kbd> Add current entry to the value stored in memory.
    NumpadMemoryAdd,
    /// <kbd>MC</kbd> Clear the value stored in memory.
    NumpadMemoryClear,
    /// <kbd>MR</kbd> Replace the current entry with the value stored in memory.
    NumpadMemoryRecall,
    /// <kbd>MS</kbd> Replace the value stored in memory with the current entry.
    NumpadMemoryStore,
    /// <kbd>M-</kbd> Subtract current entry from the value stored in memory.
    NumpadMemorySubtract,
    /// <kbd>*</kbd> on a keyboard. For use with numpads that provide mathematical
    /// operations (<kbd>+</kbd>, <kbd>-</kbd>, <kbd>*</kbd> and <kbd>/</kbd>).<br/>Use [`NumpadStar`][Code::NumpadStar] for the <kbd>*</kbd> key on phones and remote controls.
    NumpadMultiply,
    /// <kbd>(</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenLeft,
    /// <kbd>)</kbd> Found on the Microsoft Natural Keyboard.
    NumpadParenRight,
    /// <kbd>*</kbd> on a phone or remote control device.
    /// This key is typically found below the <kbd>7</kbd> key and to the left of
    /// the <kbd>0</kbd> key.<br/>Use [`NumpadMultiply`][Code::NumpadMultiply] for the <kbd>*</kbd> key on
    /// numeric keypads.
    NumpadStar,
    /// <kbd>-</kbd>
    NumpadSubtract,
    /// <kbd>Esc</kbd> or <kbd>⎋</kbd>
    Escape,
    /// <kbd>Fn</kbd> This is typically a hardware key that does not generate a separate
    /// code. Most keyboards do not place this key in the function section, but it is
    /// included here to keep it with related keys.
    Fn,
    /// <kbd>FLock</kbd> or <kbd>FnLock</kbd>. Function Lock key. Found on the Microsoft
    /// Natural Keyboard.
    FnLock,
    /// <kbd>PrtScr SysRq</kbd> or <kbd>Print Screen</kbd>
    PrintScreen,
    /// <kbd>Scroll Lock</kbd>
    ScrollLock,
    /// <kbd>Pause Break</kbd>
    Pause,
    /// Some laptops place this key to the left of the <kbd>↑</kbd> key.
    BrowserBack,
    BrowserFavorites,
    /// Some laptops place this key to the right of the <kbd>↑</kbd> key.
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    /// <kbd>Eject</kbd> or <kbd>⏏</kbd>. This key is placed in the function
    /// section on some Apple keyboards.
    Eject,
    /// Sometimes labelled <kbd>My Computer</kbd> on the keyboard
    LaunchApp1,
    /// Sometimes labelled <kbd>Calculator</kbd> on the keyboard
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    /// This key is placed in the function section on some Apple keyboards,
    /// replacing the <kbd>Eject</kbd> key.
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Hyper,
    Super,
    Turbo,
    Abort,
    Resume,
    Suspend,
    /// Found on Sun’s USB keyboard.
    Again,
    /// Found on Sun’s USB keyboard.
    Copy,
    /// Found on Sun’s USB keyboard.
    Cut,
    /// Found on Sun’s USB keyboard.
    Find,
    /// Found on Sun’s USB keyboard.
    Open,
    /// Found on Sun’s USB keyboard.
    Paste,
    /// Found on Sun’s USB keyboard.
    Props,
    /// Found on Sun’s USB keyboard.
    Select,
    /// Found on Sun’s USB keyboard.
    Undo,
    /// Use for dedicated <kbd>ひらがな</kbd> key found on some Japanese word processing keyboards.
    Hiragana,
    /// Use for dedicated <kbd>カタカナ</kbd> key found on some Japanese word processing keyboards.
    Katakana,
    /// This value code should be used when no other
    /// value given in this specification is appropriate.
    Unidentified,
    /// <kbd>F1</kbd>
    F1,
    /// <kbd>F2</kbd>
    F2,
    /// <kbd>F3</kbd>
    F3,
    /// <kbd>F4</kbd>
    F4,
    /// <kbd>F5</kbd>
    F5,
    /// <kbd>F6</kbd>
    F6,
    /// <kbd>F7</kbd>
    F7,
    /// <kbd>F8</kbd>
    F8,
    /// <kbd>F9</kbd>
    F9,
    /// <kbd>F10</kbd>
    F10,
    /// <kbd>F11</kbd>
    F11,
    /// <kbd>F12</kbd>
    F12,
    /// <kbd>F13</kbd>
    F13,
    /// <kbd>F14</kbd>
    F14,
    /// <kbd>F15</kbd>
    F15,
    /// <kbd>F16</kbd>
    F16,
    /// <kbd>F17</kbd>
    F17,
    /// <kbd>F18</kbd>
    F18,
    /// <kbd>F19</kbd>
    F19,
    /// <kbd>F20</kbd>
    F20,
    /// <kbd>F21</kbd>
    F21,
    /// <kbd>F22</kbd>
    F22,
    /// <kbd>F23</kbd>
    F23,
    /// <kbd>F24</kbd>
    F24,
    /// <kbd>F25</kbd>
    F25,
    /// <kbd>F26</kbd>
    F26,
    /// <kbd>F27</kbd>
    F27,
    /// <kbd>F28</kbd>
    F28,
    /// <kbd>F29</kbd>
    F29,
    /// <kbd>F30</kbd>
    F30,
    /// <kbd>F31</kbd>
    F31,
    /// <kbd>F32</kbd>
    F32,
    /// <kbd>F33</kbd>
    F33,
    /// <kbd>F34</kbd>
    F34,
    /// <kbd>F35</kbd>
    F35,
    /// Non-standard code value supported by Chromium.
    BrightnessDown,
    /// Non-standard code value supported by Chromium.
    BrightnessUp,
    /// Non-standard code value supported by Chromium.
    DisplayToggleIntExt,
    /// Non-standard code value supported by Chromium.
    KeyboardLayoutSelect,
    /// Non-standard code value supported by Chromium.
    LaunchAssistant,
    /// Non-standard code value supported by Chromium.
    LaunchControlPanel,
    /// Non-standard code value supported by Chromium.
    LaunchScreenSaver,
    /// Non-standard code value supported by Chromium.
    MailForward,
    /// Non-standard code value supported by Chromium.
    MailReply,
    /// Non-standard code value supported by Chromium.
    MailSend,
    /// Non-standard code value supported by Chromium.
    MediaFastForward,
    /// Non-standard code value supported by Chromium.
    MediaPause,
    /// Non-standard code value supported by Chromium.
    MediaPlay,
    /// Non-standard code value supported by Chromium.
    MediaRecord,
    /// Non-standard code value supported by Chromium.
    MediaRewind,
    /// Non-standard code value supported by Chromium.
    MicrophoneMuteToggle,
    /// Non-standard code value supported by Chromium.
    PrivacyScreenToggle,
    /// Non-standard code value supported by Chromium.
    SelectTask,
    /// Non-standard code value supported by Chromium.
    ShowAllWindows,
    /// Non-standard code value supported by Chromium.
    ZoomToggle,
}


impl Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Code::*;
        match *self {
            Backquote => f.write_str("Backquote"),
            Backslash => f.write_str("Backslash"),
            BracketLeft => f.write_str("BracketLeft"),
            BracketRight => f.write_str("BracketRight"),
            Comma => f.write_str("Comma"),
            Digit0 => f.write_str("Digit0"),
            Digit1 => f.write_str("Digit1"),
            Digit2 => f.write_str("Digit2"),
            Digit3 => f.write_str("Digit3"),
            Digit4 => f.write_str("Digit4"),
            Digit5 => f.write_str("Digit5"),
            Digit6 => f.write_str("Digit6"),
            Digit7 => f.write_str("Digit7"),
            Digit8 => f.write_str("Digit8"),
            Digit9 => f.write_str("Digit9"),
            Equal => f.write_str("Equal"),
            IntlBackslash => f.write_str("IntlBackslash"),
            IntlRo => f.write_str("IntlRo"),
            IntlYen => f.write_str("IntlYen"),
            KeyA => f.write_str("KeyA"),
            KeyB => f.write_str("KeyB"),
            KeyC => f.write_str("KeyC"),
            KeyD => f.write_str("KeyD"),
            KeyE => f.write_str("KeyE"),
            KeyF => f.write_str("KeyF"),
            KeyG => f.write_str("KeyG"),
            KeyH => f.write_str("KeyH"),
            KeyI => f.write_str("KeyI"),
            KeyJ => f.write_str("KeyJ"),
            KeyK => f.write_str("KeyK"),
            KeyL => f.write_str("KeyL"),
            KeyM => f.write_str("KeyM"),
            KeyN => f.write_str("KeyN"),
            KeyO => f.write_str("KeyO"),
            KeyP => f.write_str("KeyP"),
            KeyQ => f.write_str("KeyQ"),
            KeyR => f.write_str("KeyR"),
            KeyS => f.write_str("KeyS"),
            KeyT => f.write_str("KeyT"),
            KeyU => f.write_str("KeyU"),
            KeyV => f.write_str("KeyV"),
            KeyW => f.write_str("KeyW"),
            KeyX => f.write_str("KeyX"),
            KeyY => f.write_str("KeyY"),
            KeyZ => f.write_str("KeyZ"),
            Minus => f.write_str("Minus"),
            Period => f.write_str("Period"),
            Quote => f.write_str("Quote"),
            Semicolon => f.write_str("Semicolon"),
            Slash => f.write_str("Slash"),
            AltLeft => f.write_str("AltLeft"),
            AltRight => f.write_str("AltRight"),
            Backspace => f.write_str("Backspace"),
            CapsLock => f.write_str("CapsLock"),
            ContextMenu => f.write_str("ContextMenu"),
            ControlLeft => f.write_str("ControlLeft"),
            ControlRight => f.write_str("ControlRight"),
            Enter => f.write_str("Enter"),
            MetaLeft => f.write_str("MetaLeft"),
            MetaRight => f.write_str("MetaRight"),
            ShiftLeft => f.write_str("ShiftLeft"),
            ShiftRight => f.write_str("ShiftRight"),
            Space => f.write_str("Space"),
            Tab => f.write_str("Tab"),
            Convert => f.write_str("Convert"),
            KanaMode => f.write_str("KanaMode"),
            Lang1 => f.write_str("Lang1"),
            Lang2 => f.write_str("Lang2"),
            Lang3 => f.write_str("Lang3"),
            Lang4 => f.write_str("Lang4"),
            Lang5 => f.write_str("Lang5"),
            NonConvert => f.write_str("NonConvert"),
            Delete => f.write_str("Delete"),
            End => f.write_str("End"),
            Help => f.write_str("Help"),
            Home => f.write_str("Home"),
            Insert => f.write_str("Insert"),
            PageDown => f.write_str("PageDown"),
            PageUp => f.write_str("PageUp"),
            ArrowDown => f.write_str("ArrowDown"),
            ArrowLeft => f.write_str("ArrowLeft"),
            ArrowRight => f.write_str("ArrowRight"),
            ArrowUp => f.write_str("ArrowUp"),
            NumLock => f.write_str("NumLock"),
            Numpad0 => f.write_str("Numpad0"),
            Numpad1 => f.write_str("Numpad1"),
            Numpad2 => f.write_str("Numpad2"),
            Numpad3 => f.write_str("Numpad3"),
            Numpad4 => f.write_str("Numpad4"),
            Numpad5 => f.write_str("Numpad5"),
            Numpad6 => f.write_str("Numpad6"),
            Numpad7 => f.write_str("Numpad7"),
            Numpad8 => f.write_str("Numpad8"),
            Numpad9 => f.write_str("Numpad9"),
            NumpadAdd => f.write_str("NumpadAdd"),
            NumpadBackspace => f.write_str("NumpadBackspace"),
            NumpadClear => f.write_str("NumpadClear"),
            NumpadClearEntry => f.write_str("NumpadClearEntry"),
            NumpadComma => f.write_str("NumpadComma"),
            NumpadDecimal => f.write_str("NumpadDecimal"),
            NumpadDivide => f.write_str("NumpadDivide"),
            NumpadEnter => f.write_str("NumpadEnter"),
            NumpadEqual => f.write_str("NumpadEqual"),
            NumpadHash => f.write_str("NumpadHash"),
            NumpadMemoryAdd => f.write_str("NumpadMemoryAdd"),
            NumpadMemoryClear => f.write_str("NumpadMemoryClear"),
            NumpadMemoryRecall => f.write_str("NumpadMemoryRecall"),
            NumpadMemoryStore => f.write_str("NumpadMemoryStore"),
            NumpadMemorySubtract => f.write_str("NumpadMemorySubtract"),
            NumpadMultiply => f.write_str("NumpadMultiply"),
            NumpadParenLeft => f.write_str("NumpadParenLeft"),
            NumpadParenRight => f.write_str("NumpadParenRight"),
            NumpadStar => f.write_str("NumpadStar"),
            NumpadSubtract => f.write_str("NumpadSubtract"),
            Escape => f.write_str("Escape"),
            Fn => f.write_str("Fn"),
            FnLock => f.write_str("FnLock"),
            PrintScreen => f.write_str("PrintScreen"),
            ScrollLock => f.write_str("ScrollLock"),
            Pause => f.write_str("Pause"),
            BrowserBack => f.write_str("BrowserBack"),
            BrowserFavorites => f.write_str("BrowserFavorites"),
            BrowserForward => f.write_str("BrowserForward"),
            BrowserHome => f.write_str("BrowserHome"),
            BrowserRefresh => f.write_str("BrowserRefresh"),
            BrowserSearch => f.write_str("BrowserSearch"),
            BrowserStop => f.write_str("BrowserStop"),
            Eject => f.write_str("Eject"),
            LaunchApp1 => f.write_str("LaunchApp1"),
            LaunchApp2 => f.write_str("LaunchApp2"),
            LaunchMail => f.write_str("LaunchMail"),
            MediaPlayPause => f.write_str("MediaPlayPause"),
            MediaSelect => f.write_str("MediaSelect"),
            MediaStop => f.write_str("MediaStop"),
            MediaTrackNext => f.write_str("MediaTrackNext"),
            MediaTrackPrevious => f.write_str("MediaTrackPrevious"),
            Power => f.write_str("Power"),
            Sleep => f.write_str("Sleep"),
            AudioVolumeDown => f.write_str("AudioVolumeDown"),
            AudioVolumeMute => f.write_str("AudioVolumeMute"),
            AudioVolumeUp => f.write_str("AudioVolumeUp"),
            WakeUp => f.write_str("WakeUp"),
            Hyper => f.write_str("Hyper"),
            Super => f.write_str("Super"),
            Turbo => f.write_str("Turbo"),
            Abort => f.write_str("Abort"),
            Resume => f.write_str("Resume"),
            Suspend => f.write_str("Suspend"),
            Again => f.write_str("Again"),
            Copy => f.write_str("Copy"),
            Cut => f.write_str("Cut"),
            Find => f.write_str("Find"),
            Open => f.write_str("Open"),
            Paste => f.write_str("Paste"),
            Props => f.write_str("Props"),
            Select => f.write_str("Select"),
            Undo => f.write_str("Undo"),
            Hiragana => f.write_str("Hiragana"),
            Katakana => f.write_str("Katakana"),
            Unidentified => f.write_str("Unidentified"),
            F1 => f.write_str("F1"),
            F2 => f.write_str("F2"),
            F3 => f.write_str("F3"),
            F4 => f.write_str("F4"),
            F5 => f.write_str("F5"),
            F6 => f.write_str("F6"),
            F7 => f.write_str("F7"),
            F8 => f.write_str("F8"),
            F9 => f.write_str("F9"),
            F10 => f.write_str("F10"),
            F11 => f.write_str("F11"),
            F12 => f.write_str("F12"),
            F13 => f.write_str("F13"),
            F14 => f.write_str("F14"),
            F15 => f.write_str("F15"),
            F16 => f.write_str("F16"),
            F17 => f.write_str("F17"),
            F18 => f.write_str("F18"),
            F19 => f.write_str("F19"),
            F20 => f.write_str("F20"),
            F21 => f.write_str("F21"),
            F22 => f.write_str("F22"),
            F23 => f.write_str("F23"),
            F24 => f.write_str("F24"),
            F25 => f.write_str("F25"),
            F26 => f.write_str("F26"),
            F27 => f.write_str("F27"),
            F28 => f.write_str("F28"),
            F29 => f.write_str("F29"),
            F30 => f.write_str("F30"),
            F31 => f.write_str("F31"),
            F32 => f.write_str("F32"),
            F33 => f.write_str("F33"),
            F34 => f.write_str("F34"),
            F35 => f.write_str("F35"),
            BrightnessDown => f.write_str("BrightnessDown"),
            BrightnessUp => f.write_str("BrightnessUp"),
            DisplayToggleIntExt => f.write_str("DisplayToggleIntExt"),
            KeyboardLayoutSelect => f.write_str("KeyboardLayoutSelect"),
            LaunchAssistant => f.write_str("LaunchAssistant"),
            LaunchControlPanel => f.write_str("LaunchControlPanel"),
            LaunchScreenSaver => f.write_str("LaunchScreenSaver"),
            MailForward => f.write_str("MailForward"),
            MailReply => f.write_str("MailReply"),
            MailSend => f.write_str("MailSend"),
            MediaFastForward => f.write_str("MediaFastForward"),
            MediaPause => f.write_str("MediaPause"),
            MediaPlay => f.write_str("MediaPlay"),
            MediaRecord => f.write_str("MediaRecord"),
            MediaRewind => f.write_str("MediaRewind"),
            MicrophoneMuteToggle => f.write_str("MicrophoneMuteToggle"),
            PrivacyScreenToggle => f.write_str("PrivacyScreenToggle"),
            SelectTask => f.write_str("SelectTask"),
            ShowAllWindows => f.write_str("ShowAllWindows"),
            ZoomToggle => f.write_str("ZoomToggle"),

        }
    }
}

impl FromStr for Code {
    type Err = UnrecognizedCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::Code::*;
        match s {
            "Backquote" => Ok(Backquote),
            "Backslash" => Ok(Backslash),
            "BracketLeft" => Ok(BracketLeft),
            "BracketRight" => Ok(BracketRight),
            "Comma" => Ok(Comma),
            "Digit0" => Ok(Digit0),
            "Digit1" => Ok(Digit1),
            "Digit2" => Ok(Digit2),
            "Digit3" => Ok(Digit3),
            "Digit4" => Ok(Digit4),
            "Digit5" => Ok(Digit5),
            "Digit6" => Ok(Digit6),
            "Digit7" => Ok(Digit7),
            "Digit8" => Ok(Digit8),
            "Digit9" => Ok(Digit9),
            "Equal" => Ok(Equal),
            "IntlBackslash" => Ok(IntlBackslash),
            "IntlRo" => Ok(IntlRo),
            "IntlYen" => Ok(IntlYen),
            "KeyA" => Ok(KeyA),
            "KeyB" => Ok(KeyB),
            "KeyC" => Ok(KeyC),
            "KeyD" => Ok(KeyD),
            "KeyE" => Ok(KeyE),
            "KeyF" => Ok(KeyF),
            "KeyG" => Ok(KeyG),
            "KeyH" => Ok(KeyH),
            "KeyI" => Ok(KeyI),
            "KeyJ" => Ok(KeyJ),
            "KeyK" => Ok(KeyK),
            "KeyL" => Ok(KeyL),
            "KeyM" => Ok(KeyM),
            "KeyN" => Ok(KeyN),
            "KeyO" => Ok(KeyO),
            "KeyP" => Ok(KeyP),
            "KeyQ" => Ok(KeyQ),
            "KeyR" => Ok(KeyR),
            "KeyS" => Ok(KeyS),
            "KeyT" => Ok(KeyT),
            "KeyU" => Ok(KeyU),
            "KeyV" => Ok(KeyV),
            "KeyW" => Ok(KeyW),
            "KeyX" => Ok(KeyX),
            "KeyY" => Ok(KeyY),
            "KeyZ" => Ok(KeyZ),
            "Minus" => Ok(Minus),
            "Period" => Ok(Period),
            "Quote" => Ok(Quote),
            "Semicolon" => Ok(Semicolon),
            "Slash" => Ok(Slash),
            "AltLeft" => Ok(AltLeft),
            "AltRight" => Ok(AltRight),
            "Backspace" => Ok(Backspace),
            "CapsLock" => Ok(CapsLock),
            "ContextMenu" => Ok(ContextMenu),
            "ControlLeft" => Ok(ControlLeft),
            "ControlRight" => Ok(ControlRight),
            "Enter" => Ok(Enter),
            "MetaLeft" | "OSLeft" => Ok(MetaLeft),
            "MetaRight" | "OSRight" => Ok(MetaRight),
            "ShiftLeft" => Ok(ShiftLeft),
            "ShiftRight" => Ok(ShiftRight),
            "Space" => Ok(Space),
            "Tab" => Ok(Tab),
            "Convert" => Ok(Convert),
            "KanaMode" => Ok(KanaMode),
            "Lang1" => Ok(Lang1),
            "Lang2" => Ok(Lang2),
            "Lang3" => Ok(Lang3),
            "Lang4" => Ok(Lang4),
            "Lang5" => Ok(Lang5),
            "NonConvert" => Ok(NonConvert),
            "Delete" => Ok(Delete),
            "End" => Ok(End),
            "Help" => Ok(Help),
            "Home" => Ok(Home),
            "Insert" => Ok(Insert),
            "PageDown" => Ok(PageDown),
            "PageUp" => Ok(PageUp),
            "ArrowDown" => Ok(ArrowDown),
            "ArrowLeft" => Ok(ArrowLeft),
            "ArrowRight" => Ok(ArrowRight),
            "ArrowUp" => Ok(ArrowUp),
            "NumLock" => Ok(NumLock),
            "Numpad0" => Ok(Numpad0),
            "Numpad1" => Ok(Numpad1),
            "Numpad2" => Ok(Numpad2),
            "Numpad3" => Ok(Numpad3),
            "Numpad4" => Ok(Numpad4),
            "Numpad5" => Ok(Numpad5),
            "Numpad6" => Ok(Numpad6),
            "Numpad7" => Ok(Numpad7),
            "Numpad8" => Ok(Numpad8),
            "Numpad9" => Ok(Numpad9),
            "NumpadAdd" => Ok(NumpadAdd),
            "NumpadBackspace" => Ok(NumpadBackspace),
            "NumpadClear" => Ok(NumpadClear),
            "NumpadClearEntry" => Ok(NumpadClearEntry),
            "NumpadComma" => Ok(NumpadComma),
            "NumpadDecimal" => Ok(NumpadDecimal),
            "NumpadDivide" => Ok(NumpadDivide),
            "NumpadEnter" => Ok(NumpadEnter),
            "NumpadEqual" => Ok(NumpadEqual),
            "NumpadHash" => Ok(NumpadHash),
            "NumpadMemoryAdd" => Ok(NumpadMemoryAdd),
            "NumpadMemoryClear" => Ok(NumpadMemoryClear),
            "NumpadMemoryRecall" => Ok(NumpadMemoryRecall),
            "NumpadMemoryStore" => Ok(NumpadMemoryStore),
            "NumpadMemorySubtract" => Ok(NumpadMemorySubtract),
            "NumpadMultiply" => Ok(NumpadMultiply),
            "NumpadParenLeft" => Ok(NumpadParenLeft),
            "NumpadParenRight" => Ok(NumpadParenRight),
            "NumpadStar" => Ok(NumpadStar),
            "NumpadSubtract" => Ok(NumpadSubtract),
            "Escape" => Ok(Escape),
            "Fn" => Ok(Fn),
            "FnLock" => Ok(FnLock),
            "PrintScreen" => Ok(PrintScreen),
            "ScrollLock" => Ok(ScrollLock),
            "Pause" => Ok(Pause),
            "BrowserBack" => Ok(BrowserBack),
            "BrowserFavorites" => Ok(BrowserFavorites),
            "BrowserForward" => Ok(BrowserForward),
            "BrowserHome" => Ok(BrowserHome),
            "BrowserRefresh" => Ok(BrowserRefresh),
            "BrowserSearch" => Ok(BrowserSearch),
            "BrowserStop" => Ok(BrowserStop),
            "Eject" => Ok(Eject),
            "LaunchApp1" => Ok(LaunchApp1),
            "LaunchApp2" => Ok(LaunchApp2),
            "LaunchMail" => Ok(LaunchMail),
            "MediaPlayPause" => Ok(MediaPlayPause),
            "MediaSelect" | "LaunchMediaPlayer" => Ok(MediaSelect),
            "MediaStop" => Ok(MediaStop),
            "MediaTrackNext" => Ok(MediaTrackNext),
            "MediaTrackPrevious" => Ok(MediaTrackPrevious),
            "Power" => Ok(Power),
            "Sleep" => Ok(Sleep),
            "AudioVolumeDown" | "VolumeDown" => Ok(AudioVolumeDown),
            "AudioVolumeMute" | "VolumeMute" => Ok(AudioVolumeMute),
            "AudioVolumeUp" | "VolumeUp" => Ok(AudioVolumeUp),
            "WakeUp" => Ok(WakeUp),
            "Hyper" => Ok(Hyper),
            "Super" => Ok(Super),
            "Turbo" => Ok(Turbo),
            "Abort" => Ok(Abort),
            "Resume" => Ok(Resume),
            "Suspend" => Ok(Suspend),
            "Again" => Ok(Again),
            "Copy" => Ok(Copy),
            "Cut" => Ok(Cut),
            "Find" => Ok(Find),
            "Open" => Ok(Open),
            "Paste" => Ok(Paste),
            "Props" => Ok(Props),
            "Select" => Ok(Select),
            "Undo" => Ok(Undo),
            "Hiragana" => Ok(Hiragana),
            "Katakana" => Ok(Katakana),
            "Unidentified" => Ok(Unidentified),
            "F1" => Ok(F1),
            "F2" => Ok(F2),
            "F3" => Ok(F3),
            "F4" => Ok(F4),
            "F5" => Ok(F5),
            "F6" => Ok(F6),
            "F7" => Ok(F7),
            "F8" => Ok(F8),
            "F9" => Ok(F9),
            "F10" => Ok(F10),
            "F11" => Ok(F11),
            "F12" => Ok(F12),
            "F13" => Ok(F13),
            "F14" => Ok(F14),
            "F15" => Ok(F15),
            "F16" => Ok(F16),
            "F17" => Ok(F17),
            "F18" => Ok(F18),
            "F19" => Ok(F19),
            "F20" => Ok(F20),
            "F21" => Ok(F21),
            "F22" => Ok(F22),
            "F23" => Ok(F23),
            "F24" => Ok(F24),
            "F25" => Ok(F25),
            "F26" => Ok(F26),
            "F27" => Ok(F27),
            "F28" => Ok(F28),
            "F29" => Ok(F29),
            "F30" => Ok(F30),
            "F31" => Ok(F31),
            "F32" => Ok(F32),
            "F33" => Ok(F33),
            "F34" => Ok(F34),
            "F35" => Ok(F35),
            "BrightnessDown" => Ok(BrightnessDown),
            "BrightnessUp" => Ok(BrightnessUp),
            "DisplayToggleIntExt" => Ok(DisplayToggleIntExt),
            "KeyboardLayoutSelect" => Ok(KeyboardLayoutSelect),
            "LaunchAssistant" => Ok(LaunchAssistant),
            "LaunchControlPanel" => Ok(LaunchControlPanel),
            "LaunchScreenSaver" => Ok(LaunchScreenSaver),
            "MailForward" => Ok(MailForward),
            "MailReply" => Ok(MailReply),
            "MailSend" => Ok(MailSend),
            "MediaFastForward" => Ok(MediaFastForward),
            "MediaPause" => Ok(MediaPause),
            "MediaPlay" => Ok(MediaPlay),
            "MediaRecord" => Ok(MediaRecord),
            "MediaRewind" => Ok(MediaRewind),
            "MicrophoneMuteToggle" => Ok(MicrophoneMuteToggle),
            "PrivacyScreenToggle" => Ok(PrivacyScreenToggle),
            "SelectTask" => Ok(SelectTask),
            "ShowAllWindows" => Ok(ShowAllWindows),
            "ZoomToggle" => Ok(ZoomToggle),

            _ => Err(UnrecognizedCodeError),
        }
    }
}

/// Parse from string error, returned when string does not match to any Code variant.
#[derive(Clone, Debug)]
pub struct UnrecognizedCodeError;

impl fmt::Display for UnrecognizedCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognized code")
    }
}

impl Error for UnrecognizedCodeError {}

