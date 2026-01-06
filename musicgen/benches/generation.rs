use criterion::{black_box, criterion_group, criterion_main, Criterion};
use midly::Smf;
use musicgen::{music_modules_v2::midi::MidiFile, test_utils::generate_midi_all_chord_types};
use sha2::{digest::Output, Digest, Sha256};

const NUM_CHORDS: usize = 10_000;
const KEY: &str = "Fmin";

fn generate_melodies(c: &mut Criterion) {
    c.bench_function("generate_melodies", |b| {
        b.iter(|| {
            black_box(generate_midi_all_chord_types(
                "test", 
                "melody", 
                NUM_CHORDS, 
                KEY, 
                "original", 
                0,
            ))
        })
    });
}

fn generate_chords(c: &mut Criterion) {
    c.bench_function("generate_chords", |b| {
        b.iter(|| {
            black_box(generate_midi_all_chord_types(
                "test", 
                "chords", 
                NUM_CHORDS, 
                KEY, 
                "original", 
                0,
            ))
        })
    });
}

fn setup_chord_choices() -> MidiFile {
    let hash: Output<Sha256> = Sha256::digest("test".as_bytes());
    let mut musician = musicgen::music_modules_v2::Music::smoke_hash_all_custom_handpicked_chords(hash, KEY);
    musician.make_music_no_finalize(
        NUM_CHORDS, 
        "chords", 
        false, 
        "original", 
        0,
        4
    )
}

#[inline]
fn finalize_midi_file(mut midi_file: MidiFile) {
    let track = midi_file.finalize();
    let smf = Smf {
        header: midly::Header { 
            format: midly::Format::SingleTrack, 
            timing: midly::Timing::Metrical(96.into()) 
        },
        tracks: vec![track]
    };

    let mut output = vec![];
    smf.write(&mut output).unwrap();
}

fn generate_file(c: &mut Criterion) {
    c.bench_function("generate_file", |b| {
        b.iter_batched(
            || setup_chord_choices(),
            |midi_file| {
                black_box(finalize_midi_file(midi_file));
            },
            criterion::BatchSize::PerIteration
        );
    });
}

criterion_group!(benches, generate_melodies, generate_chords, generate_file);
criterion_main!(benches);