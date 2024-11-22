use std::collections::HashSet;

use super::{chord::{expand_chords, Chord}, chord_type::ChordType, utils::parse_key};

/// Removes chords that have notes outside of the chosen scale
/// The base key is C Minor, so for the natural minor scale, we will remove:
/// * C# - 1
/// * E  - 4
/// * F# - 6
/// * A  - 9
/// * B - 11
pub fn prune_chords(chord_table: &mut Vec<Vec<Chord>>, chord_list: &mut Vec<Chord>, scale: &str, key: i16) {
    

    let good_notes_set: HashSet<usize> = match scale {
        "disabled" => return,
        "natural" => HashSet::from([0, 2, 3, 4, 5, 7, 8, 10]),
        "melodic" => HashSet::from([0, 2, 3, 5, 7, 9, 11]),
        "harmonic" => HashSet::from([0, 2, 3, 5, 7, 8, 11]),
        "pentatonic" => HashSet::from([0, 3, 5, 7, 10]),
        "romanian" => HashSet::from([0, 2, 3, 6, 7, 9, 10]),
        "hungarian" => HashSet::from([0, 2, 3, 6, 7, 8, 11]),
        // "all_notes" restructures `chord_table` and `chord_list` with the
        // optional notes vecs getting converted to new chords
        "all_notes" => HashSet::from([0,1,2,3,4,5,6,7,8,9,10,11]),
        _ => return
    };

    // turn chords with optional notes into new chords
    let (chord_set, mut chord_table_sets) = expand_chords(chord_list);

    // prune the chords
    let bad_notes: Vec<usize> = HashSet::from_iter(0..12)
        .difference(&good_notes_set)
        .cloned()
        .collect();

    let mut bad_chords: HashSet<Chord> = HashSet::new();

    for mut bad_note in bad_notes {
        bad_note = (bad_note + key as usize) % 12;
        bad_chords = bad_chords.union(&chord_table_sets[bad_note]).cloned().collect();
        chord_table[bad_note] = Vec::new();
        chord_table_sets[bad_note] = HashSet::new();
    }

    for note in 0..11 {
        // version 1
        let chords: HashSet<Chord> = HashSet::from_iter(chord_table[note].iter().cloned());
        let subtracted: Vec<Chord> = chords.difference(&bad_chords).cloned().collect();
        chord_table[note] = subtracted;

        // version 2 - does not work for some reason
        // chord_table_sets[note] = chord_table_sets[note]
        //     .difference(&bad_chords)
        //     .cloned()
        //     .collect();
        // chord_table[note] = chord_table_sets[note].iter().cloned().collect();
    }

    *chord_list = chord_set.difference(&bad_chords).cloned().collect();
}

