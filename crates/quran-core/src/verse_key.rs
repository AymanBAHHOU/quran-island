//! `VerseKey`: a reference to one ayah, written "surah:ayah" (e.g. "2:255").

use std::fmt;
use std::str::FromStr;

use thiserror::Error;

use crate::surah::VERSE_COUNTS;

/// A valid reference to one ayah, e.g. 2:255 (Ayat al-Kursi).
///
/// The fields are private: the only ways to build a `VerseKey` are `new` and
/// `parse`, which both validate. So any `VerseKey` that exists is valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VerseKey {
    surah: u8,
    ayah: u16,
}

/// Why a verse key was rejected.
///
/// `thiserror` writes the `Display` impl for us from the `#[error(...)]` messages.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum VerseKeyError {
    #[error("expected the format \"surah:ayah\", got {0:?}")]
    Format(String),
    #[error("{0:?} is not a valid number")]
    InvalidNumber(String),
    #[error("surah {0} does not exist (valid surahs are 1 to 114)")]
    SurahOutOfRange(u8),
    #[error("surah {surah} has {max} ayahs, so ayah {ayah} does not exist")]
    AyahOutOfRange { surah: u8, ayah: u16, max: u16 },
}

impl VerseKey {
    /// Builds a verse key, checking that the surah and ayah exist.
    ///
    /// This one is written for you, as an example to read before exercise #2.
    pub fn new(surah: u8, ayah: u16) -> Result<Self, VerseKeyError> {
        if !(1..=114).contains(&surah) {
            return Err(VerseKeyError::SurahOutOfRange(surah));
        }
        // `surah` is 1-based, arrays are 0-based. `usize::from` converts u8 -> usize.
        let max = VERSE_COUNTS[usize::from(surah) - 1];
        if !(1..=max).contains(&ayah) {
            return Err(VerseKeyError::AyahOutOfRange { surah, ayah, max });
        }
        Ok(Self { surah, ayah })
    }

    pub fn surah(&self) -> u8 {
        self.surah
    }

    pub fn ayah(&self) -> u16 {
        self.ayah
    }
}

// LEARN(#2): implement `Display` so `VerseKey::new(2, 255)?.to_string() == "2:255"`.
// Hint: the `write!` macro. https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for VerseKey {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("LEARN(#2)")
    }
}

// LEARN(#2): implement `FromStr` so `"2:255".parse::<VerseKey>()` works.
//
// Steps:
//   1. split the text on ':' into two parts (look at `str::split_once`),
//      returning `VerseKeyError::Format` if there's no ':'
//   2. parse each part into a number (`str::parse`), mapping a failure to
//      `VerseKeyError::InvalidNumber` (look at `Result::map_err`)
//   3. let `VerseKey::new` do the range checks
// The `?` operator will keep this short.
//
// When done: delete both `#[allow(unused_variables)]` lines in this file and
// the `#[ignore]` lines below, then run `cargo test -p quran-core`.
impl FromStr for VerseKey {
    type Err = VerseKeyError;

    #[allow(unused_variables)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("LEARN(#2)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // These test `new`, which is already written, so they run right away.

    #[test]
    fn new_accepts_valid_keys() {
        let key = VerseKey::new(2, 255).unwrap();
        assert_eq!(key.surah(), 2);
        assert_eq!(key.ayah(), 255);
        assert!(VerseKey::new(1, 1).is_ok());
        assert!(VerseKey::new(114, 6).is_ok());
    }

    #[test]
    fn new_rejects_invalid_keys() {
        assert_eq!(VerseKey::new(0, 1), Err(VerseKeyError::SurahOutOfRange(0)));
        assert_eq!(
            VerseKey::new(115, 1),
            Err(VerseKeyError::SurahOutOfRange(115))
        );
        assert_eq!(
            VerseKey::new(2, 287),
            Err(VerseKeyError::AyahOutOfRange {
                surah: 2,
                ayah: 287,
                max: 286
            })
        );
        assert_eq!(
            VerseKey::new(1, 0),
            Err(VerseKeyError::AyahOutOfRange {
                surah: 1,
                ayah: 0,
                max: 7
            })
        );
    }

    // Exercise #2 tests.

    #[test]
    #[ignore = "LEARN(#2): delete this line when you start the exercise"]
    fn display() {
        assert_eq!(VerseKey::new(2, 255).unwrap().to_string(), "2:255");
        assert_eq!(VerseKey::new(1, 1).unwrap().to_string(), "1:1");
    }

    #[test]
    #[ignore = "LEARN(#2): delete this line when you start the exercise"]
    fn parse_valid() {
        assert_eq!("2:255".parse(), Ok(VerseKey::new(2, 255).unwrap()));
        assert_eq!("114:6".parse(), Ok(VerseKey::new(114, 6).unwrap()));
    }

    #[test]
    #[ignore = "LEARN(#2): delete this line when you start the exercise"]
    fn parse_wrong_format() {
        assert_eq!(
            "2255".parse::<VerseKey>(),
            Err(VerseKeyError::Format("2255".to_string()))
        );
        assert_eq!(
            "".parse::<VerseKey>(),
            Err(VerseKeyError::Format(String::new()))
        );
    }

    #[test]
    #[ignore = "LEARN(#2): delete this line when you start the exercise"]
    fn parse_invalid_numbers() {
        assert_eq!(
            "x:1".parse::<VerseKey>(),
            Err(VerseKeyError::InvalidNumber("x".to_string()))
        );
        assert_eq!(
            "2:abc".parse::<VerseKey>(),
            Err(VerseKeyError::InvalidNumber("abc".to_string()))
        );
        assert_eq!(
            "-1:5".parse::<VerseKey>(),
            Err(VerseKeyError::InvalidNumber("-1".to_string()))
        );
        // "1:2:3" can fairly be a Format or an InvalidNumber error,
        // depending on how you split. Either is accepted.
        assert!(matches!(
            "1:2:3".parse::<VerseKey>(),
            Err(VerseKeyError::Format(_) | VerseKeyError::InvalidNumber(_))
        ));
    }

    #[test]
    #[ignore = "LEARN(#2): delete this line when you start the exercise"]
    fn parse_out_of_range() {
        assert_eq!(
            "115:1".parse::<VerseKey>(),
            Err(VerseKeyError::SurahOutOfRange(115))
        );
        assert_eq!(
            "2:287".parse::<VerseKey>(),
            Err(VerseKeyError::AyahOutOfRange {
                surah: 2,
                ayah: 287,
                max: 286
            })
        );
    }

    #[test]
    #[ignore = "LEARN(#2): delete this line when you start the exercise"]
    fn display_and_parse_round_trip() {
        for text in ["1:1", "2:255", "36:83", "114:6"] {
            let key: VerseKey = text.parse().unwrap();
            assert_eq!(key.to_string(), text);
        }
    }
}
