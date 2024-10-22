use std::{fs::{self, OpenOptions}, io::Write, path::Path};
use musicgen::generate_midi;

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