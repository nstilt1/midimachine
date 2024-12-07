use std::{collections::HashSet, hash::Hash};

use base64::{engine::general_purpose, Engine};
use midly::Smf;
use serde::{ser::SerializeStruct, Serialize};

use crate::music_modules_v2::music::KEYS;

use super::{chord_type::*, midi::MidiFile};

#[derive(Clone, Debug)]
pub struct Chord {
    pub chord_type: ChordType,
    pub root: u8,
    pub key: i16,
    pub probability_2d: f32,
    pub probability_1d: f32,
    pub show_probability: bool,
    _chords_to_not_play_next: Vec<Chord>,
}

impl Default for Chord {
    fn default() -> Self {
        Self {
            chord_type: ChordType::default(),
            root: 0,
            key: 0,
            probability_2d: 0f32,
            probability_1d: 0f32,
            show_probability: false,
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
        self.root.hash(state);
        self.chord_type.note_intervals.hash(state);
        self.chord_type.optional_notes.hash(state);
    }
}

impl Serialize for Chord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer 
    {
        // increment this by one when adding a new serialized field
        let num_standard_fields = 4;
        let mut state = if self.show_probability {
            let mut state = serializer.serialize_struct("Chord", num_standard_fields + 2)?;
            state.serialize_field("probability_2d", &format!("{:.1}%", &self.probability_2d * 100f32))?;
            state.serialize_field("probability_1d", &format!("{:.1}%", &self.probability_1d * 100f32))?;
            state
        } else {
            serializer.serialize_struct("Chord", num_standard_fields)?
        };
        state.serialize_field("name", &self.get_name())?;
        state.serialize_field("notes", &self.get_note_names())?;
        state.serialize_field("midi", &self.to_midi())?;
        state.serialize_field("note_vec", &self.get_notes_vec())?;
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
            probability_2d: 0f32,
            probability_1d: 0f32,
            show_probability: false,
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
        for note in self.get_notes_vec() {
            track.add_note_beats(note as u8, 0f64, 1f64, 80);
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

    fn get_notes_vec(&self) -> Vec<i16> {
        let mut result = self.get_notes();
        result.iter_mut().for_each(|note| *note = *note + 12 * 4);
        result
    }

    pub fn get_notes_u16(&self) -> u16 {
        let mut result = 0u16;
        for note in self.get_notes() {
            result |= 1 << (note % 12);
        }
        result
    }
}

/// Expands chords into new chords based on their optional notes, adding them
pub fn expand_chords(chord_list: &mut Vec<Chord>) -> (HashSet<Chord>, Vec<HashSet<Chord>>) {
    let mut chord_set: HashSet<Chord> = HashSet::with_capacity(chord_list.capacity());
    let mut chord_table_sets: Vec<HashSet<Chord>> = vec![HashSet::new(); 12];
    for chord in chord_list.iter() {
        let mut base_chord_type = chord.chord_type.clone();
        let optional_notes = base_chord_type.optional_notes;
        let root = chord.root;
        base_chord_type.optional_notes = Vec::new();
        let base_chord = Chord::new(chord.root, &base_chord_type);
        for note in base_chord.get_notes() {
            chord_table_sets[(note + chord.key) as usize % 12].insert(base_chord.clone());
        }
        chord_set.insert(base_chord.clone());

        // add more chords with different combinations of notes to the sets
        // `cumulated_notes` is used to add chords with 1, 2, ..., n optional notes
        // to the sets
        // `notes` is used to add chords with just one of the optional notes
        let mut cumulated_notes = base_chord_type.note_intervals.clone();
        for note in optional_notes.iter() {
            let mut notes = base_chord_type.note_intervals.clone();
            cumulated_notes.push(*note);
            notes.push(*note);
            let new_chord_type = ChordType::new(&base_chord_type.name, &cumulated_notes, &[root], None);
            let new_chord_type_2 = ChordType::new(&base_chord_type.name, &notes, &[root], None);
            let new_chord = Chord::new(chord.root, &new_chord_type);
            let new_chord_2 = Chord::new(chord.root, &new_chord_type_2);
            chord_set.insert(new_chord.to_owned());
            chord_set.insert(new_chord_2.to_owned());
            for c in &[&new_chord, &new_chord_2, &base_chord] {
                for n_2 in c.get_notes().iter() {
                    let index = (*n_2 as u8) % 12;
                    chord_table_sets[index as usize].insert(c.to_owned().to_owned());
                }
            }
        }
        // accumulate chords with optional notes added in reverse order
        if optional_notes.len() > 2 {
            let mut cumulated_notes = base_chord_type.note_intervals.clone();
            for note in optional_notes.iter().rev() {
                cumulated_notes.push(*note);
                let new_chord_type = ChordType::new(&base_chord_type.name, &cumulated_notes, &[root], None);
                let new_chord = Chord::new(chord.root, &new_chord_type);
                chord_set.insert(new_chord.to_owned());
                for n_2 in new_chord.get_notes().iter() {
                    let index = (*n_2 as u8) % 12;
                    chord_table_sets[index as usize].insert(new_chord.to_owned());
                }
            }
        }
    }
    (chord_set, chord_table_sets)
}