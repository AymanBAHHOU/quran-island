//! The 114 surahs of the Quran.
//!
//! Metadata lives in `data/surahs.json` and is embedded into the binary at
//! compile time with `include_str!`, so the app works offline.

use serde::Deserialize;

/// The surah list as raw JSON text, baked into the binary.
const SURAHS_JSON: &str = include_str!("../data/surahs.json");

/// Number of ayahs in each surah. `VERSE_COUNTS[0]` is Al-Fatihah (7 ayahs).
///
/// A plain array is handy where we only need the counts (see `VerseKey::new`)
/// and don't want to parse JSON. A test below checks it matches `surahs.json`.
pub const VERSE_COUNTS: [u16; 114] = [
    7, 286, 200, 176, 120, 165, 206, 75, 129, 109, 123, 111, 43, 52, 99, 128, 111, 110, 98, 135,
    112, 78, 118, 64, 77, 227, 93, 88, 69, 60, 34, 30, 73, 54, 45, 83, 182, 88, 75, 85, 54, 53, 89,
    59, 37, 35, 38, 29, 18, 45, 60, 49, 62, 55, 78, 96, 29, 22, 24, 13, 14, 11, 11, 18, 12, 12, 30,
    52, 52, 44, 28, 28, 20, 56, 40, 31, 50, 40, 46, 42, 29, 19, 36, 25, 22, 17, 19, 26, 30, 20, 15,
    21, 11, 8, 8, 19, 5, 8, 8, 11, 11, 8, 3, 9, 5, 4, 7, 3, 6, 3, 5, 4, 5, 6,
];

/// One surah (chapter) of the Quran.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Surah {
    /// 1 to 114.
    pub id: u8,
    /// Transliterated name, e.g. "Al-Fatihah".
    pub name_simple: String,
    /// Arabic name, e.g. "الفاتحة".
    pub name_arabic: String,
    /// English meaning, e.g. "The Opener".
    pub translated_name: String,
    /// Number of ayahs (verses).
    pub verses_count: u16,
    pub revelation_place: RevelationPlace,
}

/// Where a surah was revealed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum RevelationPlace {
    Makkah,
    Madinah,
}

/// Every surah, in order (index 0 is Al-Fatihah).
pub fn all() -> Vec<Surah> {
    serde_json::from_str(SURAHS_JSON)
        .expect("bundled surahs.json should be valid and match the Surah struct")
}

/// The surah with this id (1 to 114), or `None` if it doesn't exist.
pub fn find(id: u8) -> Option<Surah> {
    all().into_iter().find(|surah| surah.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Not an exercise: keeps `VERSE_COUNTS` in sync with `surahs.json`.
    #[test]
    fn verse_counts_match_json() {
        let json: serde_json::Value = serde_json::from_str(SURAHS_JSON).unwrap();
        let counts: Vec<u64> = json
            .as_array()
            .unwrap()
            .iter()
            .map(|s| s["verses_count"].as_u64().unwrap())
            .collect();
        let expected: Vec<u64> = VERSE_COUNTS.iter().map(|&c| u64::from(c)).collect();
        assert_eq!(counts, expected);
        assert_eq!(expected.iter().sum::<u64>(), 6236);
    }

    #[test]
    fn there_are_114_surahs() {
        assert_eq!(all().len(), 114);
    }

    #[test]
    fn al_fatihah_is_first() {
        let fatihah = &all()[0];
        assert_eq!(fatihah.id, 1);
        assert_eq!(fatihah.name_simple, "Al-Fatihah");
        assert_eq!(fatihah.name_arabic, "الفاتحة");
        assert_eq!(fatihah.translated_name, "The Opener");
        assert_eq!(fatihah.verses_count, 7);
        assert_eq!(fatihah.revelation_place, RevelationPlace::Makkah);
    }

    #[test]
    fn find_al_baqarah() {
        let baqarah = find(2).expect("surah 2 should exist");
        assert_eq!(baqarah.name_simple, "Al-Baqarah");
        assert_eq!(baqarah.verses_count, 286);
        assert_eq!(baqarah.revelation_place, RevelationPlace::Madinah);
    }

    #[test]
    fn find_out_of_range() {
        assert_eq!(find(0), None);
        assert_eq!(find(115), None);
    }

    #[test]
    fn ids_are_1_to_114_in_order() {
        let ids: Vec<u8> = all().iter().map(|s| s.id).collect();
        let expected: Vec<u8> = (1..=114).collect();
        assert_eq!(ids, expected);
    }
}