pub fn translate_and_prune(
    chord_table: &mut Vec<Vec<Chord>>, 
    chord_list: &mut Vec<Chord>, 
    key: &str,
    scale: &str
) {
    let key_int = parse_key(key);
    for chord in chord_list.iter_mut() {
        chord.key = key_int;
    }
    *chord_table = vec![Vec::new(); 12];

    for chord in chord_list.iter() {
        for note in chord.get_notes() {
            let n = (note + key_int) % 12;
            chord_table[n as usize].push(chord.clone());
        }
    }

    let mut chord_table_sets: Vec<HashSet<Chord>> = vec![HashSet::with_capacity(16); 12];
    for (i, col) in chord_table.iter().enumerate() {
        chord_table_sets[i] = HashSet::from_iter(col.iter().cloned());
    }

    let good_notes_set: HashSet<usize> = match scale {
        "disabled" => return,
        "natural" => HashSet::from([0, 2, 3, 4, 5, 7, 8, 10]),
        "melodic" => HashSet::from([0, 2, 3, 5, 7, 9, 11]),
        "harmonic" => HashSet::from([0, 2, 3, 5, 7, 8, 11]),
        "pentatonic" => HashSet::from([0, 3, 5, 7, 10]),
        "romanian" => HashSet::from([0, 2, 3, 6, 7, 9, 10]),
        "hungarian" => HashSet::from([0, 2, 3, 6, 7, 8, 11]),
        // "all_notes" restructures `chord_table` and `chord_list` with the
        // optional notes vecs getting converted to new chords
        "all_notes" => HashSet::from([0,1,2,3,4,5,6,7,8,9,10,11]),
        _ => return
    };

    let bad_notes: Vec<usize> = HashSet::from_iter(0..12)
        .difference(&good_notes_set)
        .cloned()
        .collect();

    let mut bad_chords: HashSet<Chord> = HashSet::with_capacity(12 * 8);
    for mut bad_note in bad_notes {
        bad_note = (bad_note + key_int as usize) % 12;
        bad_chords = bad_chords
            .union(&chord_table_sets[bad_note])
            .cloned()
            .collect();
        chord_table[bad_note] = Vec::new();
        chord_table_sets[bad_note] = HashSet::new();
    }

    for note in 0..11 {
        chord_table_sets[note] = chord_table_sets[note]
            .difference(&bad_chords)
            .cloned()
            .collect();
    }
    let chord_set: HashSet<Chord> = HashSet::from_iter(chord_list.iter().cloned());
    *chord_list = chord_set
        .difference(&bad_chords)
        .cloned()
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::music_modules_v2::{music::notes::*, Music};

    use super::*;

    #[test]
    fn get_bad_notes() {
        let good_notes = HashSet::from([0, 1, 2, 3, 10]);
        let mut bad_notes: Vec<usize> = HashSet::from_iter(0..12)
            .difference(&good_notes)
            .cloned()
            .collect();
        bad_notes.sort();
        assert_eq!(bad_notes, [4, 5, 6, 7, 8, 9, 11]);
    }

    #[test]
    fn pruning_c_minor() {
        let mut musician = Music::smoke_hash(
            Default::default(),
            "Cmin",
            &HashSet::new(),
            "default",
            "disabled"
        ).unwrap();

        prune_chords(&mut musician.chord_table, &mut musician.chord_list, "pentatonic", 0);

        // for chord in musician.chord_list.iter_mut() {
        //     chord.key = 0;
        // }

        // let mut result: Vec<Vec<Chord>> = vec![Vec::new(); 12];
        // for chord in musician.chord_list.iter() {
        //     for note in chord.get_notes() {
        //         let n = note % 12;
        //         result[n as usize].push(chord.clone());
        //     }
        // }
        //
        //musician.chord_table = result;

        assert!(musician.chord_table[CSHARP as usize].len() == 0, "C# had some chords in it");
        assert!(musician.chord_table[D as usize].len() == 0, "D had some notes in it");
        assert!(musician.chord_table[E as usize].len() == 0, "E had some notes in it");
        assert!(musician.chord_table[FSHARP as usize].len() == 0, "F# had some notes in it");
        assert!(musician.chord_table[GSHARP as usize].len() == 0, "G# had some notes in it");
        assert!(musician.chord_table[A as usize].len() == 0, "A had some notes in it");
        assert!(musician.chord_table[B as usize].len() == 0, "B had some notes in it");

        assert!(musician.chord_table[C as usize].len() != 0, "C was empty");
        assert!(musician.chord_table[DSHARP as usize].len() != 0, "D# was empty");
        assert!(musician.chord_table[F as usize].len() != 0, "F was empty");
        assert!(musician.chord_table[G as usize].len() != 0, "G was empty");
        assert!(musician.chord_table[ASHARP as usize].len() != 0, "A# was empty");
    }

    #[test]
    fn prune_in_fsharp_min() {
        let mut musician = Music::smoke_hash(
            Default::default(),
            "F#min",
            &HashSet::new(),
            "default",
            "disabled"
        ).unwrap();

        prune_chords(&mut musician.chord_table, &mut musician.chord_list, "pentatonic", parse_key("F#min"));

        assert!(musician.chord_table[G as usize].len() == 0, "G had some chords in it");
        assert!(musician.chord_table[GSHARP as usize].len() == 0, "G# had some notes in it");
        assert!(musician.chord_table[ASHARP as usize].len() == 0, "A# had some notes in it");
        assert!(musician.chord_table[C as usize].len() == 0, "C had some notes in it");
        assert!(musician.chord_table[D as usize].len() == 0, "D had some notes in it");
        assert!(musician.chord_table[DSHARP as usize].len() == 0, "D# had some notes in it");
        assert!(musician.chord_table[F as usize].len() == 0, "F had some notes in it");

        assert!(musician.chord_table[FSHARP as usize].len() != 0, "F# was empty");
        assert!(musician.chord_table[A as usize].len() != 0, "A was empty");
        assert!(musician.chord_table[B as usize].len() != 0, "B was empty");
        assert!(musician.chord_table[CSHARP as usize].len() != 0, "C# was empty");
        assert!(musician.chord_table[E as usize].len() != 0, "E was empty");
    }
}