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
    SerdeError(serde_json::Error),
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

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerdeError(value)
    }
}

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
pub fn get_chords_of_key(
    key: &str,
    chord_selection: Array,
    chord_type_group: &str,
    scale: &str
) -> Result<String, Error> {
    use music_modules_v2::utils::parse_key;
    use serde_json::json;

    let chord_selection_hashset: HashSet<String> = chord_selection.iter()
        .map(|js_val| js_val.as_string().unwrap_or_default())
        .collect();
    let mut musician = Music::smoke_hash(Default::default(), "Cmin", &chord_selection_hashset, chord_type_group, scale)?;

    let key_int = parse_key(key);
    //translate_and_prune(&mut musician.notes_of_chords, &mut musician.all_chords, key, scale);

    for chord in musician.all_chords.iter_mut() {
        chord.key = key_int;
        //chord.root = (chord.root + key_int as u8) % 12;
    }

    for chords in musician.notes_of_chords.iter_mut() {
        for chord in chords.iter_mut() {
            chord.key = key_int;
            //chord.root = (chord.root + key_int as u8) % 12;
        }
    }

    musician.notes_of_chords.rotate_right(key_int as usize);

    // sort the sub-arrays
    for col in musician.notes_of_chords.iter_mut() {
        col.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));
    }

    musician.all_chords.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));

    let json = json!({
        "all_chords": musician.all_chords,
        "notes_of_chords": musician.notes_of_chords
    });

    //let json = to_string(&musician.notes_of_chords)?;
    
    Ok(json.to_string())
}

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
pub fn chord_finder(
    key: &str,
    chord_selection: Array,
    chord_type_group: &str,
    scale: &str,
    notes: Array,
) -> Result<String, Error> {
    use music_modules_v2::{chord::Chord, utils::parse_key};
    use serde_json::json;

    let chord_selection_hashset: HashSet<String> = chord_selection.iter()
        .map(|js_val| js_val.as_string().unwrap_or_default())
        .collect();
    let notes_vec: Vec<usize> = notes
        .iter()
        .map(|js_val| parse_key(&js_val.as_string().unwrap_or_default()) as usize)
        .collect();
    if notes_vec.is_empty() {
        return Ok(json!({}).to_string())
    }
    let mut musician = Music::smoke_hash(Default::default(), "Cmin", &chord_selection_hashset, chord_type_group, scale)?;

    let key_int = parse_key(key);

    for chord in musician.all_chords.iter_mut() {
        chord.key = key_int;
    }

    for chords in musician.notes_of_chords.iter_mut() {
        for chord in chords.iter_mut() {
            chord.key = key_int;
        }
    }

    musician.notes_of_chords.rotate_right(key_int as usize);

    let mut intersected_chords: HashSet<Chord> = HashSet::from_iter(musician.notes_of_chords[notes_vec[0]].iter().cloned());
    for note in notes_vec.iter().skip(1) {
        intersected_chords = intersected_chords
            .intersection(
                &HashSet::from_iter(musician.notes_of_chords[*note]
                    .iter()
                    .cloned()
                )
            ).cloned()
            .collect();
    }

    for chords in musician.notes_of_chords.iter_mut() {
        *chords = HashSet::from_iter(chords.iter().cloned())
            .intersection(&intersected_chords)
            .cloned()
            .collect();
    }
    musician.all_chords = intersected_chords.iter().cloned().collect();

    // sort the sub-arrays
    for col in musician.notes_of_chords.iter_mut() {
        col.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));
    }

    musician.all_chords.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));

    let json = json!({
        "all_chords": musician.all_chords,
        "notes_of_chords": musician.notes_of_chords
    });

    Ok(json.to_string())
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
    scale: &str,
) -> Result<Vec<u8>, Error> {
    let hash = Sha256::digest(file_content);
    
    let chord_selection_hashset: HashSet<String> = chord_selection.iter()
        .map(|js_val| js_val.as_string().unwrap_or_default())
        .collect();
    // smoke the hash
    let mut musician = Music::smoke_hash(hash, key, &chord_selection_hashset, chord_type_group, scale)?;
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
        let mut musician = Music::smoke_hash(hash, key, &chord_selection_hashset, chord_type_group, "disabled").unwrap();
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
    use rand::{rngs::StdRng, RngCore, SeedableRng};

    use super::*;

    fn hash(data: &[u8]) -> [u8; 32] {
        let hash = Sha256::digest(data);
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&hash);
        bytes
    }

    #[test]
    fn hashing_demo() {
        let file_input = "some file's contents";
        let text_input = "write a chord progression about being lost at voodoo";
        let vibe_input = "4";

        let concatenated = format!("{}{}{}", file_input, text_input, vibe_input);

        // use an RNG to make decisions
        let mut rng = StdRng::from_seed(hash(concatenated.as_bytes()));

        // the output stream will always be the same for a given seed
        assert_eq!(rng.next_u32(), 430701571);
        assert_eq!(rng.next_u32(), 4153666748);
        assert_eq!(rng.next_u32(), 3817228526);
        assert_eq!(rng.next_u32(), 59595166);
    }

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