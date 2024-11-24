//! Some test utils.

use std::collections::HashSet;

use crate::music_modules_v2::{chord::Chord, Music};

impl Music {
    /// Creates a Music struct with all chords and with pruning chords.
    #[allow(unused)]
    pub fn smoke_hash_all_pruning_chords(key: &str, scale: &str) -> Self {
        let chord_selection = HashSet::from_iter([
            "minor7",
            "major7",
            "diminished",
            "augmented",
            "major6",
            "minor6",
            "major9",
            "major7sharp9",
            "major7flat5sharp9",
            "major9flat5",
            "major7flat9",
            "major",
            "minor",
            "minor9",
            "major13",
            "dominant9",
            "add9"
        ].iter().map(|str| str.to_string()));
        let mut musician = Music::smoke_hash(Default::default(), key, &chord_selection, "custom_pruning", scale).unwrap();
        musician
    }
}

impl Chord {
    /// Gets a hashset of the notes in this chord. The notes will be between 
    /// 0 and 12.
    pub fn get_notes_set(&self) -> HashSet<i16> {
        self.get_notes().iter().map(|n| *n % 12).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chords_are_present() {
        let m = Music::smoke_hash_all_pruning_chords("Cmin", "pentatonic");
        assert!(m.chord_list.len() > 0);
    }
}