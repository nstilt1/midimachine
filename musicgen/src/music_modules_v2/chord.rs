use std::hash::Hash;

use super::chord_type::*;

#[derive(Clone, Debug)]
pub struct Chord {
    pub chord_type: ChordType,
    pub root: u8,
    _chords_to_not_play_next: Vec<Chord>,
}

impl Default for Chord {
    fn default() -> Self {
        Self {
            chord_type: ChordType::default(),
            root: 0,
            _chords_to_not_play_next: Vec::new()
        }
    }
}

impl PartialEq for Chord {
    fn eq(&self, other: &Self) -> bool {
        self.root.eq(&other.root) && self.chord_type.eq(&other.chord_type)
    }
}

impl Eq for Chord {}

impl Hash for Chord {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.chord_type.hash(state);
        self.root.hash(state);
    }
}

impl Chord {
    pub fn new(root_index: u8, chord_type: &ChordType) -> Self {
        Chord {
            chord_type: chord_type.to_owned(),
            root: root_index,
            _chords_to_not_play_next: Vec::new()
        }
    }

    /**
     * Gets an array of pitches of the notes for this chord
     * returns note_intervals.map(|n| n + self.root)
     */
    pub fn get_notes(&self) -> Vec<i16> {
        let mut result: Vec<i16> = Vec::new();
        for n in self.chord_type.to_owned().note_intervals {
            result.push((n + self.root) as i16);
        }
        return result
    }

    /**
     * Gets an array of optional notes for this chord.
     */
    pub fn get_optional_notes(&self) -> Vec<i16> {
        let mut result: Vec<i16> = Vec::new();
        for n in self.chord_type.optional_notes.iter() {
            result.push((n + self.root) as i16);
        }
        return result
    }
}