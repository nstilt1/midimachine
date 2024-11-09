use std::hash::Hash;


#[derive(Clone, Debug)]
pub struct ChordType {
    pub note_intervals: Vec<u8>,
    pub roots: Vec<u8>,
    pub optional_notes: Vec<u8>
}
impl ChordType {
    pub fn new(notes: &[u8], root_vec: &[u8], optional_notes_vec: Option<&[u8]>) -> Self {
        let opt_notes: Vec<u8>;
        if optional_notes_vec.is_none() {
            opt_notes = Vec::new();
        }else{
            opt_notes = optional_notes_vec.unwrap().to_vec();
        }
        ChordType {
            note_intervals: notes.to_vec(),
            roots: root_vec.to_vec(),
            optional_notes: opt_notes,
        }
    }
    pub fn all_roots(notes: &[u8], optional_notes_vec: Option<&[u8]>) -> Self {
        let opt_notes: Vec<u8>;
        if optional_notes_vec.is_none() {
            opt_notes = Vec::new();
        } else {
            opt_notes = optional_notes_vec.unwrap().to_vec();
        }
        ChordType {
            note_intervals: notes.to_vec(),
            roots: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            optional_notes: opt_notes,
        }
    }
}

impl Default for ChordType {
    fn default() -> Self {
        Self {
            note_intervals: Vec::new(),
            roots: Vec::new(),
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