use std::collections::HashSet;

#[cfg(target_arch = "wasm32")]
use js_sys::Array;
use midly::Smf;
use music_modules_v2::{error::MusicError, Music};
use wasm_bindgen::prelude::*;
use sha2::{Digest, Sha256};

mod music_modules_v2;

#[derive(Debug)]
pub enum Error {
    MusicError(MusicError),
    MidlyError(midly::Error),
    StrError(String),
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str("Error")
    }
}

impl From<MusicError> for Error {
    fn from(value: MusicError) -> Self {
        Self::MusicError(value)
    }
}

impl From<midly::Error> for Error {
    fn from(value: midly::Error) -> Self {
        Self::MidlyError(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::StrError(value.to_string())
    }
}

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
pub fn generate_midi(
    file_content: &[u8], 
    generation_mode: &str, 
    should_use_same_chords: bool, 
    num_chords: usize, 
    key: &str,
    chord_selection: Array,
    chord_type_group: &str,
    chord_picking_method: &str,
    min_number_of_unique_chords: u32,
) -> Result<Vec<u8>, Error> {
    let hash = Sha256::digest(file_content);
    
    let chord_selection_hashset: HashSet<String> = chord_selection.iter()
        .map(|js_val| js_val.as_string().unwrap_or_default())
        .collect();
    // smoke the hash
    let mut musician = Music::smoke_hash(hash, key, &chord_selection_hashset, chord_type_group)?;
    let track = musician.make_music(num_chords, generation_mode, should_use_same_chords, chord_picking_method, min_number_of_unique_chords)?;

    let smf = Smf {
        header: midly::Header { format: midly::Format::SingleTrack, timing: midly::Timing::Metrical(96.into()) },
        tracks: vec![track]
    };

    let mut output = vec![];

    smf.write(&mut output)?;

    Ok(output)
}

#[cfg(not(target_arch = "wasm32"))]
pub mod test_utils {
    use super::*;
    
    pub const GENERATION_MODES: [&str; 5] = ["melody", "chords", "intended", "melody v2", "melody_v3"];
    pub const FILENAMES: [&str; 5] = ["melody_", "chords_", "intended_", "melody_v2_", "melody_v3_"];

    /// Rust variant for testing.
    #[cfg(not(target_arch="wasm32"))]
    pub fn generate_midi(
        file_content: &[u8], 
        generation_mode: &str, 
        should_use_same_chords: bool, 
        num_chords: u32, 
        key: &str,
        _chord_selection: &str,
        chord_type_group: &str,
        chord_picking_method: &str,
        min_number_of_unique_chords: u32,
    ) -> Result<Vec<u8>, Error> {
        let hash = Sha256::digest(file_content);
        
        let chord_selection_hashset: HashSet<String> = HashSet::new();
        // smoke the hash
        let mut musician = Music::smoke_hash(hash, key, &chord_selection_hashset, chord_type_group).unwrap();
        let track = musician.make_music(num_chords as usize, generation_mode, should_use_same_chords, chord_picking_method, min_number_of_unique_chords).unwrap();

        let smf = Smf {
            header: midly::Header { format: midly::Format::SingleTrack, timing: midly::Timing::Metrical(96.into()) },
            tracks: vec![track]
        };

        let mut output = vec![];

        smf.write(&mut output)?;

        Ok(output)
    }

    /// A shorthand way to create a MIDI file for a test.
    pub fn generate_midi_shorthand(generation_mode: &str, should_use_same_chords: bool, min_number_of_unique_chords: u32) -> Vec<u8> {
        generate_midi(
            b"a",
            generation_mode,
            should_use_same_chords,
            100,
            "random",
            "",
            "default",
            "original",
            min_number_of_unique_chords
        ).unwrap()
    }

    /// Generates some midi files with some different parameters
    pub fn generate_midi_files(generation_mode: &str) -> ([Vec<u8>; 4], [&'static str; 4]) {
        let midi_1 = generate_midi_shorthand(generation_mode, false, 0);
        let midi_2 = generate_midi_shorthand(generation_mode, true, 0);
        let midi_3 = generate_midi_shorthand(generation_mode, false, 8);
        let midi_4 = generate_midi_shorthand(generation_mode, true, 8);

        let midi_files = [midi_1, midi_2, midi_3, midi_4];
        let suffixes = ["a", "b", "c", "d"];

        return (midi_files, suffixes);
    }
}

#[cfg(test)]
mod tests {
    use rand::{rngs::StdRng, SeedableRng};

    use super::*;

    #[test]
    fn initializing_rng() {
        let data = b"abcdef";
        let hash = Sha256::digest(data);
        assert_eq!(hash.len(), 32);

        let mut seed = [0u8; 32];
        seed.copy_from_slice(&hash);
        let mut _rng = StdRng::from_seed(seed);
    }
}