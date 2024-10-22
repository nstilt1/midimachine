use std::{collections::HashSet, fs::{self, OpenOptions}, io::Write, path::Path};

use midly::Smf;
use music_modules_v2::Music;
use musicgen::{generate_midi, Error};
use sha2::{Digest, Sha256};

mod music_modules_v2;

fn write_midi_file(filename: &str, content: &[u8]) -> std::io::Result<()> {
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

#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), std::io::Error> {
    let midi = generate_midi(
        b"a", 
        "melody", 
        false, 
        100, 
        "random", 
        "", 
        "default", 
        "original"
    ).unwrap();

    write_midi_file("config1.mid", &midi)?;
    
    let midi = generate_midi(
        b"a",
        "melody",
        true,
        100,
        "random",
        "",
        "default",
        "original"
    ).unwrap();

    write_midi_file("config2.mid", &midi)?;

    let midi = generate_midi(
        b"a", 
        "chords", 
        false, 
        100, 
        "random", 
        "", 
        "default", 
        "original"
    ).unwrap();

    write_midi_file("config3.mid", &midi)?;
    Ok(())
}