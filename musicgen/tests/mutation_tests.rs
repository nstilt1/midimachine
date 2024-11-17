extern crate musicgen;
#[cfg(not(target_arch="wasm32"))]
use std::fs;

#[cfg(not(target_arch="wasm32"))]
use musicgen::test_utils::{generate_midi_files, FILENAMES, GENERATION_MODES};

#[test]
#[cfg(not(target_arch="wasm32"))]
fn generation_mode_mutations() {
    for (generation_mode, filename) in GENERATION_MODES.iter().zip(FILENAMES) {
        let (midi_files, suffixes) = generate_midi_files(&generation_mode);

        for (midi_file, suffix) in midi_files.iter().zip(suffixes) {
            let master_file = fs::read(format!("./tests/blobs/{}{}.mid", filename, suffix)).unwrap();
            assert!(midi_file.eq(&master_file), "`{}{}.mid` suffered a breaking change.", filename, suffix);
        }
    }
}