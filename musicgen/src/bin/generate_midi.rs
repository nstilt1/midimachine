//! This binary is used for populating the `/tests/blobs` folder with various
//! outputs so that breaking changes can be detected by running `cargo test`.

#[cfg(not(target_arch="wasm32"))]
use musicgen::test_utils::{FILENAMES, GENERATION_MODES, generate_midi_files};

/// Writes a midi file to the ./tests/blobs/ folder.
#[cfg(not(target_arch="wasm32"))]
fn write_midi_file(filename: &str, content: &[u8]) -> std::io::Result<()> {
    use std::{fs::{self, OpenOptions}, io::Write, path::Path};

    let dir = Path::new("./tests/blobs/");
    let path = dir.join(filename);

    // Create the directory if it doesn't exist
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    file.write_all(content)?;
    Ok(())
}

/// Writes a bunch of midi files to test the output against.
#[cfg(not(target_arch="wasm32"))]
fn write_midi_files() {
    for (generation_mode, filename) in GENERATION_MODES.iter().zip(FILENAMES) {
        let (midi_files, suffixes) = generate_midi_files(&generation_mode);
        
        for (midi_file, suffix) in midi_files.iter().zip(suffixes) {
            write_midi_file(&format!("{}{}.mid", filename, suffix), &midi_file).unwrap();
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), std::io::Error> {
    write_midi_files();
    Ok(())
}

#[cfg(target_arch="wasm32")]
fn main() {}