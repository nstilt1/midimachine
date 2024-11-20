use std::{collections::HashSet, hash::Hash};

use base64::{engine::general_purpose, Engine};
use midly::Smf;
use serde::{ser::SerializeStruct, Serialize};

use crate::music_modules_v2::music::KEYS;

use super::{chord_type::*, midi::MidiFile, utils::CustomIterators};

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
        self.get_name().eq(&other.get_name())
            && self.get_note_names().eq(&other.get_note_names())
    }
}

impl Eq for Chord {}

impl Hash for Chord {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_name().hash(state);
        self.get_note_names().hash(state);
    }
}

impl Serialize for Chord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        let mut state = serializer.serialize_struct("Chord", 3)?;
        state.serialize_field("name", &self.get_name())?;
        state.serialize_field("notes", &self.get_note_names())?;
        state.serialize_field("midi", &self.to_midi())?;
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
            result.push((n + self.root + self.key as u8) as i16);
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
    pub fn get_name(&self) -> String {
        let letter = KEYS[(self.root as usize + self.key as usize) % 12];
        format!("{} {}", letter, self.chord_type.name)
    }

    /// Gets the names of the notes of this chord.
    /// 
    /// Returns a string like "F, A, C#, E"
    pub fn get_note_names(&self) -> String {
        let notes = self.get_notes();
        if notes.len() == 0 {
            return String::new();
        }
        let mut result = String::with_capacity(4 * notes.len());
        result.push_str(KEYS[(notes[0] as usize) % 12]);
        for note in notes.iter().skip(1) {
            result.push_str(&format!(", {}", KEYS[(*note as usize) % 12]))
        }
        result
    }

    pub fn to_midi(&self) -> String {
        let mut track = MidiFile::new();
        for note in self.get_notes() {
            track.add_note_beats(note as u8 + 12 * 4, 0f64, 1f64, 80);
        }
        let track = track.finalize();

        let smf = Smf {
            header: midly::Header { format: midly::Format::SingleTrack, timing: midly::Timing::Metrical(96.into())},
            tracks: vec![track]
        };

        let mut output = vec![];
        smf.write(&mut output).expect("Should be valid");

        let mut base64 = String::new();
        general_purpose::STANDARD.encode_string(output, &mut base64);

        base64
    }

    pub fn notes_set(&self) -> HashSet<i16> {
        HashSet::from_iter(self.get_notes().transpose(0))
    }
}