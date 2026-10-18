//! Pure domain logic for Quran Island.
//!
//! Nothing in this crate touches the network, the disk or the audio device,
//! so everything here can be unit-tested quickly with `cargo test -p quran-core`.

pub mod surah;
pub mod verse_key;

pub use surah::{RevelationPlace, Surah};
pub use verse_key::{VerseKey, VerseKeyError};
