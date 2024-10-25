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