
// AUTO GENERATED CODE - DO NOT EDIT

use std::fmt::{self, Display};
use std::str::FromStr;
use std::error::Error;

/// Key represents the meaning of a keypress.
///
/// Specification:
/// <https://w3c.github.io/uievents-key/>
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Key {
    /// A key string that corresponds to the character typed by the user,
    /// taking into account the user’s current locale setting, modifier state,
    /// and any system-level keyboard mapping overrides that are in effect.
    Character(String),
    
    /// This key value is used when an implementation is unable to
    /// identify another key value, due to either hardware,
    /// platform, or software constraints.
    Unidentified,
    /// The <code class="keycap">Alt</code> (Alternative) key.<br> This key enables the alternate modifier function for interpreting concurrent or subsequent keyboard input.<br> This key value is also used for the Apple <code class="keycap">Option</code> key.
    Alt,
    /// The Alternate Graphics (<code class="keycap">AltGr</code> or <code class="keycap">AltGraph</code>) key.
    /// This key is used enable the ISO Level 3 shift modifier (the standard <code class="keycap">Shift</code> key is the level 2 modifier).
    /// See [ISO9995-1].
    AltGraph,
    /// The <code class="keycap">Caps Lock</code> (Capital) key.
    /// Toggle capital character lock function for interpreting subsequent keyboard input event.
    CapsLock,
    /// The <code class="keycap">Control</code> or <code class="keycap">Ctrl</code> key, to enable control modifier function for interpreting concurrent or subsequent keyboard input.
    Control,
    /// The Function switch <code class="keycap">Fn</code> key.<br> Activating this key simultaneously with another key changes that key’s value to an alternate character or function.
    /// This key is often handled directly in the keyboard hardware and does not usually generate key events.
    Fn,
    /// The Function-Lock (<code class="keycap">FnLock</code> or <code class="keycap">F-Lock</code>) key.
    /// Activating this key switches the mode of the keyboard to changes some keys' values to an alternate character or function.
    /// This key is often handled directly in the keyboard hardware and does not usually generate key events.
    FnLock,
    /// The <code class="keycap">Meta</code> key, to enable meta modifier function for interpreting concurrent or subsequent keyboard input.
    /// This key value is used for the <q>Windows Logo</q> key and the Apple <code class="keycap">Command</code> or <code class="keycap">⌘</code> key.
    Meta,
    /// The <code class="keycap">NumLock</code> or Number Lock key, to toggle numpad mode function for interpreting subsequent keyboard input.
    NumLock,
    /// The <code class="keycap">Scroll Lock</code> key, to toggle between scrolling and cursor movement modes.
    ScrollLock,
    /// The <code class="keycap">Shift</code> key, to enable shift modifier function for interpreting concurrent or subsequent keyboard input.
    Shift,
    /// The Symbol modifier key (used on some virtual keyboards).
    Symbol,
    /// The Symbol Lock key.
    SymbolLock,
    /// The <code class="keycap">Hyper</code> key.
    Hyper,
    /// The <code class="keycap">Super</code> key.
    Super,
    /// The <code class="keycap">Enter</code> or <code class="keycap">↵</code> key, to activate current selection or accept current input.<br> This key value is also used for the <code class="keycap">Return</code> (Macintosh numpad) key.<br> This key value is also used for the Android <code class="android">KEYCODE_DPAD_CENTER</code>.
    Enter,
    /// The Horizontal Tabulation <code class="keycap">Tab</code> key.
    Tab,
    /// The down arrow key, to navigate or traverse downward. (<code class="android">KEYCODE_DPAD_DOWN</code>)
    ArrowDown,
    /// The left arrow key, to navigate or traverse leftward. (<code class="android">KEYCODE_DPAD_LEFT</code>)
    ArrowLeft,
    /// The right arrow key, to navigate or traverse rightward. (<code class="android">KEYCODE_DPAD_RIGHT</code>)
    ArrowRight,
    /// The up arrow key, to navigate or traverse upward. (<code class="android">KEYCODE_DPAD_UP</code>)
    ArrowUp,
    /// The End key, used with keyboard entry to go to the end of content (<code class="android">KEYCODE_MOVE_END</code>).
    End,
    /// The Home key, used with keyboard entry, to go to start of content (<code class="android">KEYCODE_MOVE_HOME</code>).<br> For the mobile phone <code class="keycap">Home</code> key (which goes to the phone’s main screen), use <code class="key">"GoHome"</code>.
    Home,
    /// The Page Down key, to scroll down or display next page of content.
    PageDown,
    /// The Page Up key, to scroll up or display previous page of content.
    PageUp,
    /// The Backspace key. This key value is also used for the key labeled <code class="keycap">Delete</code> on MacOS keyboards.
    Backspace,
    /// Remove the currently selected input.
    Clear,
    /// Copy the current selection. (<code class="appcommand">APPCOMMAND_COPY</code>)
    Copy,
    /// The Cursor Select (Crsel) key.
    CrSel,
    /// Cut the current selection. (<code class="appcommand">APPCOMMAND_CUT</code>)
    Cut,
    /// The Delete (Del) Key.
    /// This key value is also used for the key labeled <code class="keycap">Delete</code> on MacOS keyboards when modified by the <code class="keycap">Fn</code> key.
    Delete,
    /// The Erase to End of Field key.
    /// This key deletes all characters from the current cursor position to the end of the current field.
    EraseEof,
    /// The Extend Selection (Exsel) key.
    ExSel,
    /// The Insert (Ins) key, to toggle between text modes for insertion or overtyping. (<code class="android">KEYCODE_INSERT</code>)
    Insert,
    /// The Paste key. (<code class="appcommand">APPCOMMAND_PASTE</code>)
    Paste,
    /// Redo the last action. (<code class="appcommand">APPCOMMAND_REDO</code>)
    Redo,
    /// Undo the last action. (<code class="appcommand">APPCOMMAND_UNDO</code>)
    Undo,
    /// The Accept (Commit, OK) key. Accept current option or input method sequence conversion.
    Accept,
    /// The Again key, to redo or repeat an action.
    Again,
    /// The Attention (Attn) key.
    Attn,
    /// The Cancel key.
    Cancel,
    /// Show the application’s context menu.
    /// This key is commonly found between the right <code class="keycap">Meta</code> key and the right <code class="keycap">Control</code> key.
    ContextMenu,
    /// The <code class="keycap">Esc</code> key. This key was originally used to initiate an escape sequence, but is
    /// now more generally used to exit or "escape" the current context, such as closing a dialog
    /// or exiting full screen mode.
    Escape,
    /// The Execute key.
    Execute,
    /// Open the Find dialog. (<code class="appcommand">APPCOMMAND_FIND</code>)
    Find,
    /// Open a help dialog or toggle display of help information. (<code class="appcommand"><code class="appcommand">APPCOMMAND_HELP</code></code>, <code class="android"><code class="android">KEYCODE_HELP</code></code>)
    Help,
    /// Pause the current state or application (as appropriate).
    /// <p class="note" role="note">Do not use this value for the <code class="keycap">Pause</code> button on media controllers. Use <code class="key">"MediaPause"</code> instead.</p>
    Pause,
    /// Play or resume the current state or application (as appropriate).
    /// <p class="note" role="note">Do not use this value for the <code class="keycap">Play</code> button on media controllers. Use <code class="key">"MediaPlay"</code> instead.</p>
    Play,
    /// The properties (Props) key.
    Props,
    /// The Select key.
    Select,
    /// The ZoomIn key. (<code class="android">KEYCODE_ZOOM_IN</code>)
    ZoomIn,
    /// The ZoomOut key. (<code class="android">KEYCODE_ZOOM_OUT</code>)
    ZoomOut,
    /// The Brightness Down key. Typically controls the display brightness. (<code class="android">KEYCODE_BRIGHTNESS_DOWN</code>)
    BrightnessDown,
    /// The Brightness Up key. Typically controls the display brightness. (<code class="android">KEYCODE_BRIGHTNESS_UP</code>)
    BrightnessUp,
    /// Toggle removable media to eject (open) and insert (close) state. (<code class="android">KEYCODE_MEDIA_EJECT</code>)
    Eject,
    /// The LogOff key.
    LogOff,
    /// Toggle power state. (<code class="android">KEYCODE_POWER</code>)
    /// <p class="note" role="note">Note: Some devices might not expose this key to the operating environment.</p>
    Power,
    /// The <code class="keycap">PowerOff</code> key. Sometime called <code class="keycap">PowerDown</code>.
    PowerOff,
    /// The <code class="keycap">Print Screen</code> or <code class="keycap">SnapShot</code> key, to initiate print-screen function.
    PrintScreen,
    /// The Hibernate key.
    /// This key saves the current state of the computer to disk so that it can be restored. The computer will then shutdown.
    Hibernate,
    /// The Standby key.
    /// This key turns off the display and places the computer into a low-power mode without completely shutting down.
    /// It is sometimes labelled <code class="keycap">Suspend</code> or <code class="keycap">Sleep</code> key. (<code class="android"><code class="android">KEYCODE_SLEEP</code></code>)
    Standby,
    /// The WakeUp key. (<code class="android">KEYCODE_WAKEUP</code>)
    WakeUp,
    /// The All Candidates key, to initate the multi-candidate mode.
    AllCandidates,
    /// The Alphanumeric key.
    Alphanumeric,
    /// The Code Input key, to initiate the Code Input mode to allow characters to be entered by their code points.
    CodeInput,
    /// The Compose key, also known as <em>Multi_key</em> on the X Window System.
    /// This key acts in a manner similar to a
    /// dead key, triggering a mode where subsequent key presses are combined to produce a different character.
    Compose,
    /// The Convert key, to convert the current input method sequence.
    Convert,
    /// A dead key combining key. It may be any combining key from any keyboard layout. For example, on a
    /// PC/AT French keyboard, using a French mapping and without any modifier activiated, this is the key value <code class="unicode">U+0302</code> COMBINING CIRCUMFLEX ACCENT. In another layout this might be a different unicode combining key.<br> For applications that need to differentiate between specific combining characters, the associated compositionupdate event’s data attribute provides the specific key value.
    Dead,
    /// The Final Mode <code class="keycap">Final</code> key used on some Asian keyboards, to enable the final mode for IMEs.
    FinalMode,
    /// Switch to the first character group. (ISO/IEC 9995)
    GroupFirst,
    /// Switch to the last character group. (ISO/IEC 9995)
    GroupLast,
    /// Switch to the next character group. (ISO/IEC 9995)
    GroupNext,
    /// Switch to the previous character group. (ISO/IEC 9995)
    GroupPrevious,
    /// The Mode Change key, to toggle between or cycle through input modes of IMEs.
    ModeChange,
    /// The Next Candidate function key.
    NextCandidate,
    /// The NonConvert ("Don’t Convert") key, to accept current input method sequence without conversion in IMEs.
    NonConvert,
    /// The Previous Candidate function key.
    PreviousCandidate,
    /// The Process key.
    Process,
    /// The Single Candidate function key.
    SingleCandidate,
    /// The Hangul (Korean characters) Mode key, to toggle between Hangul and English modes.
    HangulMode,
    /// The Hanja (Korean characters) Mode key.
    HanjaMode,
    /// The Junja (Korean characters) Mode key.
    JunjaMode,
    /// The Eisu key. This key may close the IME, but its purpose
    /// is defined by the current IME. (<code class="android">KEYCODE_EISU</code>)
    Eisu,
    /// The (Half-Width) Characters key.
    Hankaku,
    /// The Hiragana (Japanese Kana characters) key.
    Hiragana,
    /// The Hiragana/Katakana toggle key. (<code class="android">KEYCODE_KATAKANA_HIRAGANA</code>)
    HiraganaKatakana,
    /// The Kana Mode (Kana Lock) key. This key is used to enter
    /// hiragana mode (typically from romaji mode).
    KanaMode,
    /// The Kanji (Japanese name for ideographic characters of Chinese origin) Mode key.
    /// This key is typically used to switch to a hiragana keyboard for
    /// the purpose of converting input into kanji. (<code class="android">KEYCODE_KANA</code>)
    KanjiMode,
    /// The Katakana (Japanese Kana characters) key.
    Katakana,
    /// The Roman characters function key.
    Romaji,
    /// The Zenkaku (Full-Width) Characters key.
    Zenkaku,
    /// The Zenkaku/Hankaku (full-width/half-width) toggle key. (<code class="android">KEYCODE_ZENKAKU_HANKAKU</code>)
    ZenkakuHankaku,
    /// The F1 key, a general purpose function key, as index 1.
    F1,
    /// The F2 key, a general purpose function key, as index 2.
    F2,
    /// The F3 key, a general purpose function key, as index 3.
    F3,
    /// The F4 key, a general purpose function key, as index 4.
    F4,
    /// The F5 key, a general purpose function key, as index 5.
    F5,
    /// The F6 key, a general purpose function key, as index 6.
    F6,
    /// The F7 key, a general purpose function key, as index 7.
    F7,
    /// The F8 key, a general purpose function key, as index 8.
    F8,
    /// The F9 key, a general purpose function key, as index 9.
    F9,
    /// The F10 key, a general purpose function key, as index 10.
    F10,
    /// The F11 key, a general purpose function key, as index 11.
    F11,
    /// The F12 key, a general purpose function key, as index 12.
    F12,
    /// General purpose virtual function key, as index 1.
    Soft1,
    /// General purpose virtual function key, as index 2.
    Soft2,
    /// General purpose virtual function key, as index 3.
    Soft3,
    /// General purpose virtual function key, as index 4.
    Soft4,
    /// Select next (numerically or logically) lower channel. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MEDIA_CHANNEL_DOWN</code></code>, <code class="android"><code class="android">KEYCODE_CHANNEL_DOWN</code></code>)
    ChannelDown,
    /// Select next (numerically or logically) higher channel. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MEDIA_CHANNEL_UP</code></code>, <code class="android"><code class="android">KEYCODE_CHANNEL_UP</code></code>)
    ChannelUp,
    /// Close the current document or message (Note: This doesn’t close the application). (<code class="appcommand">APPCOMMAND_CLOSE</code>)
    Close,
    /// Open an editor to forward the current message. (<code class="appcommand">APPCOMMAND_FORWARD_MAIL</code>)
    MailForward,
    /// Open an editor to reply to the current message. (<code class="appcommand">APPCOMMAND_REPLY_TO_MAIL</code>)
    MailReply,
    /// Send the current message. (<code class="appcommand">APPCOMMAND_SEND_MAIL</code>)
    MailSend,
    /// Close the current media, for example to close a CD or DVD tray. (<code class="android">KEYCODE_MEDIA_CLOSE</code>)
    MediaClose,
    /// Initiate or continue forward playback at faster than normal speed, or increase speed if already fast forwarding. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MEDIA_FAST_FORWARD</code></code>, <code class="android"><code class="android">KEYCODE_MEDIA_FAST_FORWARD</code></code>)
    MediaFastForward,
    /// Pause the currently playing media. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MEDIA_PAUSE</code></code>, <code class="android"><code class="android">KEYCODE_MEDIA_PAUSE</code></code>)
    /// <p class="note" role="note">Media controller devices should use this value rather than <code class="key">"Pause"</code> for their pause keys.</p>
    MediaPause,
    /// Initiate or continue media playback at normal speed, if not currently playing at normal speed. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MEDIA_PLAY</code></code>, <code class="android"><code class="android">KEYCODE_MEDIA_PLAY</code></code>)
    MediaPlay,
    /// Toggle media between play and pause states. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MEDIA_PLAY_PAUSE</code></code>, <code class="android"><code class="android">KEYCODE_MEDIA_PLAY_PAUSE</code></code>)
    MediaPlayPause,
    /// Initiate or resume recording of currently selected media. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MEDIA_RECORD</code></code>, <code class="android"><code class="android">KEYCODE_MEDIA_RECORD</code></code>)
    MediaRecord,
    /// Initiate or continue reverse playback at faster than normal speed, or increase speed if already rewinding. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MEDIA_REWIND</code></code>, <code class="android"><code class="android">KEYCODE_MEDIA_REWIND</code></code>)
    MediaRewind,
    /// Stop media playing, pausing, forwarding, rewinding, or recording, if not already stopped. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MEDIA_STOP</code></code>, <code class="android"><code class="android">KEYCODE_MEDIA_STOP</code></code>)
    MediaStop,
    /// Seek to next media or program track. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MEDIA_NEXTTRACK</code></code>, <code class="android"><code class="android">KEYCODE_MEDIA_NEXT</code></code>)
    MediaTrackNext,
    /// Seek to previous media or program track. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MEDIA_PREVIOUSTRACK</code></code>, <code class="android"><code class="android">KEYCODE_MEDIA_PREVIOUS</code></code>)
    MediaTrackPrevious,
    /// Open a new document or message. (<code class="appcommand">APPCOMMAND_NEW</code>)
    New,
    /// Open an existing document or message. (<code class="appcommand">APPCOMMAND_OPEN</code>)
    Open,
    /// Print the current document or message. (<code class="appcommand">APPCOMMAND_PRINT</code>)
    Print,
    /// Save the current document or message. (<code class="appcommand">APPCOMMAND_SAVE</code>)
    Save,
    /// Spellcheck the current document or selection. (<code class="appcommand">APPCOMMAND_SPELL_CHECK</code>)
    SpellCheck,
    /// The <code class="keycap">11</code> key found on media numpads that
    /// have buttons from <code class="keycap">1</code> ... <code class="keycap">12</code>.
    Key11,
    /// The <code class="keycap">12</code> key found on media numpads that
    /// have buttons from <code class="keycap">1</code> ... <code class="keycap">12</code>.
    Key12,
    /// Adjust audio balance leftward. (<code class="vk">VK_AUDIO_BALANCE_LEFT</code>)
    AudioBalanceLeft,
    /// Adjust audio balance rightward. (<code class="vk">VK_AUDIO_BALANCE_RIGHT</code>)
    AudioBalanceRight,
    /// Decrease audio bass boost or cycle down through bass boost states. (<code class="appcommand"><code class="appcommand">APPCOMMAND_BASS_DOWN</code></code>, <code class="vk"><code class="vk">VK_BASS_BOOST_DOWN</code></code>)
    AudioBassBoostDown,
    /// Toggle bass boost on/off. (<code class="appcommand">APPCOMMAND_BASS_BOOST</code>)
    AudioBassBoostToggle,
    /// Increase audio bass boost or cycle up through bass boost states. (<code class="appcommand"><code class="appcommand">APPCOMMAND_BASS_UP</code></code>, <code class="vk"><code class="vk">VK_BASS_BOOST_UP</code></code>)
    AudioBassBoostUp,
    /// Adjust audio fader towards front. (<code class="vk">VK_FADER_FRONT</code>)
    AudioFaderFront,
    /// Adjust audio fader towards rear. (<code class="vk">VK_FADER_REAR</code>)
    AudioFaderRear,
    /// Advance surround audio mode to next available mode. (<code class="vk">VK_SURROUND_MODE_NEXT</code>)
    AudioSurroundModeNext,
    /// Decrease treble. (<code class="appcommand">APPCOMMAND_TREBLE_DOWN</code>)
    AudioTrebleDown,
    /// Increase treble. (<code class="appcommand">APPCOMMAND_TREBLE_UP</code>)
    AudioTrebleUp,
    /// Decrease audio volume. (<code class="appcommand"><code class="appcommand">APPCOMMAND_VOLUME_DOWN</code></code>, <code class="android"><code class="android">KEYCODE_VOLUME_DOWN</code></code>)
    AudioVolumeDown,
    /// Increase audio volume. (<code class="appcommand"><code class="appcommand">APPCOMMAND_VOLUME_UP</code></code>, <code class="android"><code class="android">KEYCODE_VOLUME_UP</code></code>)
    AudioVolumeUp,
    /// Toggle between muted state and prior volume level. (<code class="appcommand"><code class="appcommand">APPCOMMAND_VOLUME_MUTE</code></code>, <code class="android"><code class="android">KEYCODE_VOLUME_MUTE</code></code>)
    AudioVolumeMute,
    /// Toggle the microphone on/off. (<code class="appcommand">APPCOMMAND_MIC_ON_OFF_TOGGLE</code>)
    MicrophoneToggle,
    /// Decrease microphone volume. (<code class="appcommand">APPCOMMAND_MICROPHONE_VOLUME_DOWN</code>)
    MicrophoneVolumeDown,
    /// Increase microphone volume. (<code class="appcommand">APPCOMMAND_MICROPHONE_VOLUME_UP</code>)
    MicrophoneVolumeUp,
    /// Mute the microphone. (<code class="appcommand"><code class="appcommand">APPCOMMAND_MICROPHONE_VOLUME_MUTE</code></code>, <code class="android"><code class="android">KEYCODE_MUTE</code></code>)
    MicrophoneVolumeMute,
    /// Show correction list when a word is incorrectly identified. (<code class="appcommand">APPCOMMAND_CORRECTION_LIST</code>)
    SpeechCorrectionList,
    /// Toggle between dictation mode and command/control mode. (<code class="appcommand">APPCOMMAND_DICTATE_OR_COMMAND_CONTROL_TOGGLE</code>)
    SpeechInputToggle,
    /// The first generic "LaunchApplication" key. This is commonly associated with launching "My Computer", and may have a computer symbol on the key. (<code class="appcommand">APPCOMMAND_LAUNCH_APP1</code>)
    LaunchApplication1,
    /// The second generic "LaunchApplication" key. This is commonly associated with launching "Calculator", and may have a calculator symbol on the key. (<code class="appcommand"><code class="appcommand">APPCOMMAND_LAUNCH_APP2</code></code>, <code class="android"><code class="android">KEYCODE_CALCULATOR</code></code>)
    LaunchApplication2,
    /// The "Calendar" key. (<code class="android">KEYCODE_CALENDAR</code>)
    LaunchCalendar,
    /// The "Contacts" key. (<code class="android">KEYCODE_CONTACTS</code>)
    LaunchContacts,
    /// The "Mail" key. (<code class="appcommand">APPCOMMAND_LAUNCH_MAIL</code>)
    LaunchMail,
    /// The "Media Player" key. (<code class="appcommand">APPCOMMAND_LAUNCH_MEDIA_SELECT</code>)
    LaunchMediaPlayer,
    /// The "Music Player" key.
    LaunchMusicPlayer,
    /// The "Phone" key.
    LaunchPhone,
    /// The "Screen Saver" key.
    LaunchScreenSaver,
    /// The "Spreadsheet" key.
    LaunchSpreadsheet,
    /// The "Web Browser" key.
    LaunchWebBrowser,
    /// The "WebCam" key.
    LaunchWebCam,
    /// The "Word Processor" key.
    LaunchWordProcessor,
    /// Navigate to previous content or page in current history. (<code class="appcommand">APPCOMMAND_BROWSER_BACKWARD</code>)
    BrowserBack,
    /// Open the list of browser favorites. (<code class="appcommand">APPCOMMAND_BROWSER_FAVORITES</code>)
    BrowserFavorites,
    /// Navigate to next content or page in current history. (<code class="appcommand">APPCOMMAND_BROWSER_FORWARD</code>)
    BrowserForward,
    /// Go to the user’s preferred home page. (<code class="appcommand">APPCOMMAND_BROWSER_HOME</code>)
    BrowserHome,
    /// Refresh the current page or content. (<code class="appcommand">APPCOMMAND_BROWSER_REFRESH</code>)
    BrowserRefresh,
    /// Call up the user’s preferred search page. (<code class="appcommand">APPCOMMAND_BROWSER_SEARCH</code>)
    BrowserSearch,
    /// Stop loading the current page or content. (<code class="appcommand">APPCOMMAND_BROWSER_STOP</code>)
    BrowserStop,
    /// The Application switch key, which provides a list of recent apps to switch between. (<code class="android">KEYCODE_APP_SWITCH</code>)
    AppSwitch,
    /// The Call key. (<code class="android">KEYCODE_CALL</code>)
    Call,
    /// The Camera key. (<code class="android">KEYCODE_CAMERA</code>)
    Camera,
    /// The Camera focus key. (<code class="android">KEYCODE_FOCUS</code>)
    CameraFocus,
    /// The End Call key. (<code class="android">KEYCODE_ENDCALL</code>)
    EndCall,
    /// The Back key. (<code class="android">KEYCODE_BACK</code>)
    GoBack,
    /// The Home key, which goes to the phone’s main screen. (<code class="android">KEYCODE_HOME</code>)
    GoHome,
    /// The Headset Hook key. (<code class="android">KEYCODE_HEADSETHOOK</code>)
    HeadsetHook,
    /// The Last Number Redial key.
    LastNumberRedial,
    /// The Notification key. (<code class="android">KEYCODE_NOTIFICATION</code>)
    Notification,
    /// Toggle between manner mode state: silent, vibrate, ring, ... (<code class="android">KEYCODE_MANNER_MODE</code>)
    MannerMode,
    /// The Voice Dial key.
    VoiceDial,
    /// Switch to viewing TV. (<code class="android">KEYCODE_TV</code>)
    TV,
    /// TV 3D Mode. (<code class="android">KEYCODE_3D_MODE</code>)
    TV3DMode,
    /// Toggle between antenna and cable input. (<code class="android">KEYCODE_TV_ANTENNA_CABLE</code>)
    TVAntennaCable,
    /// Audio description. (<code class="android">KEYCODE_TV_AUDIO_DESCRIPTION</code>)
    TVAudioDescription,
    /// Audio description mixing volume down. (<code class="android">KEYCODE_TV_AUDIO_DESCRIPTION_MIX_DOWN</code>)
    TVAudioDescriptionMixDown,
    /// Audio description mixing volume up. (<code class="android">KEYCODE_TV_AUDIO_DESCRIPTION_MIX_UP</code>)
    TVAudioDescriptionMixUp,
    /// Contents menu. (<code class="android">KEYCODE_TV_CONTENTS_MENU</code>)
    TVContentsMenu,
    /// Contents menu. (<code class="android">KEYCODE_TV_DATA_SERVICE</code>)
    TVDataService,
    /// Switch the input mode on an external TV. (<code class="android">KEYCODE_TV_INPUT</code>)
    TVInput,
    /// Switch to component input #1. (<code class="android">KEYCODE_TV_INPUT_COMPONENT_1</code>)
    TVInputComponent1,
    /// Switch to component input #2. (<code class="android">KEYCODE_TV_INPUT_COMPONENT_2</code>)
    TVInputComponent2,
    /// Switch to composite input #1. (<code class="android">KEYCODE_TV_INPUT_COMPOSITE_1</code>)
    TVInputComposite1,
    /// Switch to composite input #2. (<code class="android">KEYCODE_TV_INPUT_COMPOSITE_2</code>)
    TVInputComposite2,
    /// Switch to HDMI input #1. (<code class="android">KEYCODE_TV_INPUT_HDMI_1</code>)
    TVInputHDMI1,
    /// Switch to HDMI input #2. (<code class="android">KEYCODE_TV_INPUT_HDMI_2</code>)
    TVInputHDMI2,
    /// Switch to HDMI input #3. (<code class="android">KEYCODE_TV_INPUT_HDMI_3</code>)
    TVInputHDMI3,
    /// Switch to HDMI input #4. (<code class="android">KEYCODE_TV_INPUT_HDMI_4</code>)
    TVInputHDMI4,
    /// Switch to VGA input #1. (<code class="android">KEYCODE_TV_INPUT_VGA_1</code>)
    TVInputVGA1,
    /// Media context menu. (<code class="android">KEYCODE_TV_MEDIA_CONTEXT_MENU</code>)
    TVMediaContext,
    /// Toggle network. (<code class="android">KEYCODE_TV_NETWORK</code>)
    TVNetwork,
    /// Number entry. (<code class="android">KEYCODE_TV_NUMBER_ENTRY</code>)
    TVNumberEntry,
    /// Toggle the power on an external TV. (<code class="android">KEYCODE_TV_POWER</code>)
    TVPower,
    /// Radio. (<code class="android">KEYCODE_TV_RADIO_SERVICE</code>)
    TVRadioService,
    /// Satellite. (<code class="android">KEYCODE_TV_SATELLITE</code>)
    TVSatellite,
    /// Broadcast Satellite. (<code class="android">KEYCODE_TV_SATELLITE_BS</code>)
    TVSatelliteBS,
    /// Communication Satellite. (<code class="android">KEYCODE_TV_SATELLITE_CS</code>)
    TVSatelliteCS,
    /// Toggle between available satellites. (<code class="android">KEYCODE_TV_SATELLITE_SERVICE</code>)
    TVSatelliteToggle,
    /// Analog Terrestrial. (<code class="android">KEYCODE_TV_TERRESTRIAL_ANALOG</code>)
    TVTerrestrialAnalog,
    /// Digital Terrestrial. (<code class="android">KEYCODE_TV_TERRESTRIAL_DIGITAL</code>)
    TVTerrestrialDigital,
    /// Timer programming. (<code class="android">KEYCODE_TV_TIMER_PROGRAMMING</code>)
    TVTimer,
    /// Switch the input mode on an external AVR (audio/video receiver). (<code class="android">KEYCODE_AVR_INPUT</code>)
    AVRInput,
    /// Toggle the power on an external AVR (audio/video receiver). (<code class="android">KEYCODE_AVR_POWER</code>)
    AVRPower,
    /// General purpose color-coded media function key, as index 0 (red). (<code class="vk"><code class="vk">VK_COLORED_KEY_0</code></code>, <code class="android"><code class="android">KEYCODE_PROG_RED</code></code>)
    ColorF0Red,
    /// General purpose color-coded media function key, as index 1 (green). (<code class="vk"><code class="vk">VK_COLORED_KEY_1</code></code>, <code class="android"><code class="android">KEYCODE_PROG_GREEN</code></code>)
    ColorF1Green,
    /// General purpose color-coded media function key, as index 2 (yellow). (<code class="vk"><code class="vk">VK_COLORED_KEY_2</code></code>, <code class="android"><code class="android">KEYCODE_PROG_YELLOW</code></code>)
    ColorF2Yellow,
    /// General purpose color-coded media function key, as index 3 (blue). (<code class="vk"><code class="vk">VK_COLORED_KEY_3</code></code>, <code class="android"><code class="android">KEYCODE_PROG_BLUE</code></code>)
    ColorF3Blue,
    /// General purpose color-coded media function key, as index 4 (grey). (<code class="vk">VK_COLORED_KEY_4</code>)
    ColorF4Grey,
    /// General purpose color-coded media function key, as index 5 (brown). (<code class="vk">VK_COLORED_KEY_5</code>)
    ColorF5Brown,
    /// Toggle the display of Closed Captions. (<code class="vk"><code class="vk">VK_CC</code></code>, <code class="android"><code class="android">KEYCODE_CAPTIONS</code></code>)
    ClosedCaptionToggle,
    /// Adjust brightness of device, by toggling between or cycling through states. (<code class="vk">VK_DIMMER</code>)
    Dimmer,
    /// Swap video sources. (<code class="vk">VK_DISPLAY_SWAP</code>)
    DisplaySwap,
    /// Select Digital Video Rrecorder. (<code class="android">KEYCODE_DVR</code>)
    DVR,
    /// Exit the current application. (<code class="vk">VK_EXIT</code>)
    Exit,
    /// Clear program or content stored as favorite 0. (<code class="vk">VK_CLEAR_FAVORITE_0</code>)
    FavoriteClear0,
    /// Clear program or content stored as favorite 1. (<code class="vk">VK_CLEAR_FAVORITE_1</code>)
    FavoriteClear1,
    /// Clear program or content stored as favorite 2. (<code class="vk">VK_CLEAR_FAVORITE_2</code>)
    FavoriteClear2,
    /// Clear program or content stored as favorite 3. (<code class="vk">VK_CLEAR_FAVORITE_3</code>)
    FavoriteClear3,
    /// Select (recall) program or content stored as favorite 0. (<code class="vk">VK_RECALL_FAVORITE_0</code>)
    FavoriteRecall0,
    /// Select (recall) program or content stored as favorite 1. (<code class="vk">VK_RECALL_FAVORITE_1</code>)
    FavoriteRecall1,
    /// Select (recall) program or content stored as favorite 2. (<code class="vk">VK_RECALL_FAVORITE_2</code>)
    FavoriteRecall2,
    /// Select (recall) program or content stored as favorite 3. (<code class="vk">VK_RECALL_FAVORITE_3</code>)
    FavoriteRecall3,
    /// Store current program or content as favorite 0. (<code class="vk">VK_STORE_FAVORITE_0</code>)
    FavoriteStore0,
    /// Store current program or content as favorite 1. (<code class="vk">VK_STORE_FAVORITE_1</code>)
    FavoriteStore1,
    /// Store current program or content as favorite 2. (<code class="vk">VK_STORE_FAVORITE_2</code>)
    FavoriteStore2,
    /// Store current program or content as favorite 3. (<code class="vk">VK_STORE_FAVORITE_3</code>)
    FavoriteStore3,
    /// Toggle display of program or content guide. (<code class="vk"><code class="vk">VK_GUIDE</code></code>, <code class="android"><code class="android">KEYCODE_GUIDE</code></code>)
    Guide,
    /// If guide is active and displayed, then display next day’s content. (<code class="vk">VK_NEXT_DAY</code>)
    GuideNextDay,
    /// If guide is active and displayed, then display previous day’s content. (<code class="vk">VK_PREV_DAY</code>)
    GuidePreviousDay,
    /// Toggle display of information about currently selected context or media. (<code class="vk"><code class="vk">VK_INFO</code></code>, <code class="android"><code class="android">KEYCODE_INFO</code></code>)
    Info,
    /// Toggle instant replay. (<code class="vk">VK_INSTANT_REPLAY</code>)
    InstantReplay,
    /// Launch linked content, if available and appropriate. (<code class="vk">VK_LINK</code>)
    Link,
    /// List the current program. (<code class="vk">VK_LIST</code>)
    ListProgram,
    /// Toggle display listing of currently available live content or programs. (<code class="vk">VK_LIVE</code>)
    LiveContent,
    /// Lock or unlock current content or program. (<code class="vk">VK_LOCK</code>)
    Lock,
    /// Show a list of media applications: audio/video players and image viewers. (<code class="vk">VK_APPS</code>)
    /// <p class="note" role="note">Do not confuse this key value with the Windows' <code class="vk"><code class="vk">VK_APPS</code></code> / <code class="vk"><code class="vk">VK_CONTEXT_MENU</code></code> key, which is encoded as <code class="key">"ContextMenu"</code>.</p>
    MediaApps,
    /// Audio track key. (<code class="android">KEYCODE_MEDIA_AUDIO_TRACK</code>)
    MediaAudioTrack,
    /// Select previously selected channel or media. (<code class="vk"><code class="vk">VK_LAST</code></code>, <code class="android"><code class="android">KEYCODE_LAST_CHANNEL</code></code>)
    MediaLast,
    /// Skip backward to next content or program. (<code class="android">KEYCODE_MEDIA_SKIP_BACKWARD</code>)
    MediaSkipBackward,
    /// Skip forward to next content or program. (<code class="vk"><code class="vk">VK_SKIP</code></code>, <code class="android"><code class="android">KEYCODE_MEDIA_SKIP_FORWARD</code></code>)
    MediaSkipForward,
    /// Step backward to next content or program. (<code class="android">KEYCODE_MEDIA_STEP_BACKWARD</code>)
    MediaStepBackward,
    /// Step forward to next content or program. (<code class="android">KEYCODE_MEDIA_STEP_FORWARD</code>)
    MediaStepForward,
    /// Media top menu. (<code class="android">KEYCODE_MEDIA_TOP_MENU</code>)
    MediaTopMenu,
    /// Navigate in. (<code class="android">KEYCODE_NAVIGATE_IN</code>)
    NavigateIn,
    /// Navigate to next key. (<code class="android">KEYCODE_NAVIGATE_NEXT</code>)
    NavigateNext,
    /// Navigate out. (<code class="android">KEYCODE_NAVIGATE_OUT</code>)
    NavigateOut,
    /// Navigate to previous key. (<code class="android">KEYCODE_NAVIGATE_PREVIOUS</code>)
    NavigatePrevious,
    /// Cycle to next favorite channel (in favorites list). (<code class="vk">VK_NEXT_FAVORITE_CHANNEL</code>)
    NextFavoriteChannel,
    /// Cycle to next user profile (if there are multiple user profiles). (<code class="vk">VK_USER</code>)
    NextUserProfile,
    /// Access on-demand content or programs. (<code class="vk">VK_ON_DEMAND</code>)
    OnDemand,
    /// Pairing key to pair devices. (<code class="android">KEYCODE_PAIRING</code>)
    Pairing,
    /// Move picture-in-picture window down. (<code class="vk">VK_PINP_DOWN</code>)
    PinPDown,
    /// Move picture-in-picture window. (<code class="vk">VK_PINP_MOVE</code>)
    PinPMove,
    /// Toggle display of picture-in-picture window. (<code class="vk">VK_PINP_TOGGLE</code>)
    PinPToggle,
    /// Move picture-in-picture window up. (<code class="vk">VK_PINP_UP</code>)
    PinPUp,
    /// Decrease media playback speed. (<code class="vk">VK_PLAY_SPEED_DOWN</code>)
    PlaySpeedDown,
    /// Reset playback to normal speed. (<code class="vk">VK_PLAY_SPEED_RESET</code>)
    PlaySpeedReset,
    /// Increase media playback speed. (<code class="vk">VK_PLAY_SPEED_UP</code>)
    PlaySpeedUp,
    /// Toggle random media or content shuffle mode. (<code class="vk">VK_RANDOM_TOGGLE</code>)
    RandomToggle,
    /// Not a physical key, but this key code is sent when the remote control battery is low. (<code class="vk">VK_RC_LOW_BATTERY</code>)
    RcLowBattery,
    /// Toggle or cycle between media recording speeds. (<code class="vk">VK_RECORD_SPEED_NEXT</code>)
    RecordSpeedNext,
    /// Toggle RF (radio frequency) input bypass mode (pass RF input directly to the RF output). (<code class="vk">VK_RF_BYPASS</code>)
    RfBypass,
    /// Toggle scan channels mode. (<code class="vk">VK_SCAN_CHANNELS_TOGGLE</code>)
    ScanChannelsToggle,
    /// Advance display screen mode to next available mode. (<code class="vk">VK_SCREEN_MODE_NEXT</code>)
    ScreenModeNext,
    /// Toggle display of device settings screen. (<code class="vk"><code class="vk">VK_SETTINGS</code></code>, <code class="android"><code class="android">KEYCODE_SETTINGS</code></code>)
    Settings,
    /// Toggle split screen mode. (<code class="vk">VK_SPLIT_SCREEN_TOGGLE</code>)
    SplitScreenToggle,
    /// Switch the input mode on an external STB (set top box). (<code class="android">KEYCODE_STB_INPUT</code>)
    STBInput,
    /// Toggle the power on an external STB (set top box). (<code class="android">KEYCODE_STB_POWER</code>)
    STBPower,
    /// Toggle display of subtitles, if available. (<code class="vk">VK_SUBTITLE</code>)
    Subtitle,
    /// Toggle display of teletext, if available (<code class="vk"><code class="vk">VK_TELETEXT</code></code>, <code class="android"><code class="android">KEYCODE_TV_TELETEXT</code></code>).
    Teletext,
    /// Advance video mode to next available mode. (<code class="vk">VK_VIDEO_MODE_NEXT</code>)
    VideoModeNext,
    /// Cause device to identify itself in some manner, e.g., audibly or visibly. (<code class="vk">VK_WINK</code>)
    Wink,
    /// Toggle between full-screen and scaled content, or alter magnification level. (<code class="vk"><code class="vk">VK_ZOOM</code></code>, <code class="android"><code class="android">KEYCODE_TV_ZOOM_MODE</code></code>)
    ZoomToggle,

    #[doc(hidden)]
    __Nonexhaustive,
}

