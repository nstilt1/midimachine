use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use musicgen::music_modules_v2::{pruning::{prune_chords, prune_chords_u16}, Music};

fn setup_input_data() -> Music {
    // Example setup logic: Generate a vector with random data
    Music::smoke_hash_all_pruning_chords("Cmin", "disabled")
}

fn pruning_hash_sets(c: &mut Criterion) {
    c.bench_function("pruning_hash_sets", |b| {
        b.iter_batched(
            || setup_input_data(),   // Setup logic (runs before each iteration)
            |mut musician| {
                criterion::black_box(prune_chords(&mut musician.chord_table, &mut musician.chord_list, "natural", true));          // Prevent compiler optimizations
            },
            BatchSize::PerIteration         // Batch size (explained below)
        );
    });
}

fn pruning_u16(c: &mut Criterion) {
    c.bench_function("pruning_u16", |b| {
        b.iter_batched(
            || setup_input_data(),   // Setup logic (runs before each iteration)
            |mut musician| {
                criterion::black_box(prune_chords_u16(&mut musician.chord_table, &mut musician.chord_list, "natural", true));          // Prevent compiler optimizations
            },
            BatchSize::PerIteration         // Batch size (explained below)
        );
    });
}

criterion_group!(benches, pruning_hash_sets, pruning_u16);
criterion_main!(benches);