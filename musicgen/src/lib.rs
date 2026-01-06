#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use std::{collections::HashSet, fmt::Display};

#[cfg(target_arch = "wasm32")]
use js_sys::Array;
use midly::Smf;
#[allow(unused)]
use music_modules_v2::{midi::MidiFile, Music};
use wasm_bindgen::prelude::*;
use sha2::{Digest, Sha256};

pub mod music_modules_v2;

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {
        if true {
            let x = format!($($arg)*);
            crate::log(&x);
        }
    };
}

#[cfg(not(target_arch = "wasm32"))]
#[macro_export]
macro_rules! console_log {
    ($($arg:tt)*) => {
        if true {
            println!($($arg)*);
        }
    };
}

#[derive(Debug)]
pub enum Error {
    MidlyError(midly::Error),
    StrError(String),
    SerdeError(serde_json::Error),
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str("Error")
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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::MidlyError(v) => v.to_string(),
            Self::SerdeError(v) => v.to_string(),
            Self::StrError(v) => v.to_string()
        };
        f.write_str(&s)
    }
}

impl From<Error> for JsError {
    fn from(value: Error) -> Self {
        Self::new(&value.to_string())
    }
}

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
pub fn get_chords_of_key(
    mut key: &str,
    chord_selection: Array,
    chord_type_group: &str,
    scale: &str,
    table_scheme: &str,
    show_probabilities: bool,
) -> Result<String, JsError> {
    use serde_json::json;

    let use_all_roots = key.eq("random");
    if use_all_roots {
        key = "Cmin";
    }
    let chord_selection_hashset: HashSet<String> = chord_selection.iter()
        .map(|js_val| js_val.as_string().unwrap_or_default())
        .collect();
    let mut musician = Music::smoke_hash(Default::default(), "Cmin", &chord_selection_hashset, chord_type_group, scale, false, use_all_roots)?;

    match table_scheme {
        "contains_note" => musician.rotate_chords(key),
        "highest_note" => musician.rearrange_by_highest_note(key),
        "lowest_note" => musician.rearrange_by_lowest_note(key),
        _ => return Err(JsError::new("table_scheme did not match"))
    }

    if show_probabilities {
        musician.set_probabilities();
    }

    // sort the sub-arrays
    for col in musician.chord_table.iter_mut() {
        col.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));
    }

    musician.chord_list.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));

    let json = json!({
        "chord_list": musician.chord_list,
        "chord_table": musician.chord_table
    });

    //let json = to_string(&musician.chord_table)?;
    
    Ok(json.to_string())
}

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
pub fn chord_finder(
    mut key: &str,
    chord_selection: Array,
    chord_type_group: &str,
    scale: &str,
    notes: Array,
    table_scheme: &str
) -> Result<String, JsError> {
    use music_modules_v2::{chord::Chord, utils::{parse_key, sets::{SetMath, SetOpsCollection}}};
    use serde_json::json;
    let use_all_roots = key.eq("random");
    if use_all_roots {
        key = "Cmin";
    }
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
    let mut musician = Music::smoke_hash(Default::default(), "Cmin", &chord_selection_hashset, chord_type_group, scale, false, use_all_roots)?;

    musician.rotate_chords(key);

    let mut intersected_chords: HashSet<Chord> = HashSet::from_iter(musician.chord_table[notes_vec[0]].iter().cloned());
    for note in notes_vec.iter().skip(1) {
        intersected_chords = intersected_chords
            .intersection(
                &HashSet::from_iter(musician.chord_table[*note % 12]
                    .iter()
                    .cloned()
                )
            ).to_set();
    }

    match table_scheme {
        "contains_note" => (),
        "highest_note" => musician.rearrange_by_highest_note(key),
        "lowest_note" => musician.rearrange_by_lowest_note(key),
        _ => return Err(JsError::new("table_scheme did not match"))
    }

    for chords in musician.chord_table.iter_mut() {
        *chords = HashSet::from_iter(chords.iter().cloned())
            .intersection(&intersected_chords)
            .to_vec();
    }
    musician.chord_list = intersected_chords.to_vec();

    // sort the sub-arrays
    for col in musician.chord_table.iter_mut() {
        col.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));
    }

    musician.chord_list.sort_unstable_by(|a, b| a.get_name().cmp(&b.get_name()));

    let json = json!({
        "chord_list": musician.chord_list,
        "chord_table": musician.chord_table
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
    is_reproducible: bool,
    pattern_to_use: &str,
    duration: u32,
) -> Result<Vec<u8>, Error> {
    let hash = Sha256::digest(file_content);
    
    let chord_selection_hashset: HashSet<String> = chord_selection.iter()
        .map(|js_val| js_val.as_string().unwrap_or_default())
        .collect();
    let pattern_to_use: Vec<u8> = match generation_mode {
        "chords" => {
            use crate::music_modules_v2::patterns::validation::validate_pattern;

            let (is_valid, pattern) = validate_pattern(pattern_to_use);
            if !is_valid {
                Vec::new()
            } else {
                pattern
            }
        },
        _ => Vec::new(),
    };
    let mut max_chords = num_chords;
    for chord_idx in pattern_to_use.iter() {
        max_chords = max_chords.max(*chord_idx as usize);
    }
    let num_chords = max_chords;
    // smoke the hash
    let mut musician = Music::smoke_hash(hash, key, &chord_selection_hashset, chord_type_group, scale, is_reproducible, false)?;
    let track = musician.make_music(num_chords, generation_mode, should_use_same_chords, chord_picking_method, min_number_of_unique_chords, &pattern_to_use, duration)?;

    let smf = Smf {
        header: midly::Header { format: midly::Format::SingleTrack, timing: midly::Timing::Metrical(96.into()) },
        tracks: vec![track]
    };

    let mut output = Vec::new();

    smf.write(&mut output)?;

    Ok(output)
}

#[wasm_bindgen]
#[cfg(target_arch="wasm32")]
pub fn generate_midi_chord_progression(chords: JsValue) -> Result<Vec<u8>, JsError> {
    let chords: Vec<Vec<usize>> = serde_wasm_bindgen::from_value(chords).expect("Bad input");
    
    let mut track = MidiFile::new();
    let mut time = 0f64;
    for chord in chords.iter() {
        for note in chord {
            track.add_note_beats(*note as u8, time, 4f64, 80);
        }
        time += 4f64;
    }
    let track = track.finalize();
    let smf = Smf {
        header: midly::Header { format: midly::Format::SingleTrack, timing: midly::Timing::Metrical(96.into()) },
        tracks: vec![track]
    };
    let mut output = Vec::new();
    smf.write(&mut output).expect("Should be valid");
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
        let mut musician = Music::smoke_hash(hash, key, &chord_selection_hashset, chord_type_group, "disabled", true, false).unwrap();
        let track = musician.make_music(num_chords as usize, generation_mode, should_use_same_chords, chord_picking_method, min_number_of_unique_chords, &Vec::new(), 4).unwrap();

        let smf = Smf {
            header: midly::Header { format: midly::Format::SingleTrack, timing: midly::Timing::Metrical(96.into()) },
            tracks: vec![track]
        };

        let mut output = Vec::new();

        smf.write(&mut output)?;

        Ok(output)
    }

    pub fn generate_midi_all_chord_types(
        text_input: &str,
        generation_mode: &str,
        num_chords: usize,
        key: &str,
        chord_picking_method: &str,
        min_number_of_unique_chords: u32,
    ) -> Vec<u8> {
        let hash = Sha256::digest(text_input.as_bytes());
        let mut musician = Music::smoke_hash_all_custom_handpicked_chords(hash, key);
        let track = musician.make_music(num_chords, generation_mode, false, chord_picking_method, min_number_of_unique_chords, &Vec::new(), 4).unwrap();

        let smf = Smf {
            header: midly::Header {
                format: midly::Format::SingleTrack,
                timing: midly::Timing::Metrical(96.into())
            },
            tracks: vec![track]
        };

        let mut output = Vec::new();
        smf.write(&mut output).unwrap();

        output
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
            min_number_of_unique_chords,
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
        let text_input = "write a song about being lost at voodoo";
        let vibe_input = "4";

        let concatenated = format!("{}{}{}", file_input, text_input, vibe_input);

        // use an RNG to make decisions
        let mut rng = StdRng::from_seed(hash(concatenated.as_bytes()));

        // the output stream will always be the same for a given seed
        assert_eq!(rng.next_u32(), 529601244);
        assert_eq!(rng.next_u32(), 2730124356);
        assert_eq!(rng.next_u32(), 3297863714);
        assert_eq!(rng.next_u32(), 2846115852);
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