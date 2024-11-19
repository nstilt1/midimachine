use std::collections::HashSet;

use super::{chord::Chord, chord_type::ChordType, utils::{parse_key, HashSetMath}};

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

    chord_list.iter_mut().for_each(|c| c.key = key);

    let (mut chord_set, mut chord_table_sets) = expand_chords(chord_list);

    // prune the chords
    let bad_notes: Vec<usize> = HashSet::from_iter(0..12)
        .difference(&good_notes_set)
        .cloned()
        .collect();

    let mut bad_chords: HashSet<Chord> = HashSet::with_capacity(chord_set.capacity());

    for bad_note in bad_notes {
        bad_chords.add_assign(&chord_table_sets[(bad_note + key as usize) % 12]);
        //chord_table[bad_note] = Vec::new();
        //chord_table_sets[bad_note] = HashSet::new();
    }

    chord_table_sets.iter_mut().zip(chord_table.iter_mut()).for_each(|(chord_table_set, chord_table_vec)| {
        // version 1
        // let chords: HashSet<&Chord> = HashSet::from_iter(chord_table_vec.iter());
        // let subtracted: Vec<Chord> = chords.sub(&bad_chords).to_vec();
        // chord_table_vec = subtracted;

        // version 2 - does not work for some reason
        chord_table_set.sub_assign(&bad_chords);
        *chord_table_vec = chord_table_set.to_vec();
    });

    *chord_list = chord_set.sub(&bad_chords).to_vec();
}

/// Turns chords with optional notes into new chords.
fn expand_chords(chord_list: &mut Vec<Chord>) -> (HashSet<Chord>, Vec<HashSet<Chord>>) {
    let mut chord_set: HashSet<Chord> = HashSet::with_capacity(chord_list.capacity());
    for chord in chord_list.iter() {
        let mut base_chord_type = chord.chord_type.clone();
        let root = chord.root;
        let optional_notes = base_chord_type.optional_notes.clone();
        base_chord_type.optional_notes = Vec::new();
        let base_chord = Chord::new(chord.root, &base_chord_type);
        chord_set.insert(base_chord.clone());

        // add more chords with different combinations of notes to the sets
        // `cumulated_notes` is used to add chords with 1, 2, ..., n optional notes
        // to the sets
        // `notes` is used to add chords with just one of the optional notes
        let mut cumulated_notes = base_chord_type.note_intervals.clone();
        for note in optional_notes.iter() {
            let mut notes = base_chord_type.note_intervals.clone();
            cumulated_notes.push(*note);
            notes.push(*note);
            let new_chord_type = ChordType::new(&base_chord_type.name, &cumulated_notes, &[root], None);
            let new_chord_type_2 = ChordType::new(&base_chord_type.name, &notes, &[root], None);
            let new_chord = Chord::new(chord.root, &new_chord_type);
            let new_chord_2 = Chord::new(chord.root, &new_chord_type_2);
            chord_set.insert(new_chord.to_owned());
            chord_set.insert(new_chord_2.to_owned());
        }
        // accumulate chords with optional notes added in reverse order
        if optional_notes.len() > 2 {
            let mut cumulated_notes = base_chord_type.note_intervals.clone();
            for note in optional_notes.iter().rev() {
                cumulated_notes.push(*note);
                let new_chord_type = ChordType::new(&base_chord_type.name, &cumulated_notes, &[root], None);
                let new_chord = Chord::new(chord.root, &new_chord_type);
                chord_set.insert(new_chord.to_owned());
            }
        }
    }

    let mut chord_table_sets: Vec<HashSet<Chord>> = vec![HashSet::new(); 12];
    // insert chord_set into chord_table_sets
    chord_set.iter().for_each(|chord| {
        chord.get_notes()
            .iter()
            .for_each(|n| {
                chord_table_sets[*n as usize % 12].insert(chord.clone());
            });
    });

    (chord_set, chord_table_sets)
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
    *chord_list = chord_set.sub(&bad_chords).to_vec()
}

#[cfg(test)]
mod tests {
    use crate::music_modules_v2::{music::{A, ASHARP, B, C, CSHARP, D, DSHARP, E, F, FSHARP, G, GSHARP}, Music};

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
            "pentatonic"
        ).unwrap();

        //prune_chords(&mut musician.chord_table, &mut musician.chord_list, "pentatonic", parse_key("Cmin"));

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