//! URLs of the per-ayah MP3 files on everyayah.com.
//!
//! Every ayah is its own file, named with the surah and ayah numbers padded
//! to 3 digits: surah 2, ayah 255 is `002255.mp3`.
//! Example: <https://everyayah.com/data/Alafasy_128kbps/002255.mp3>

use quran_core::VerseKey;

/// Base address of the everyayah audio files.
pub const EVERYAYAH_BASE: &str = "https://everyayah.com/data";

/// Folder of Mishary Rashid Alafasy's recitation at 128 kbps, our default reciter.
pub const DEFAULT_RECITER: &str = "Alafasy_128kbps";

// LEARN(#3): build the URL of one ayah's MP3 file.
//
// `everyayah_url("Alafasy_128kbps", <key for 2:255>)` must return
// "https://everyayah.com/data/Alafasy_128kbps/002255.mp3".
//
// Hints:
//   - `format!` works like `println!` but returns a `String`
//   - padding a number with zeros: https://doc.rust-lang.org/std/fmt/#width
//   - reuse `EVERYAYAH_BASE` instead of typing the address again
//
// When done: delete the `#[allow(unused_variables)]` line and the `#[ignore]`
// lines below, then run `cargo test -p quran-data`.

/// URL of the MP3 for one ayah, recited by the reciter in `reciter_folder`.
#[allow(unused_variables)]
pub fn everyayah_url(reciter_folder: &str, key: VerseKey) -> String {
    todo!("LEARN(#3)")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(surah: u8, ayah: u16) -> VerseKey {
        VerseKey::new(surah, ayah).unwrap()
    }

    #[test]
    #[ignore = "LEARN(#3): delete this line when you start the exercise"]
    fn ayat_al_kursi() {
        assert_eq!(
            everyayah_url(DEFAULT_RECITER, key(2, 255)),
            "https://everyayah.com/data/Alafasy_128kbps/002255.mp3"
        );
    }

    #[test]
    #[ignore = "LEARN(#3): delete this line when you start the exercise"]
    fn pads_small_numbers() {
        assert_eq!(
            everyayah_url(DEFAULT_RECITER, key(1, 1)),
            "https://everyayah.com/data/Alafasy_128kbps/001001.mp3"
        );
        assert_eq!(
            everyayah_url(DEFAULT_RECITER, key(36, 12)),
            "https://everyayah.com/data/Alafasy_128kbps/036012.mp3"
        );
    }

    #[test]
    #[ignore = "LEARN(#3): delete this line when you start the exercise"]
    fn other_reciter() {
        assert_eq!(
            everyayah_url("Abdul_Basit_Murattal_192kbps", key(114, 6)),
            "https://everyayah.com/data/Abdul_Basit_Murattal_192kbps/114006.mp3"
        );
    }
}
