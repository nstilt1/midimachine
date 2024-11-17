use std::hash::Hash;

use serde::{ser::SerializeStruct, Serialize};

use crate::music_modules_v2::music::KEYS;

use super::chord_type::*;

#[derive(Clone, Debug)]
pub struct Chord {
    pub chord_type: ChordType,
    pub root: u8,
    pub key: i16,
    _chords_to_not_play_next: Vec<Chord>,
}

impl Default for Chord {
    fn default() -> Self {
        Self {
            chord_type: ChordType::default(),
            root: 0,
            key: 0,
            _chords_to_not_play_next: Vec::new()
        }
    }
}

impl PartialEq for Chord {
    fn eq(&self, other: &Self) -> bool {
        self.root.eq(&other.root) 
            && self.chord_type.note_intervals.eq(&other.chord_type.note_intervals)
            && self.chord_type.optional_notes.eq(&other.chord_type.optional_notes)
    }
}

impl Eq for Chord {}

impl Hash for Chord {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_name(self.key).hash(state);
        self.get_note_names(self.key).hash(state);
    }
}

impl Serialize for Chord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut state = serializer.serialize_struct("Chord", 2)?;
        state.serialize_field("name", &self.get_name(self.key))?;
        state.serialize_field("notes", &self.get_note_names(self.key))?;
        state.end()
    }
}

impl Chord {
    /// Creates a new chord.
    pub fn new(root_index: u8, chord_type: &ChordType) -> Self {
        Chord {
            chord_type: chord_type.to_owned(),
            root: root_index,
            key: 0,
            _chords_to_not_play_next: Vec::new()
        }
    }

    
    /// Gets an array of pitches of the notes for this chord
    /// returns note_intervals.map(|n| n + self.root)
    pub fn get_notes(&self) -> Vec<i16> {
        let mut result: Vec<i16> = Vec::new();
        for n in self.chord_type.to_owned().note_intervals {
            result.push((n + self.root) as i16);
        }
        return result
    }

    
    /// Gets an array of optional notes for this chord.
    pub fn get_optional_notes(&self) -> Vec<i16> {
        let mut result: Vec<i16> = Vec::new();
        for n in self.chord_type.optional_notes.iter() {
            result.push((n + self.root) as i16);
        }
        return result
    }

    /// Gets the name of this chord, given the current key.
    pub fn get_name(&self, key: i16) -> String {
        let letter = KEYS[(self.root as usize + key as usize) % 12];
        format!("{} {}", letter, self.chord_type.name)
    }

    /// Gets the names of the notes of this chord.
    /// 
    /// Returns a string like "F, A, C#, E"
    pub fn get_note_names(&self, key: i16) -> String {
        let notes = self.get_notes();
        if notes.len() == 0 {
            return String::new();
        }
        let mut result = String::with_capacity(4 * notes.len());
        result.push_str(KEYS[(notes[0] as usize + key as usize) % 12]);
        for note in notes.iter().skip(1) {
            result.push_str(&format!(", {}", KEYS[(*note as usize + key as usize) % 12]))
        }
        result
    }
}