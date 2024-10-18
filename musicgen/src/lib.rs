use std::collections::HashSet;

use js_sys::Array;
use midly::Smf;
use music_modules_v2::Music;
use my_modules::error::HttpError;
use wasm_bindgen::prelude::*;
use sha2::{Digest, Sha256};

mod music_modules_v2;
mod my_modules;

pub enum Error {
    HttpError(HttpError),
    MidlyError(midly::Error),
    StrError(String),
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str("Error")
    }
}

impl From<HttpError> for Error {
    fn from(value: HttpError) -> Self {
        Self::HttpError(value)
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
pub fn generate_midi(
    file_content: &[u8], 
    generation_mode: &str, 
    should_use_same_chords: bool, 
    num_chords: usize, 
    key: &str,
    chord_selection: Array,
    chord_type_group: &str
) -> Result<Vec<u8>, Error> {
    let hash = Sha256::digest(file_content);
    
    let chord_selection_hashset: HashSet<String> = chord_selection.iter()
        .map(|js_val| js_val.as_string().unwrap_or_default())
        .collect();
    // smoke the hash
    let mut musician = Music::smoke_hash(hash, key, &chord_selection_hashset, chord_type_group)?;
    let track = musician.make_music(num_chords, generation_mode, should_use_same_chords)?;

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