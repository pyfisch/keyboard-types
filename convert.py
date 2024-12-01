import re
import requests
from bs4 import BeautifulSoup


def parse(text):
    # The document is not valid XML, so we need a more lenient parser than 'html.parser'
    soup = BeautifulSoup(text, 'lxml')
    display = []
    for table in soup.find_all('table'):
        section_name = table.find_previous_sibling('h3').find(class_='content').text

        # Skip duplicate table
        if table['id'] == 'key-table-media-controller-dup':
            continue

        for row in table.find('tbody').find_all('tr'):
            [name, _required, typical_usage] = row.find_all('td')

            name = name.text.strip().strip('"')

            # Skip F keys here
            if re.match(r'^F\d+$', name):
                continue

            # Strip <a> tags
            for a in typical_usage.find_all('a'):
                a.replace_with(a.text)

            # Use the semantic `<kbd>` element instead.
            for keycap in typical_usage.find_all(class_='keycap'):
                kbd = soup.new_tag("kbd")
                kbd.string = keycap.text
                keycap.replace_with(kbd)

            # Link to the relevant type.
            for code in typical_usage.find_all(class_='code'):
                text = code.text.strip().strip('"')
                code.replace_with(f"[`{text}`][Code::{text}]")
            for code in typical_usage.find_all(class_='key'):
                text = code.text.strip().strip('"')
                code.replace_with(f"[`{text}`][Key::{text}]")

            comment = re.sub(r"[ \t][ \t]+", "\n", typical_usage.decode_contents())

            display.append([name, comment, []])
    return display


def emit_enum_entries(display, file):
    for [key, doc_comment, alternatives] in display:
        for line in doc_comment.split('\n'):
            line = line.strip()
            if len(line) == 0:
                continue
            print(f"    /// {line}", file=file)
        print(f"    {key},", file=file)


def print_display_entries(display, file):
    for [key, doc_comment, alternatives] in display:
        print("            {0} => f.write_str(\"{0}\"),".format(
            key), file=file)


def print_from_str_entries(display, file):
    for [key, doc_comment, alternatives] in display:
        print("            \"{0}\"".format(key), file=file, end='')
        for alternative in alternatives:
            print(" | \"{0}\"".format(alternative), file=file, end='')
        print(" => Ok({0}),".format(key), file=file)


def add_alternative_for(display, key, alternative):
    for [found_key, doc_comment, alternatives] in display:
        if found_key != key:
            continue
        alternatives.append(alternative)


def convert_key(text, file):
    print("""
// AUTO GENERATED CODE - DO NOT EDIT
#![cfg_attr(rustfmt, rustfmt_skip)]

use std::fmt::{self, Display};
use std::str::FromStr;
use std::error::Error;

/// Key represents the meaning of a keypress.
///
/// Specification:
/// <https://w3c.github.io/uievents-key/>
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum NamedKey {""", file=file)
    display = parse(text)

    for i in range(1, 36):
        display.append([
            'F{}'.format(i),
            'The F{0} key, a general purpose function key, as index {0}.'.format(i),
            []
        ])

    emit_enum_entries(display, file)
    print("}", file=file)

    print("""

impl Display for NamedKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::NamedKey::*;
        match *self {""", file=file)
    print_display_entries(display, file)
    print("""
        }
    }
}

impl FromStr for NamedKey {
    type Err = UnrecognizedNamedKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::NamedKey::*;
        match s {""", file=file)
    print_from_str_entries(display, file)
    print("""
            _ => Err(UnrecognizedNamedKeyError),
        }
    }
}

/// Parse from string error, returned when string does not match to any Key variant.
#[derive(Clone, Debug)]
pub struct UnrecognizedNamedKeyError;

impl fmt::Display for UnrecognizedNamedKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognized key")
    }
}

impl Error for UnrecognizedNamedKeyError {}
""", file=file)


def convert_code(text, file):
    print("""
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
pub enum Code {""", file=file)
    display = parse(text)

    for i in range(1, 36):
        display.append([
            'F{}'.format(i),
            '<kbd>F{}</kbd>'.format(i),
            []
        ])

    chromium_key_codes = [
        'BrightnessDown',
        'BrightnessUp',
        'DisplayToggleIntExt',
        'KeyboardLayoutSelect',
        'LaunchAssistant',
        'LaunchControlPanel',
        'LaunchScreenSaver',
        'MailForward',
        'MailReply',
        'MailSend',
        'MediaFastForward',
        'MediaPause',
        'MediaPlay',
        'MediaRecord',
        'MediaRewind',
        'MicrophoneMuteToggle',
        'PrivacyScreenToggle',
        'SelectTask',
        'ShowAllWindows',
        'ZoomToggle',
    ]

    for chromium_only in chromium_key_codes:
        display.append([
            chromium_only,
            'Non-standard code value supported by Chromium.',
            []
        ])

    add_alternative_for(display, 'MetaLeft', 'OSLeft')
    add_alternative_for(display, 'MetaRight', 'OSRight')
    add_alternative_for(display, 'AudioVolumeDown', 'VolumeDown')
    add_alternative_for(display, 'AudioVolumeMute', 'VolumeMute')
    add_alternative_for(display, 'AudioVolumeUp', 'VolumeUp')
    add_alternative_for(display, 'MediaSelect', 'LaunchMediaPlayer')

    emit_enum_entries(display, file)
    print("}", file=file)

    print("""

impl Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Code::*;
        match *self {""", file=file)
    print_display_entries(display, file)
    print("""
        }
    }
}

impl FromStr for Code {
    type Err = UnrecognizedCodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::Code::*;
        match s {""", file=file)
    print_from_str_entries(display, file)
    print("""
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
""", file=file)


if __name__ == '__main__':
    input = requests.get('https://w3c.github.io/uievents-key/').text
    with open('src/named_key.rs', 'w', encoding='utf-8') as output:
        convert_key(input, output)
    input = requests.get('https://w3c.github.io/uievents-code/').text
    with open('src/code.rs', 'w', encoding='utf-8') as output:
        convert_code(input, output)
