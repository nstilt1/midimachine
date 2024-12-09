use criterion::{black_box, criterion_group, criterion_main, Criterion};
use musicgen::test_utils::generate_midi_all_chord_types;

const NUM_CHORDS: usize = 10_000;

fn generate_melodies(c: &mut Criterion) {
    c.bench_function("generate_melodies", |b| {
        b.iter(|| {
            black_box(generate_midi_all_chord_types(
                "test", 
                "melody", 
                NUM_CHORDS, 
                "Fmin", 
                "original", 
                0
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
                "Fmin", 
                "original", 
                0
            ))
        })
    });
}

criterion_group!(benches, generate_melodies, generate_chords);
criterion_main!(benches);