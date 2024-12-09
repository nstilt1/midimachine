use std::hash::Hash;

use crate::music_modules_v2::music::notes::*;


#[derive(Clone, Debug)]
pub struct ChordType {
    pub note_intervals: Vec<u8>,
    pub name: String,
    pub roots: Vec<u8>,
    pub optional_notes: Vec<u8>
}
impl ChordType {
    pub fn new(name: &str, intervals: &[u8], root_vec: &[u8], optional_notes_vec: Option<&[u8]>) -> Self {
        let opt_notes: Vec<u8>;
        if optional_notes_vec.is_none() {
            opt_notes = Vec::new();
        }else{
            opt_notes = optional_notes_vec.unwrap().to_vec();
        }
        ChordType {
            note_intervals: intervals.to_vec(),
            name: name.to_string(),
            roots: root_vec.to_vec(),
            optional_notes: opt_notes,
        }
    }
    pub fn all_roots(name: &str, notes: &[u8], optional_notes_vec: Option<&[u8]>) -> Self {
        let opt_notes: Vec<u8>;
        if optional_notes_vec.is_none() {
            opt_notes = Vec::new();
        } else {
            opt_notes = optional_notes_vec.unwrap().to_vec();
        }
        ChordType {
            note_intervals: notes.to_vec(),
            name: name.to_string(),
            roots: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            optional_notes: opt_notes,
        }
    }
    /// Sets this chord type to use all roots.
    pub fn use_all_roots(&mut self, should_use_all_roots: bool) {
        if should_use_all_roots {
            self.roots = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        }
    }
}

impl Default for ChordType {
    fn default() -> Self {
        Self {
            note_intervals: Vec::new(),
            roots: Vec::new(),
            name: String::new(),
            optional_notes: Vec::new()
        }
    }
}

impl PartialEq for ChordType {
    fn eq(&self, other: &Self) -> bool {
        self.note_intervals.eq(&other.note_intervals) && self.roots.eq(&other.roots)
    }
}

impl Hash for ChordType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.note_intervals.hash(state);
        self.optional_notes.hash(state);
        self.roots.hash(state);
    }
}

/// Default chord type definitions
pub fn default_chord_types() -> Vec<ChordType> {
    let minor7 = ChordType::new("minor 7", &[0, 10, 15, 19], &[C, D, F, FSHARP, ASHARP], None);
    let major7 = ChordType::new("major 7", &[0, 11, 16, 19], &[DSHARP, GSHARP], None);
    let diminished = ChordType::new("diminished", &[0, 3, 6], &[DSHARP, FSHARP], None);
    let augmented = ChordType::new("augmented", &[0,4,8], &[D, FSHARP, ASHARP], Some(&[12]));
    //let major6 = ChordType::new(&[0, 4, 7, 9], &[3, 10], None);
    let major6 = ChordType::new("major 6", &[0, 9, 16, 19], &[DSHARP, GSHARP, ASHARP], Some(&[23]));

    let minor6 = ChordType::new("minor 6", &[0, 9, 15, 19], &[C, D, F, G], None);
    let major9 = ChordType::new("major 9", &[0, 4, 10, 14], &[C, F, G], None);
    let major7sharp9 = ChordType::new("major 7 #9", &[0, 4, 10, 15], &[C, D, G, A], None);
    let major7flat5sharp9 = ChordType::new("major 7b5#9", &[0, 4, 10, 15, 18], &[C, A], None);
    let major9flat5 = ChordType::new("major 9b5", &[0, 4, 10, 15, 17], &[C, A], None);
    let major7flat9 = ChordType::new("major 7b9", &[0, 4, 10, 13], &[C, D], None);
    
    vec![
        minor7,
        major7,
        diminished,
        augmented,
        major6,
        minor6,
        major9,
        major7sharp9,
        major7flat5sharp9,
        major9flat5,
        major7flat9
    ]
}

#[cfg(not(target_arch="wasm32"))]
#[allow(unused)]
pub fn all_custom_chord_types_handpicked() -> Vec<ChordType> {
    let minor7 = ChordType::new("minor 7", &[0, 10, 15, 19], &[C, D, F, FSHARP, ASHARP], None);
    let major7 = ChordType::new("major 7", &[0, 11, 16, 19], &[DSHARP, GSHARP], None);
    let diminished = ChordType::new("diminished", &[0, 3, 6], &[DSHARP, FSHARP], None);
    let augmented = ChordType::new("augmented", &[0,4,8], &[D, FSHARP, ASHARP], Some(&[12]));
    //let major6 = ChordType::new(&[0, 4, 7, 9], &[3, 10], None);
    let major6 = ChordType::new("major 6", &[0, 9, 16, 19], &[DSHARP, GSHARP, ASHARP], Some(&[23]));

    let minor6 = ChordType::new("minor 6", &[0, 9, 15, 19], &[C, D, F, G], None);
    let major9 = ChordType::new("major 9", &[0, 4, 10, 14], &[C, F, G], None);
    let major7sharp9 = ChordType::new("major 7#9", &[0, 4, 10, 15], &[C, D, G, A], None);
    let major7flat5sharp9 = ChordType::new("major 7b5#9", &[0, 4, 10, 15, 18], &[C, A], None);
    let major9flat5 = ChordType::new("major 9b5", &[0, 4, 10, 15, 17], &[C, A], None);
    let major7flat9 = ChordType::new("major 7b9", &[0, 4, 10, 13], &[C, D], None);

    // extra chord types
    let major = ChordType::new("major", &[0, 4, 7], &[DSHARP, GSHARP, ASHARP], None);
    let minor = ChordType::new("minor", &[0, 3, 7], &[C, D, F, G], None);

    let minor9 = ChordType::new("minor 9", &[0, 3, 7, 10, 14], &[G], None);

    let major13 = ChordType::new("major 13", &[0, 5, 10, 21, 26, 31], &[C], Some(&[0, 5]));
    let dominant9 = ChordType::new("dominant 9", &[0, 4, 9, 14, 18], &[CSHARP], None);
    
    let add9 = ChordType::new("add 9", &[0, 4, 7, 14], &[DSHARP, ASHARP], None);

    vec![
        minor7,
        major7,
        diminished,
        augmented,
        major6,
        minor6,
        major9,
        major7sharp9,
        major7flat5sharp9,
        major9flat5,
        major7flat9,
        major,
        minor,
        minor9,
        major13,
        dominant9,
        add9
    ]
}