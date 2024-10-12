
#[derive(Clone, Debug)]
pub struct ChordType {
    pub note_intervals: Vec<u8>,
    pub roots: Vec<u8>,
    pub optional_notes: Vec<u8>
}
impl ChordType {
    pub fn new(notes: Vec<u8>, root_vec: Vec<u8>, optional_notes_vec: Option<Vec<u8>>) -> Self {
        let opt_notes: Vec<u8>;
        if optional_notes_vec.is_none() {
            opt_notes = Vec::new();
        }else{
            opt_notes = optional_notes_vec.unwrap();
        }
        ChordType {
            note_intervals: notes,
            roots: root_vec,
            optional_notes: opt_notes,
        }
    }
}