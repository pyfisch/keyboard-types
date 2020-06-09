import re
import requests

def handle_enum_entries(text, file):
    display = []
    for match in re.findall(r"id=\".*?\">\"(.*?)\"</code>\n.*<td>(((.*?)\n)+?)\s+(<tr>|</table>)", text):
        doc = re.sub(r"[ \t][ \t]+", "\n", match[1])
        doc = re.sub(r"<a .*?>(.*?)</a>", "\\1", doc)
        for line in doc.split('\n'):
            line = line.strip()
            if not line:
                continue
            print("    /// {}".format(line), file=file)
        display.append(match[0])
        print("    {},".format(match[0]), file=file)
    return display

def print_variant_name_entries(display, file):
    for key in display:
        print("        \"{0}\",".format(key), file=file)

def print_discriminant_entries(display, file):
    for i, key in enumerate(display, 1):
        print("            {0} => {1},".format(key, i), file=file)

def print_display_entries(display, file):
    for key in display:
        print("            {0} => f.write_str(\"{0}\"),".format(key), file=file)

def print_from_str_entries(display, file):
    for key in display:
        print("            \"{0}\" => Ok({0}),".format(key), file=file)

def print_deserialize_variant_entries(display, file):
    for i, key in enumerate(display, 1):
        print("                    {0} => Some(KeyVariant::Other({1})),".format(i, key), file=file)

def convert_key(text, file):
    print("""
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
    """, file=file)
    display = handle_enum_entries(text, file)
    print("""
    #[doc(hidden)]
    __Nonexhaustive,
}

impl Key {
    const VARIANTS: &'static [&'static str] = &[
        "Character",
    """, file=file)
    print_variant_name_entries(display, file)
    print("""
    ];
    fn discriminant(&self) -> u16 {
        use Key::*;
        match self {
            Character(_) => 0,
    """, file=file)
    print_discriminant_entries(display, file)
    print("""
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
            s if is_key_string(s) => Ok(Character(s.to_string())),""", file=file)
    print_from_str_entries(display, file)
    print("""
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
    """, file=file)
    print_deserialize_variant_entries(display, file)
    print("""
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
                        &"variant index 0 <= i < """, file=file, end="")
    print(1 + len(display), file=file, end="")
    print("""\",
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
        assert!(!is_key_string("\t"));
    }
}
    """, file=file)

def convert_code(text, file):
    print("""
// AUTO GENERATED CODE - DO NOT EDIT

use std::fmt::{self, Display};

/// Code is the physical position of a key.
///
/// The names are based on the US keyboard. If the key
/// is not present on US keyboards a name from another
/// layout is used.
///
/// Specification:
/// <https://w3c.github.io/uievents-code/>
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Code {""", file=file)
    display = handle_enum_entries(text, file)
    print("""
    #[doc(hidden)]
    __Nonexhaustive,
}
    """, file=file)

    print("""

impl Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Code::*;
        match *self {
    """, file=file)
    print_display_entries(display, file)
    print("""
            __Nonexhaustive => unreachable!(),
        }
    }
} 

    """, file=file)

if __name__ == '__main__':
    input = requests.get('https://w3c.github.io/uievents-key/').text
    with open('src/key.rs', 'w') as output:
        convert_key(input, output)
    input = requests.get('https://w3c.github.io/uievents-code/').text
    with open('src/code.rs', 'w') as output:
        convert_code(input, output)