impl Key {
    const VARIANTS: &'static [&'static str] = &[
        "Character",
    
        "Unidentified",
        "Alt",
        "AltGraph",
        "CapsLock",
        "Control",
        "Fn",
        "FnLock",
        "Meta",
        "NumLock",
        "ScrollLock",
        "Shift",
        "Symbol",
        "SymbolLock",
        "Hyper",
        "Super",
        "Enter",
        "Tab",
        "ArrowDown",
        "ArrowLeft",
        "ArrowRight",
        "ArrowUp",
        "End",
        "Home",
        "PageDown",
        "PageUp",
        "Backspace",
        "Clear",
        "Copy",
        "CrSel",
        "Cut",
        "Delete",
        "EraseEof",
        "ExSel",
        "Insert",
        "Paste",
        "Redo",
        "Undo",
        "Accept",
        "Again",
        "Attn",
        "Cancel",
        "ContextMenu",
        "Escape",
        "Execute",
        "Find",
        "Help",
        "Pause",
        "Play",
        "Props",
        "Select",
        "ZoomIn",
        "ZoomOut",
        "BrightnessDown",
        "BrightnessUp",
        "Eject",
        "LogOff",
        "Power",
        "PowerOff",
        "PrintScreen",
        "Hibernate",
        "Standby",
        "WakeUp",
        "AllCandidates",
        "Alphanumeric",
        "CodeInput",
        "Compose",
        "Convert",
        "Dead",
        "FinalMode",
        "GroupFirst",
        "GroupLast",
        "GroupNext",
        "GroupPrevious",
        "ModeChange",
        "NextCandidate",
        "NonConvert",
        "PreviousCandidate",
        "Process",
        "SingleCandidate",
        "HangulMode",
        "HanjaMode",
        "JunjaMode",
        "Eisu",
        "Hankaku",
        "Hiragana",
        "HiraganaKatakana",
        "KanaMode",
        "KanjiMode",
        "Katakana",
        "Romaji",
        "Zenkaku",
        "ZenkakuHankaku",
        "F1",
        "F2",
        "F3",
        "F4",
        "F5",
        "F6",
        "F7",
        "F8",
        "F9",
        "F10",
        "F11",
        "F12",
        "Soft1",
        "Soft2",
        "Soft3",
        "Soft4",
        "ChannelDown",
        "ChannelUp",
        "Close",
        "MailForward",
        "MailReply",
        "MailSend",
        "MediaClose",
        "MediaFastForward",
        "MediaPause",
        "MediaPlay",
        "MediaPlayPause",
        "MediaRecord",
        "MediaRewind",
        "MediaStop",
        "MediaTrackNext",
        "MediaTrackPrevious",
        "New",
        "Open",
        "Print",
        "Save",
        "SpellCheck",
        "Key11",
        "Key12",
        "AudioBalanceLeft",
        "AudioBalanceRight",
        "AudioBassBoostDown",
        "AudioBassBoostToggle",
        "AudioBassBoostUp",
        "AudioFaderFront",
        "AudioFaderRear",
        "AudioSurroundModeNext",
        "AudioTrebleDown",
        "AudioTrebleUp",
        "AudioVolumeDown",
        "AudioVolumeUp",
        "AudioVolumeMute",
        "MicrophoneToggle",
        "MicrophoneVolumeDown",
        "MicrophoneVolumeUp",
        "MicrophoneVolumeMute",
        "SpeechCorrectionList",
        "SpeechInputToggle",
        "LaunchApplication1",
        "LaunchApplication2",
        "LaunchCalendar",
        "LaunchContacts",
        "LaunchMail",
        "LaunchMediaPlayer",
        "LaunchMusicPlayer",
        "LaunchPhone",
        "LaunchScreenSaver",
        "LaunchSpreadsheet",
        "LaunchWebBrowser",
        "LaunchWebCam",
        "LaunchWordProcessor",
        "BrowserBack",
        "BrowserFavorites",
        "BrowserForward",
        "BrowserHome",
        "BrowserRefresh",
        "BrowserSearch",
        "BrowserStop",
        "AppSwitch",
        "Call",
        "Camera",
        "CameraFocus",
        "EndCall",
        "GoBack",
        "GoHome",
        "HeadsetHook",
        "LastNumberRedial",
        "Notification",
        "MannerMode",
        "VoiceDial",
        "TV",
        "TV3DMode",
        "TVAntennaCable",
        "TVAudioDescription",
        "TVAudioDescriptionMixDown",
        "TVAudioDescriptionMixUp",
        "TVContentsMenu",
        "TVDataService",
        "TVInput",
        "TVInputComponent1",
        "TVInputComponent2",
        "TVInputComposite1",
        "TVInputComposite2",
        "TVInputHDMI1",
        "TVInputHDMI2",
        "TVInputHDMI3",
        "TVInputHDMI4",
        "TVInputVGA1",
        "TVMediaContext",
        "TVNetwork",
        "TVNumberEntry",
        "TVPower",
        "TVRadioService",
        "TVSatellite",
        "TVSatelliteBS",
        "TVSatelliteCS",
        "TVSatelliteToggle",
        "TVTerrestrialAnalog",
        "TVTerrestrialDigital",
        "TVTimer",
        "AVRInput",
        "AVRPower",
        "ColorF0Red",
        "ColorF1Green",
        "ColorF2Yellow",
        "ColorF3Blue",
        "ColorF4Grey",
        "ColorF5Brown",
        "ClosedCaptionToggle",
        "Dimmer",
        "DisplaySwap",
        "DVR",
        "Exit",
        "FavoriteClear0",
        "FavoriteClear1",
        "FavoriteClear2",
        "FavoriteClear3",
        "FavoriteRecall0",
        "FavoriteRecall1",
        "FavoriteRecall2",
        "FavoriteRecall3",
        "FavoriteStore0",
        "FavoriteStore1",
        "FavoriteStore2",
        "FavoriteStore3",
        "Guide",
        "GuideNextDay",
        "GuidePreviousDay",
        "Info",
        "InstantReplay",
        "Link",
        "ListProgram",
        "LiveContent",
        "Lock",
        "MediaApps",
        "MediaAudioTrack",
        "MediaLast",
        "MediaSkipBackward",
        "MediaSkipForward",
        "MediaStepBackward",
        "MediaStepForward",
        "MediaTopMenu",
        "NavigateIn",
        "NavigateNext",
        "NavigateOut",
        "NavigatePrevious",
        "NextFavoriteChannel",
        "NextUserProfile",
        "OnDemand",
        "Pairing",
        "PinPDown",
        "PinPMove",
        "PinPToggle",
        "PinPUp",
        "PlaySpeedDown",
        "PlaySpeedReset",
        "PlaySpeedUp",
        "RandomToggle",
        "RcLowBattery",
        "RecordSpeedNext",
        "RfBypass",
        "ScanChannelsToggle",
        "ScreenModeNext",
        "Settings",
        "SplitScreenToggle",
        "STBInput",
        "STBPower",
        "Subtitle",
        "Teletext",
        "VideoModeNext",
        "Wink",
        "ZoomToggle",

    ];
    fn discriminant(&self) -> u16 {
        use Key::*;
        match self {
            Character(_) => 0,
    
            Unidentified => 1,
            Alt => 2,
            AltGraph => 3,
            CapsLock => 4,
            Control => 5,
            Fn => 6,
            FnLock => 7,
            Meta => 8,
            NumLock => 9,
            ScrollLock => 10,
            Shift => 11,
            Symbol => 12,
            SymbolLock => 13,
            Hyper => 14,
            Super => 15,
            Enter => 16,
            Tab => 17,
            ArrowDown => 18,
            ArrowLeft => 19,
            ArrowRight => 20,
            ArrowUp => 21,
            End => 22,
            Home => 23,
            PageDown => 24,
            PageUp => 25,
            Backspace => 26,
            Clear => 27,
            Copy => 28,
            CrSel => 29,
            Cut => 30,
            Delete => 31,
            EraseEof => 32,
            ExSel => 33,
            Insert => 34,
            Paste => 35,
            Redo => 36,
            Undo => 37,
            Accept => 38,
            Again => 39,
            Attn => 40,
            Cancel => 41,
            ContextMenu => 42,
            Escape => 43,
            Execute => 44,
            Find => 45,
            Help => 46,
            Pause => 47,
            Play => 48,
            Props => 49,
            Select => 50,
            ZoomIn => 51,
            ZoomOut => 52,
            BrightnessDown => 53,
            BrightnessUp => 54,
            Eject => 55,
            LogOff => 56,
            Power => 57,
            PowerOff => 58,
            PrintScreen => 59,
            Hibernate => 60,
            Standby => 61,
            WakeUp => 62,
            AllCandidates => 63,
            Alphanumeric => 64,
            CodeInput => 65,
            Compose => 66,
            Convert => 67,
            Dead => 68,
            FinalMode => 69,
            GroupFirst => 70,
            GroupLast => 71,
            GroupNext => 72,
            GroupPrevious => 73,
            ModeChange => 74,
            NextCandidate => 75,
            NonConvert => 76,
            PreviousCandidate => 77,
            Process => 78,
            SingleCandidate => 79,
            HangulMode => 80,
            HanjaMode => 81,
            JunjaMode => 82,
            Eisu => 83,
            Hankaku => 84,
            Hiragana => 85,
            HiraganaKatakana => 86,
            KanaMode => 87,
            KanjiMode => 88,
            Katakana => 89,
            Romaji => 90,
            Zenkaku => 91,
            ZenkakuHankaku => 92,
            F1 => 93,
            F2 => 94,
            F3 => 95,
            F4 => 96,
            F5 => 97,
            F6 => 98,
            F7 => 99,
            F8 => 100,
            F9 => 101,
            F10 => 102,
            F11 => 103,
            F12 => 104,
            Soft1 => 105,
            Soft2 => 106,
            Soft3 => 107,
            Soft4 => 108,
            ChannelDown => 109,
            ChannelUp => 110,
            Close => 111,
            MailForward => 112,
            MailReply => 113,
            MailSend => 114,
            MediaClose => 115,
            MediaFastForward => 116,
            MediaPause => 117,
            MediaPlay => 118,
            MediaPlayPause => 119,
            MediaRecord => 120,
            MediaRewind => 121,
            MediaStop => 122,
            MediaTrackNext => 123,
            MediaTrackPrevious => 124,
            New => 125,
            Open => 126,
            Print => 127,
            Save => 128,
            SpellCheck => 129,
            Key11 => 130,
            Key12 => 131,
            AudioBalanceLeft => 132,
            AudioBalanceRight => 133,
            AudioBassBoostDown => 134,
            AudioBassBoostToggle => 135,
            AudioBassBoostUp => 136,
            AudioFaderFront => 137,
            AudioFaderRear => 138,
            AudioSurroundModeNext => 139,
            AudioTrebleDown => 140,
            AudioTrebleUp => 141,
            AudioVolumeDown => 142,
            AudioVolumeUp => 143,
            AudioVolumeMute => 144,
            MicrophoneToggle => 145,
            MicrophoneVolumeDown => 146,
            MicrophoneVolumeUp => 147,
            MicrophoneVolumeMute => 148,
            SpeechCorrectionList => 149,
            SpeechInputToggle => 150,
            LaunchApplication1 => 151,
            LaunchApplication2 => 152,
            LaunchCalendar => 153,
            LaunchContacts => 154,
            LaunchMail => 155,
            LaunchMediaPlayer => 156,
            LaunchMusicPlayer => 157,
            LaunchPhone => 158,
            LaunchScreenSaver => 159,
            LaunchSpreadsheet => 160,
            LaunchWebBrowser => 161,
            LaunchWebCam => 162,
            LaunchWordProcessor => 163,
            BrowserBack => 164,
            BrowserFavorites => 165,
            BrowserForward => 166,
            BrowserHome => 167,
            BrowserRefresh => 168,
            BrowserSearch => 169,
            BrowserStop => 170,
            AppSwitch => 171,
            Call => 172,
            Camera => 173,
            CameraFocus => 174,
            EndCall => 175,
            GoBack => 176,
            GoHome => 177,
            HeadsetHook => 178,
            LastNumberRedial => 179,
            Notification => 180,
            MannerMode => 181,
            VoiceDial => 182,
            TV => 183,
            TV3DMode => 184,
            TVAntennaCable => 185,
            TVAudioDescription => 186,
            TVAudioDescriptionMixDown => 187,
            TVAudioDescriptionMixUp => 188,
            TVContentsMenu => 189,
            TVDataService => 190,
            TVInput => 191,
            TVInputComponent1 => 192,
            TVInputComponent2 => 193,
            TVInputComposite1 => 194,
            TVInputComposite2 => 195,
            TVInputHDMI1 => 196,
            TVInputHDMI2 => 197,
            TVInputHDMI3 => 198,
            TVInputHDMI4 => 199,
            TVInputVGA1 => 200,
            TVMediaContext => 201,
            TVNetwork => 202,
            TVNumberEntry => 203,
            TVPower => 204,
            TVRadioService => 205,
            TVSatellite => 206,
            TVSatelliteBS => 207,
            TVSatelliteCS => 208,
            TVSatelliteToggle => 209,
            TVTerrestrialAnalog => 210,
            TVTerrestrialDigital => 211,
            TVTimer => 212,
            AVRInput => 213,
            AVRPower => 214,
            ColorF0Red => 215,
            ColorF1Green => 216,
            ColorF2Yellow => 217,
            ColorF3Blue => 218,
            ColorF4Grey => 219,
            ColorF5Brown => 220,
            ClosedCaptionToggle => 221,
            Dimmer => 222,
            DisplaySwap => 223,
            DVR => 224,
            Exit => 225,
            FavoriteClear0 => 226,
            FavoriteClear1 => 227,
            FavoriteClear2 => 228,
            FavoriteClear3 => 229,
            FavoriteRecall0 => 230,
            FavoriteRecall1 => 231,
            FavoriteRecall2 => 232,
            FavoriteRecall3 => 233,
            FavoriteStore0 => 234,
            FavoriteStore1 => 235,
            FavoriteStore2 => 236,
            FavoriteStore3 => 237,
            Guide => 238,
            GuideNextDay => 239,
            GuidePreviousDay => 240,
            Info => 241,
            InstantReplay => 242,
            Link => 243,
            ListProgram => 244,
            LiveContent => 245,
            Lock => 246,
            MediaApps => 247,
            MediaAudioTrack => 248,
            MediaLast => 249,
            MediaSkipBackward => 250,
            MediaSkipForward => 251,
            MediaStepBackward => 252,
            MediaStepForward => 253,
            MediaTopMenu => 254,
            NavigateIn => 255,
            NavigateNext => 256,
            NavigateOut => 257,
            NavigatePrevious => 258,
            NextFavoriteChannel => 259,
            NextUserProfile => 260,
            OnDemand => 261,
            Pairing => 262,
            PinPDown => 263,
            PinPMove => 264,
            PinPToggle => 265,
            PinPUp => 266,
            PlaySpeedDown => 267,
            PlaySpeedReset => 268,
            PlaySpeedUp => 269,
            RandomToggle => 270,
            RcLowBattery => 271,
            RecordSpeedNext => 272,
            RfBypass => 273,
            ScanChannelsToggle => 274,
            ScreenModeNext => 275,
            Settings => 276,
            SplitScreenToggle => 277,
            STBInput => 278,
            STBPower => 279,
            Subtitle => 280,
            Teletext => 281,
            VideoModeNext => 282,
            Wink => 283,
            ZoomToggle => 284,

            __Nonexhaustive => unreachable!(),
        }
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            Key::Character(s) => s,
            _ => Key::VARIANTS[self.discriminant() as usize],
        })
    }
}

