extern crate musicgen;
use std::fs;

use musicgen::generate_midi;

fn gen_midi(is_melody: bool, use_same_chords: bool) -> Vec<u8> {
    generate_midi(
        b"a",
        if is_melody { "melody" } else { "chords" },
        use_same_chords,
        100,
        "random",
        "",
        "default",
        "original"
    ).unwrap()
}

#[test]
fn melody_mutations() {
    let generated = gen_midi(true, false);
    let master_content = fs::read("./tests/blobs/config1.mid").unwrap();
    assert!(generated.eq(&master_content));

    let generated_same_chords = gen_midi(true, true);
    let master_content = fs::read("./tests/blobs/config2.mid").unwrap();
    assert!(generated_same_chords.eq(&master_content));
}

#[test]
fn chords_mutations() {
    let generated = gen_midi(false, false);
    let master_content = fs::read("./tests/blobs/config3.mid").unwrap();
    assert!(generated.eq(&master_content));
}