impl FromStr for Key {
    type Err = UnrecognizedKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Key::*;
        match s {
            s if is_key_string(s) => Ok(Character(s.to_string())),
            "Unidentified" => Ok(Unidentified),
            "Alt" => Ok(Alt),
            "AltGraph" => Ok(AltGraph),
            "CapsLock" => Ok(CapsLock),
            "Control" => Ok(Control),
            "Fn" => Ok(Fn),
            "FnLock" => Ok(FnLock),
            "Meta" => Ok(Meta),
            "NumLock" => Ok(NumLock),
            "ScrollLock" => Ok(ScrollLock),
            "Shift" => Ok(Shift),
            "Symbol" => Ok(Symbol),
            "SymbolLock" => Ok(SymbolLock),
            "Hyper" => Ok(Hyper),
            "Super" => Ok(Super),
            "Enter" => Ok(Enter),
            "Tab" => Ok(Tab),
            "ArrowDown" => Ok(ArrowDown),
            "ArrowLeft" => Ok(ArrowLeft),
            "ArrowRight" => Ok(ArrowRight),
            "ArrowUp" => Ok(ArrowUp),
            "End" => Ok(End),
            "Home" => Ok(Home),
            "PageDown" => Ok(PageDown),
            "PageUp" => Ok(PageUp),
            "Backspace" => Ok(Backspace),
            "Clear" => Ok(Clear),
            "Copy" => Ok(Copy),
            "CrSel" => Ok(CrSel),
            "Cut" => Ok(Cut),
            "Delete" => Ok(Delete),
            "EraseEof" => Ok(EraseEof),
            "ExSel" => Ok(ExSel),
            "Insert" => Ok(Insert),
            "Paste" => Ok(Paste),
            "Redo" => Ok(Redo),
            "Undo" => Ok(Undo),
            "Accept" => Ok(Accept),
            "Again" => Ok(Again),
            "Attn" => Ok(Attn),
            "Cancel" => Ok(Cancel),
            "ContextMenu" => Ok(ContextMenu),
            "Escape" => Ok(Escape),
            "Execute" => Ok(Execute),
            "Find" => Ok(Find),
            "Help" => Ok(Help),
            "Pause" => Ok(Pause),
            "Play" => Ok(Play),
            "Props" => Ok(Props),
            "Select" => Ok(Select),
            "ZoomIn" => Ok(ZoomIn),
            "ZoomOut" => Ok(ZoomOut),
            "BrightnessDown" => Ok(BrightnessDown),
            "BrightnessUp" => Ok(BrightnessUp),
            "Eject" => Ok(Eject),
            "LogOff" => Ok(LogOff),
            "Power" => Ok(Power),
            "PowerOff" => Ok(PowerOff),
            "PrintScreen" => Ok(PrintScreen),
            "Hibernate" => Ok(Hibernate),
            "Standby" => Ok(Standby),
            "WakeUp" => Ok(WakeUp),
            "AllCandidates" => Ok(AllCandidates),
            "Alphanumeric" => Ok(Alphanumeric),
            "CodeInput" => Ok(CodeInput),
            "Compose" => Ok(Compose),
            "Convert" => Ok(Convert),
            "Dead" => Ok(Dead),
            "FinalMode" => Ok(FinalMode),
            "GroupFirst" => Ok(GroupFirst),
            "GroupLast" => Ok(GroupLast),
            "GroupNext" => Ok(GroupNext),
            "GroupPrevious" => Ok(GroupPrevious),
            "ModeChange" => Ok(ModeChange),
            "NextCandidate" => Ok(NextCandidate),
            "NonConvert" => Ok(NonConvert),
            "PreviousCandidate" => Ok(PreviousCandidate),
            "Process" => Ok(Process),
            "SingleCandidate" => Ok(SingleCandidate),
            "HangulMode" => Ok(HangulMode),
            "HanjaMode" => Ok(HanjaMode),
            "JunjaMode" => Ok(JunjaMode),
            "Eisu" => Ok(Eisu),
            "Hankaku" => Ok(Hankaku),
            "Hiragana" => Ok(Hiragana),
            "HiraganaKatakana" => Ok(HiraganaKatakana),
            "KanaMode" => Ok(KanaMode),
            "KanjiMode" => Ok(KanjiMode),
            "Katakana" => Ok(Katakana),
            "Romaji" => Ok(Romaji),
            "Zenkaku" => Ok(Zenkaku),
            "ZenkakuHankaku" => Ok(ZenkakuHankaku),
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
            "Soft1" => Ok(Soft1),
            "Soft2" => Ok(Soft2),
            "Soft3" => Ok(Soft3),
            "Soft4" => Ok(Soft4),
            "ChannelDown" => Ok(ChannelDown),
            "ChannelUp" => Ok(ChannelUp),
            "Close" => Ok(Close),
            "MailForward" => Ok(MailForward),
            "MailReply" => Ok(MailReply),
            "MailSend" => Ok(MailSend),
            "MediaClose" => Ok(MediaClose),
            "MediaFastForward" => Ok(MediaFastForward),
            "MediaPause" => Ok(MediaPause),
            "MediaPlay" => Ok(MediaPlay),
            "MediaPlayPause" => Ok(MediaPlayPause),
            "MediaRecord" => Ok(MediaRecord),
            "MediaRewind" => Ok(MediaRewind),
            "MediaStop" => Ok(MediaStop),
            "MediaTrackNext" => Ok(MediaTrackNext),
            "MediaTrackPrevious" => Ok(MediaTrackPrevious),
            "New" => Ok(New),
            "Open" => Ok(Open),
            "Print" => Ok(Print),
            "Save" => Ok(Save),
            "SpellCheck" => Ok(SpellCheck),
            "Key11" => Ok(Key11),
            "Key12" => Ok(Key12),
            "AudioBalanceLeft" => Ok(AudioBalanceLeft),
            "AudioBalanceRight" => Ok(AudioBalanceRight),
            "AudioBassBoostDown" => Ok(AudioBassBoostDown),
            "AudioBassBoostToggle" => Ok(AudioBassBoostToggle),
            "AudioBassBoostUp" => Ok(AudioBassBoostUp),
            "AudioFaderFront" => Ok(AudioFaderFront),
            "AudioFaderRear" => Ok(AudioFaderRear),
            "AudioSurroundModeNext" => Ok(AudioSurroundModeNext),
            "AudioTrebleDown" => Ok(AudioTrebleDown),
            "AudioTrebleUp" => Ok(AudioTrebleUp),
            "AudioVolumeDown" => Ok(AudioVolumeDown),
            "AudioVolumeUp" => Ok(AudioVolumeUp),
            "AudioVolumeMute" => Ok(AudioVolumeMute),
            "MicrophoneToggle" => Ok(MicrophoneToggle),
            "MicrophoneVolumeDown" => Ok(MicrophoneVolumeDown),
            "MicrophoneVolumeUp" => Ok(MicrophoneVolumeUp),
            "MicrophoneVolumeMute" => Ok(MicrophoneVolumeMute),
            "SpeechCorrectionList" => Ok(SpeechCorrectionList),
            "SpeechInputToggle" => Ok(SpeechInputToggle),
            "LaunchApplication1" => Ok(LaunchApplication1),
            "LaunchApplication2" => Ok(LaunchApplication2),
            "LaunchCalendar" => Ok(LaunchCalendar),
            "LaunchContacts" => Ok(LaunchContacts),
            "LaunchMail" => Ok(LaunchMail),
            "LaunchMediaPlayer" => Ok(LaunchMediaPlayer),
            "LaunchMusicPlayer" => Ok(LaunchMusicPlayer),
            "LaunchPhone" => Ok(LaunchPhone),
            "LaunchScreenSaver" => Ok(LaunchScreenSaver),
            "LaunchSpreadsheet" => Ok(LaunchSpreadsheet),
            "LaunchWebBrowser" => Ok(LaunchWebBrowser),
            "LaunchWebCam" => Ok(LaunchWebCam),
            "LaunchWordProcessor" => Ok(LaunchWordProcessor),
            "BrowserBack" => Ok(BrowserBack),
            "BrowserFavorites" => Ok(BrowserFavorites),
            "BrowserForward" => Ok(BrowserForward),
            "BrowserHome" => Ok(BrowserHome),
            "BrowserRefresh" => Ok(BrowserRefresh),
            "BrowserSearch" => Ok(BrowserSearch),
            "BrowserStop" => Ok(BrowserStop),
            "AppSwitch" => Ok(AppSwitch),
            "Call" => Ok(Call),
            "Camera" => Ok(Camera),
            "CameraFocus" => Ok(CameraFocus),
            "EndCall" => Ok(EndCall),
            "GoBack" => Ok(GoBack),
            "GoHome" => Ok(GoHome),
            "HeadsetHook" => Ok(HeadsetHook),
            "LastNumberRedial" => Ok(LastNumberRedial),
            "Notification" => Ok(Notification),
            "MannerMode" => Ok(MannerMode),
            "VoiceDial" => Ok(VoiceDial),
            "TV" => Ok(TV),
            "TV3DMode" => Ok(TV3DMode),
            "TVAntennaCable" => Ok(TVAntennaCable),
            "TVAudioDescription" => Ok(TVAudioDescription),
            "TVAudioDescriptionMixDown" => Ok(TVAudioDescriptionMixDown),
            "TVAudioDescriptionMixUp" => Ok(TVAudioDescriptionMixUp),
            "TVContentsMenu" => Ok(TVContentsMenu),
            "TVDataService" => Ok(TVDataService),
            "TVInput" => Ok(TVInput),
            "TVInputComponent1" => Ok(TVInputComponent1),
            "TVInputComponent2" => Ok(TVInputComponent2),
            "TVInputComposite1" => Ok(TVInputComposite1),
            "TVInputComposite2" => Ok(TVInputComposite2),
            "TVInputHDMI1" => Ok(TVInputHDMI1),
            "TVInputHDMI2" => Ok(TVInputHDMI2),
            "TVInputHDMI3" => Ok(TVInputHDMI3),
            "TVInputHDMI4" => Ok(TVInputHDMI4),
            "TVInputVGA1" => Ok(TVInputVGA1),
            "TVMediaContext" => Ok(TVMediaContext),
            "TVNetwork" => Ok(TVNetwork),
            "TVNumberEntry" => Ok(TVNumberEntry),
            "TVPower" => Ok(TVPower),
            "TVRadioService" => Ok(TVRadioService),
            "TVSatellite" => Ok(TVSatellite),
            "TVSatelliteBS" => Ok(TVSatelliteBS),
            "TVSatelliteCS" => Ok(TVSatelliteCS),
            "TVSatelliteToggle" => Ok(TVSatelliteToggle),
            "TVTerrestrialAnalog" => Ok(TVTerrestrialAnalog),
            "TVTerrestrialDigital" => Ok(TVTerrestrialDigital),
            "TVTimer" => Ok(TVTimer),
            "AVRInput" => Ok(AVRInput),
            "AVRPower" => Ok(AVRPower),
            "ColorF0Red" => Ok(ColorF0Red),
            "ColorF1Green" => Ok(ColorF1Green),
            "ColorF2Yellow" => Ok(ColorF2Yellow),
            "ColorF3Blue" => Ok(ColorF3Blue),
            "ColorF4Grey" => Ok(ColorF4Grey),
            "ColorF5Brown" => Ok(ColorF5Brown),
            "ClosedCaptionToggle" => Ok(ClosedCaptionToggle),
            "Dimmer" => Ok(Dimmer),
            "DisplaySwap" => Ok(DisplaySwap),
            "DVR" => Ok(DVR),
            "Exit" => Ok(Exit),
            "FavoriteClear0" => Ok(FavoriteClear0),
            "FavoriteClear1" => Ok(FavoriteClear1),
            "FavoriteClear2" => Ok(FavoriteClear2),
            "FavoriteClear3" => Ok(FavoriteClear3),
            "FavoriteRecall0" => Ok(FavoriteRecall0),
            "FavoriteRecall1" => Ok(FavoriteRecall1),
            "FavoriteRecall2" => Ok(FavoriteRecall2),
            "FavoriteRecall3" => Ok(FavoriteRecall3),
            "FavoriteStore0" => Ok(FavoriteStore0),
            "FavoriteStore1" => Ok(FavoriteStore1),
            "FavoriteStore2" => Ok(FavoriteStore2),
            "FavoriteStore3" => Ok(FavoriteStore3),
            "Guide" => Ok(Guide),
            "GuideNextDay" => Ok(GuideNextDay),
            "GuidePreviousDay" => Ok(GuidePreviousDay),
            "Info" => Ok(Info),
            "InstantReplay" => Ok(InstantReplay),
            "Link" => Ok(Link),
            "ListProgram" => Ok(ListProgram),
            "LiveContent" => Ok(LiveContent),
            "Lock" => Ok(Lock),
            "MediaApps" => Ok(MediaApps),
            "MediaAudioTrack" => Ok(MediaAudioTrack),
            "MediaLast" => Ok(MediaLast),
            "MediaSkipBackward" => Ok(MediaSkipBackward),
            "MediaSkipForward" => Ok(MediaSkipForward),
            "MediaStepBackward" => Ok(MediaStepBackward),
            "MediaStepForward" => Ok(MediaStepForward),
            "MediaTopMenu" => Ok(MediaTopMenu),
            "NavigateIn" => Ok(NavigateIn),
            "NavigateNext" => Ok(NavigateNext),
            "NavigateOut" => Ok(NavigateOut),
            "NavigatePrevious" => Ok(NavigatePrevious),
            "NextFavoriteChannel" => Ok(NextFavoriteChannel),
            "NextUserProfile" => Ok(NextUserProfile),
            "OnDemand" => Ok(OnDemand),
            "Pairing" => Ok(Pairing),
            "PinPDown" => Ok(PinPDown),
            "PinPMove" => Ok(PinPMove),
            "PinPToggle" => Ok(PinPToggle),
            "PinPUp" => Ok(PinPUp),
            "PlaySpeedDown" => Ok(PlaySpeedDown),
            "PlaySpeedReset" => Ok(PlaySpeedReset),
            "PlaySpeedUp" => Ok(PlaySpeedUp),
            "RandomToggle" => Ok(RandomToggle),
            "RcLowBattery" => Ok(RcLowBattery),
            "RecordSpeedNext" => Ok(RecordSpeedNext),
            "RfBypass" => Ok(RfBypass),
            "ScanChannelsToggle" => Ok(ScanChannelsToggle),
            "ScreenModeNext" => Ok(ScreenModeNext),
            "Settings" => Ok(Settings),
            "SplitScreenToggle" => Ok(SplitScreenToggle),
            "STBInput" => Ok(STBInput),
            "STBPower" => Ok(STBPower),
            "Subtitle" => Ok(Subtitle),
            "Teletext" => Ok(Teletext),
            "VideoModeNext" => Ok(VideoModeNext),
            "Wink" => Ok(Wink),
            "ZoomToggle" => Ok(ZoomToggle),

            _ => Err(UnrecognizedKeyError),
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Key::Character(string) = self {
            serializer.serialize_newtype_variant("Key", 0, "Character", string)
        } else {
            let index = self.discriminant();
            let variant = Self::VARIANTS[index as usize];
            serializer.serialize_unit_variant("Key", index as u32, variant)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Key {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use Key::*;
        enum KeyVariant {
            Character,
            Other(Key),
        }
        impl KeyVariant {
            fn from_discriminant(discriminant: u64) -> Option<Self> {
                match discriminant {
                    0 => Some(KeyVariant::Character),
    
                    1 => Some(KeyVariant::Other(Unidentified)),
                    2 => Some(KeyVariant::Other(Alt)),
                    3 => Some(KeyVariant::Other(AltGraph)),
                    4 => Some(KeyVariant::Other(CapsLock)),
                    5 => Some(KeyVariant::Other(Control)),
                    6 => Some(KeyVariant::Other(Fn)),
                    7 => Some(KeyVariant::Other(FnLock)),
                    8 => Some(KeyVariant::Other(Meta)),
                    9 => Some(KeyVariant::Other(NumLock)),
                    10 => Some(KeyVariant::Other(ScrollLock)),
                    11 => Some(KeyVariant::Other(Shift)),
                    12 => Some(KeyVariant::Other(Symbol)),
                    13 => Some(KeyVariant::Other(SymbolLock)),
                    14 => Some(KeyVariant::Other(Hyper)),
                    15 => Some(KeyVariant::Other(Super)),
                    16 => Some(KeyVariant::Other(Enter)),
                    17 => Some(KeyVariant::Other(Tab)),
                    18 => Some(KeyVariant::Other(ArrowDown)),
                    19 => Some(KeyVariant::Other(ArrowLeft)),
                    20 => Some(KeyVariant::Other(ArrowRight)),
                    21 => Some(KeyVariant::Other(ArrowUp)),
                    22 => Some(KeyVariant::Other(End)),
                    23 => Some(KeyVariant::Other(Home)),
                    24 => Some(KeyVariant::Other(PageDown)),
                    25 => Some(KeyVariant::Other(PageUp)),
                    26 => Some(KeyVariant::Other(Backspace)),
                    27 => Some(KeyVariant::Other(Clear)),
                    28 => Some(KeyVariant::Other(Copy)),
                    29 => Some(KeyVariant::Other(CrSel)),
                    30 => Some(KeyVariant::Other(Cut)),
                    31 => Some(KeyVariant::Other(Delete)),
                    32 => Some(KeyVariant::Other(EraseEof)),
                    33 => Some(KeyVariant::Other(ExSel)),
                    34 => Some(KeyVariant::Other(Insert)),
                    35 => Some(KeyVariant::Other(Paste)),
                    36 => Some(KeyVariant::Other(Redo)),
                    37 => Some(KeyVariant::Other(Undo)),
                    38 => Some(KeyVariant::Other(Accept)),
                    39 => Some(KeyVariant::Other(Again)),
                    40 => Some(KeyVariant::Other(Attn)),
                    41 => Some(KeyVariant::Other(Cancel)),
                    42 => Some(KeyVariant::Other(ContextMenu)),
                    43 => Some(KeyVariant::Other(Escape)),
                    44 => Some(KeyVariant::Other(Execute)),
                    45 => Some(KeyVariant::Other(Find)),
                    46 => Some(KeyVariant::Other(Help)),
                    47 => Some(KeyVariant::Other(Pause)),
                    48 => Some(KeyVariant::Other(Play)),
                    49 => Some(KeyVariant::Other(Props)),
                    50 => Some(KeyVariant::Other(Select)),
                    51 => Some(KeyVariant::Other(ZoomIn)),
                    52 => Some(KeyVariant::Other(ZoomOut)),
                    53 => Some(KeyVariant::Other(BrightnessDown)),
                    54 => Some(KeyVariant::Other(BrightnessUp)),
                    55 => Some(KeyVariant::Other(Eject)),
                    56 => Some(KeyVariant::Other(LogOff)),
                    57 => Some(KeyVariant::Other(Power)),
                    58 => Some(KeyVariant::Other(PowerOff)),
                    59 => Some(KeyVariant::Other(PrintScreen)),
                    60 => Some(KeyVariant::Other(Hibernate)),
                    61 => Some(KeyVariant::Other(Standby)),
                    62 => Some(KeyVariant::Other(WakeUp)),
                    63 => Some(KeyVariant::Other(AllCandidates)),
                    64 => Some(KeyVariant::Other(Alphanumeric)),
                    65 => Some(KeyVariant::Other(CodeInput)),
                    66 => Some(KeyVariant::Other(Compose)),
                    67 => Some(KeyVariant::Other(Convert)),
                    68 => Some(KeyVariant::Other(Dead)),
                    69 => Some(KeyVariant::Other(FinalMode)),
                    70 => Some(KeyVariant::Other(GroupFirst)),
                    71 => Some(KeyVariant::Other(GroupLast)),
                    72 => Some(KeyVariant::Other(GroupNext)),
                    73 => Some(KeyVariant::Other(GroupPrevious)),
                    74 => Some(KeyVariant::Other(ModeChange)),
                    75 => Some(KeyVariant::Other(NextCandidate)),
                    76 => Some(KeyVariant::Other(NonConvert)),
                    77 => Some(KeyVariant::Other(PreviousCandidate)),
                    78 => Some(KeyVariant::Other(Process)),
                    79 => Some(KeyVariant::Other(SingleCandidate)),
                    80 => Some(KeyVariant::Other(HangulMode)),
                    81 => Some(KeyVariant::Other(HanjaMode)),
                    82 => Some(KeyVariant::Other(JunjaMode)),
                    83 => Some(KeyVariant::Other(Eisu)),
                    84 => Some(KeyVariant::Other(Hankaku)),
                    85 => Some(KeyVariant::Other(Hiragana)),
                    86 => Some(KeyVariant::Other(HiraganaKatakana)),
                    87 => Some(KeyVariant::Other(KanaMode)),
                    88 => Some(KeyVariant::Other(KanjiMode)),
                    89 => Some(KeyVariant::Other(Katakana)),
                    90 => Some(KeyVariant::Other(Romaji)),
                    91 => Some(KeyVariant::Other(Zenkaku)),
                    92 => Some(KeyVariant::Other(ZenkakuHankaku)),
                    93 => Some(KeyVariant::Other(F1)),
                    94 => Some(KeyVariant::Other(F2)),
                    95 => Some(KeyVariant::Other(F3)),
                    96 => Some(KeyVariant::Other(F4)),
                    97 => Some(KeyVariant::Other(F5)),
                    98 => Some(KeyVariant::Other(F6)),
                    99 => Some(KeyVariant::Other(F7)),
                    100 => Some(KeyVariant::Other(F8)),
                    101 => Some(KeyVariant::Other(F9)),
                    102 => Some(KeyVariant::Other(F10)),
                    103 => Some(KeyVariant::Other(F11)),
                    104 => Some(KeyVariant::Other(F12)),
                    105 => Some(KeyVariant::Other(Soft1)),
                    106 => Some(KeyVariant::Other(Soft2)),
                    107 => Some(KeyVariant::Other(Soft3)),
                    108 => Some(KeyVariant::Other(Soft4)),
                    109 => Some(KeyVariant::Other(ChannelDown)),
                    110 => Some(KeyVariant::Other(ChannelUp)),
                    111 => Some(KeyVariant::Other(Close)),
                    112 => Some(KeyVariant::Other(MailForward)),
                    113 => Some(KeyVariant::Other(MailReply)),
                    114 => Some(KeyVariant::Other(MailSend)),
                    115 => Some(KeyVariant::Other(MediaClose)),
                    116 => Some(KeyVariant::Other(MediaFastForward)),
                    117 => Some(KeyVariant::Other(MediaPause)),
                    118 => Some(KeyVariant::Other(MediaPlay)),
                    119 => Some(KeyVariant::Other(MediaPlayPause)),
                    120 => Some(KeyVariant::Other(MediaRecord)),
                    121 => Some(KeyVariant::Other(MediaRewind)),
                    122 => Some(KeyVariant::Other(MediaStop)),
                    123 => Some(KeyVariant::Other(MediaTrackNext)),
                    124 => Some(KeyVariant::Other(MediaTrackPrevious)),
                    125 => Some(KeyVariant::Other(New)),
                    126 => Some(KeyVariant::Other(Open)),
                    127 => Some(KeyVariant::Other(Print)),
                    128 => Some(KeyVariant::Other(Save)),
                    129 => Some(KeyVariant::Other(SpellCheck)),
                    130 => Some(KeyVariant::Other(Key11)),
                    131 => Some(KeyVariant::Other(Key12)),
                    132 => Some(KeyVariant::Other(AudioBalanceLeft)),
                    133 => Some(KeyVariant::Other(AudioBalanceRight)),
                    134 => Some(KeyVariant::Other(AudioBassBoostDown)),
                    135 => Some(KeyVariant::Other(AudioBassBoostToggle)),
                    136 => Some(KeyVariant::Other(AudioBassBoostUp)),
                    137 => Some(KeyVariant::Other(AudioFaderFront)),
                    138 => Some(KeyVariant::Other(AudioFaderRear)),
                    139 => Some(KeyVariant::Other(AudioSurroundModeNext)),
                    140 => Some(KeyVariant::Other(AudioTrebleDown)),
                    141 => Some(KeyVariant::Other(AudioTrebleUp)),
                    142 => Some(KeyVariant::Other(AudioVolumeDown)),
                    143 => Some(KeyVariant::Other(AudioVolumeUp)),
                    144 => Some(KeyVariant::Other(AudioVolumeMute)),
                    145 => Some(KeyVariant::Other(MicrophoneToggle)),
                    146 => Some(KeyVariant::Other(MicrophoneVolumeDown)),
                    147 => Some(KeyVariant::Other(MicrophoneVolumeUp)),
                    148 => Some(KeyVariant::Other(MicrophoneVolumeMute)),
                    149 => Some(KeyVariant::Other(SpeechCorrectionList)),
                    150 => Some(KeyVariant::Other(SpeechInputToggle)),
                    151 => Some(KeyVariant::Other(LaunchApplication1)),
                    152 => Some(KeyVariant::Other(LaunchApplication2)),
                    153 => Some(KeyVariant::Other(LaunchCalendar)),
                    154 => Some(KeyVariant::Other(LaunchContacts)),
                    155 => Some(KeyVariant::Other(LaunchMail)),
                    156 => Some(KeyVariant::Other(LaunchMediaPlayer)),
                    157 => Some(KeyVariant::Other(LaunchMusicPlayer)),
                    158 => Some(KeyVariant::Other(LaunchPhone)),
                    159 => Some(KeyVariant::Other(LaunchScreenSaver)),
                    160 => Some(KeyVariant::Other(LaunchSpreadsheet)),
                    161 => Some(KeyVariant::Other(LaunchWebBrowser)),
                    162 => Some(KeyVariant::Other(LaunchWebCam)),
                    163 => Some(KeyVariant::Other(LaunchWordProcessor)),
                    164 => Some(KeyVariant::Other(BrowserBack)),
                    165 => Some(KeyVariant::Other(BrowserFavorites)),
                    166 => Some(KeyVariant::Other(BrowserForward)),
                    167 => Some(KeyVariant::Other(BrowserHome)),
                    168 => Some(KeyVariant::Other(BrowserRefresh)),
                    169 => Some(KeyVariant::Other(BrowserSearch)),
                    170 => Some(KeyVariant::Other(BrowserStop)),
                    171 => Some(KeyVariant::Other(AppSwitch)),
                    172 => Some(KeyVariant::Other(Call)),
                    173 => Some(KeyVariant::Other(Camera)),
                    174 => Some(KeyVariant::Other(CameraFocus)),
                    175 => Some(KeyVariant::Other(EndCall)),
                    176 => Some(KeyVariant::Other(GoBack)),
                    177 => Some(KeyVariant::Other(GoHome)),
                    178 => Some(KeyVariant::Other(HeadsetHook)),
                    179 => Some(KeyVariant::Other(LastNumberRedial)),
                    180 => Some(KeyVariant::Other(Notification)),
                    181 => Some(KeyVariant::Other(MannerMode)),
                    182 => Some(KeyVariant::Other(VoiceDial)),
                    183 => Some(KeyVariant::Other(TV)),
                    184 => Some(KeyVariant::Other(TV3DMode)),
                    185 => Some(KeyVariant::Other(TVAntennaCable)),
                    186 => Some(KeyVariant::Other(TVAudioDescription)),
                    187 => Some(KeyVariant::Other(TVAudioDescriptionMixDown)),
                    188 => Some(KeyVariant::Other(TVAudioDescriptionMixUp)),
                    189 => Some(KeyVariant::Other(TVContentsMenu)),
                    190 => Some(KeyVariant::Other(TVDataService)),
                    191 => Some(KeyVariant::Other(TVInput)),
                    192 => Some(KeyVariant::Other(TVInputComponent1)),
                    193 => Some(KeyVariant::Other(TVInputComponent2)),
                    194 => Some(KeyVariant::Other(TVInputComposite1)),
                    195 => Some(KeyVariant::Other(TVInputComposite2)),
                    196 => Some(KeyVariant::Other(TVInputHDMI1)),
                    197 => Some(KeyVariant::Other(TVInputHDMI2)),
                    198 => Some(KeyVariant::Other(TVInputHDMI3)),
                    199 => Some(KeyVariant::Other(TVInputHDMI4)),
                    200 => Some(KeyVariant::Other(TVInputVGA1)),
                    201 => Some(KeyVariant::Other(TVMediaContext)),
                    202 => Some(KeyVariant::Other(TVNetwork)),
                    203 => Some(KeyVariant::Other(TVNumberEntry)),
                    204 => Some(KeyVariant::Other(TVPower)),
                    205 => Some(KeyVariant::Other(TVRadioService)),
                    206 => Some(KeyVariant::Other(TVSatellite)),
                    207 => Some(KeyVariant::Other(TVSatelliteBS)),
                    208 => Some(KeyVariant::Other(TVSatelliteCS)),
                    209 => Some(KeyVariant::Other(TVSatelliteToggle)),
                    210 => Some(KeyVariant::Other(TVTerrestrialAnalog)),
                    211 => Some(KeyVariant::Other(TVTerrestrialDigital)),
                    212 => Some(KeyVariant::Other(TVTimer)),
                    213 => Some(KeyVariant::Other(AVRInput)),
                    214 => Some(KeyVariant::Other(AVRPower)),
                    215 => Some(KeyVariant::Other(ColorF0Red)),
                    216 => Some(KeyVariant::Other(ColorF1Green)),
                    217 => Some(KeyVariant::Other(ColorF2Yellow)),
                    218 => Some(KeyVariant::Other(ColorF3Blue)),
                    219 => Some(KeyVariant::Other(ColorF4Grey)),
                    220 => Some(KeyVariant::Other(ColorF5Brown)),
                    221 => Some(KeyVariant::Other(ClosedCaptionToggle)),
                    222 => Some(KeyVariant::Other(Dimmer)),
                    223 => Some(KeyVariant::Other(DisplaySwap)),
                    224 => Some(KeyVariant::Other(DVR)),
                    225 => Some(KeyVariant::Other(Exit)),
                    226 => Some(KeyVariant::Other(FavoriteClear0)),
                    227 => Some(KeyVariant::Other(FavoriteClear1)),
                    228 => Some(KeyVariant::Other(FavoriteClear2)),
                    229 => Some(KeyVariant::Other(FavoriteClear3)),
                    230 => Some(KeyVariant::Other(FavoriteRecall0)),
                    231 => Some(KeyVariant::Other(FavoriteRecall1)),
                    232 => Some(KeyVariant::Other(FavoriteRecall2)),
                    233 => Some(KeyVariant::Other(FavoriteRecall3)),
                    234 => Some(KeyVariant::Other(FavoriteStore0)),
                    235 => Some(KeyVariant::Other(FavoriteStore1)),
                    236 => Some(KeyVariant::Other(FavoriteStore2)),
                    237 => Some(KeyVariant::Other(FavoriteStore3)),
                    238 => Some(KeyVariant::Other(Guide)),
                    239 => Some(KeyVariant::Other(GuideNextDay)),
                    240 => Some(KeyVariant::Other(GuidePreviousDay)),
                    241 => Some(KeyVariant::Other(Info)),
                    242 => Some(KeyVariant::Other(InstantReplay)),
                    243 => Some(KeyVariant::Other(Link)),
                    244 => Some(KeyVariant::Other(ListProgram)),
                    245 => Some(KeyVariant::Other(LiveContent)),
                    246 => Some(KeyVariant::Other(Lock)),
                    247 => Some(KeyVariant::Other(MediaApps)),
                    248 => Some(KeyVariant::Other(MediaAudioTrack)),
                    249 => Some(KeyVariant::Other(MediaLast)),
                    250 => Some(KeyVariant::Other(MediaSkipBackward)),
                    251 => Some(KeyVariant::Other(MediaSkipForward)),
                    252 => Some(KeyVariant::Other(MediaStepBackward)),
                    253 => Some(KeyVariant::Other(MediaStepForward)),
                    254 => Some(KeyVariant::Other(MediaTopMenu)),
                    255 => Some(KeyVariant::Other(NavigateIn)),
                    256 => Some(KeyVariant::Other(NavigateNext)),
                    257 => Some(KeyVariant::Other(NavigateOut)),
                    258 => Some(KeyVariant::Other(NavigatePrevious)),
                    259 => Some(KeyVariant::Other(NextFavoriteChannel)),
                    260 => Some(KeyVariant::Other(NextUserProfile)),
                    261 => Some(KeyVariant::Other(OnDemand)),
                    262 => Some(KeyVariant::Other(Pairing)),
                    263 => Some(KeyVariant::Other(PinPDown)),
                    264 => Some(KeyVariant::Other(PinPMove)),
                    265 => Some(KeyVariant::Other(PinPToggle)),
                    266 => Some(KeyVariant::Other(PinPUp)),
                    267 => Some(KeyVariant::Other(PlaySpeedDown)),
                    268 => Some(KeyVariant::Other(PlaySpeedReset)),
                    269 => Some(KeyVariant::Other(PlaySpeedUp)),
                    270 => Some(KeyVariant::Other(RandomToggle)),
                    271 => Some(KeyVariant::Other(RcLowBattery)),
                    272 => Some(KeyVariant::Other(RecordSpeedNext)),
                    273 => Some(KeyVariant::Other(RfBypass)),
                    274 => Some(KeyVariant::Other(ScanChannelsToggle)),
                    275 => Some(KeyVariant::Other(ScreenModeNext)),
                    276 => Some(KeyVariant::Other(Settings)),
                    277 => Some(KeyVariant::Other(SplitScreenToggle)),
                    278 => Some(KeyVariant::Other(STBInput)),
                    279 => Some(KeyVariant::Other(STBPower)),
                    280 => Some(KeyVariant::Other(Subtitle)),
                    281 => Some(KeyVariant::Other(Teletext)),
                    282 => Some(KeyVariant::Other(VideoModeNext)),
                    283 => Some(KeyVariant::Other(Wink)),
                    284 => Some(KeyVariant::Other(ZoomToggle)),

                    _ => None,
                }
            }
            fn from_str(variant_name: &str) -> Option<Self> {
                if variant_name == "Character" {
                    Some(KeyVariant::Character)
                } else {
                    Key::from_str(variant_name).ok().map(KeyVariant::Other)
                }
            }
        }
        struct KeyVariantVisitor;
        impl<'de> serde::de::Visitor<'de> for KeyVariantVisitor {
            type Value = KeyVariant;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum Key")
            }
            fn visit_u64<E>(self, discriminant: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                KeyVariant::from_discriminant(discriminant).ok_or_else(|| {
                    serde::de::Error::invalid_value(
                        serde::de::Unexpected::Unsigned(discriminant),
                        &"variant index 0 <= i < 285",
                    )
                })
            }
            fn visit_str<E>(self, variant_name: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                KeyVariant::from_str(variant_name).ok_or_else(|| {
                    serde::de::Error::unknown_variant(variant_name, Key::VARIANTS)
                })
            }
        }
        impl<'de> serde::Deserialize<'de> for KeyVariant {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_identifier(KeyVariantVisitor)
            }
        }
        struct KeyVisitor;
        impl<'de> serde::de::Visitor<'de> for KeyVisitor {
            type Value = Key;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum Key")
            }
            fn visit_enum<E>(self, data: E) -> Result<Self::Value, E::Error>
            where
                E: serde::de::EnumAccess<'de>,
            {
                use serde::de::VariantAccess;
                let (variant, visitor) = data.variant()?;
                match variant {
                    KeyVariant::Character => {
                        visitor.newtype_variant().map(Key::Character)
                    }
                    KeyVariant::Other(key) => {
                        visitor.unit_variant()?;
                        Ok(key)
                    }
                }
            }
        }
        deserializer.deserialize_enum("Key", Key::VARIANTS, KeyVisitor)
    }
}

/// Parse from string error, returned when string does not match to any Key variant.
#[derive(Clone, Debug)]
pub struct UnrecognizedKeyError;

impl fmt::Display for UnrecognizedKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognized key")
    }
}

impl Error for UnrecognizedKeyError {}

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
    
