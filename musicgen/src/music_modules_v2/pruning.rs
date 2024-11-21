use std::collections::HashSet;

use super::{chord::Chord, chord_type::ChordType, utils::{parse_key, CustomIterators, HashSetMath}};

fn get_notes_set_for_scale(scale: &str, key: i16) -> Option<HashSet<i16>> {
    let good_notes_set: HashSet<i16> = match scale {
        "disabled" => return None,
        "natural" => HashSet::from_iter([0i16, 2, 3, 4, 5, 7, 8, 10].transpose(key),),
        "melodic" => HashSet::from_iter([0i16, 2, 3, 5, 7, 9, 11].transpose(key),),
        "harmonic" => HashSet::from_iter([0i16, 2, 3, 5, 7, 8, 11].transpose(key)),
        "pentatonic" => HashSet::from_iter([0i16, 3, 5, 7, 10].transpose(key)),
        "romanian" => HashSet::from_iter([0i16, 2, 3, 6, 7, 9, 10].transpose(key)),
        "hungarian" => HashSet::from_iter([0i16, 2, 3, 6, 7, 8, 11].transpose(key)),
        // "all_notes" restructures `chord_table` and `chord_list` with the
        // optional notes vecs getting converted to new chords
        "all_notes" => HashSet::from([0,1,2,3,4,5,6,7,8,9,10,11]),
        _ => return None
    };
    Some(good_notes_set)
}
/// Removes chords that have notes outside of the chosen scale
/// The base key is C Minor, so for the natural minor scale, we will remove:
/// * C# - 1
/// * E  - 4
/// * F# - 6
/// * A  - 9
/// * B - 11
pub fn prune_chords(chord_table: &mut Vec<Vec<Chord>>, chord_list: &mut Vec<Chord>, scale: &str, key: i16) {
    let good_notes_set = match get_notes_set_for_scale(scale, key) {
        Some(v) => v,
        None => { return; }
    };

    chord_list.iter_mut().for_each(|c| c.key = key);

    // prune the chords
    let bad_notes_set: HashSet<i16> = HashSet::from_iter(0..12)
        .difference(&good_notes_set)
        .cloned()
        .collect();

    let bad_notes = bad_notes_set.to_vec();

    let (mut chord_set, mut chord_table_sets) = expand_chords(chord_list, bad_notes_set);

    /*
    let mut bad_chords: HashSet<Chord> = HashSet::with_capacity(chord_set.capacity());

    for bad_note in bad_notes {
        bad_chords.add_assign(&chord_table_sets[(bad_note + key) as usize % 12]);
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
    */
    let mut chord_set: HashSet<Chord> = HashSet::new();
    chord_table_sets.iter().zip(chord_table.iter_mut()).for_each(|(chord_table_set, chord_table_vec)| {
        *chord_table_vec = chord_table_set.to_vec();
        chord_set.add_assign(chord_table_set);
    });

    *chord_list = chord_set.to_vec();

    //*chord_list = chord_set.sub(&bad_chords).to_vec();
}

/// Turns chords with optional notes into new chords.
fn expand_chords(chord_list: &mut Vec<Chord>, bad_notes_set: HashSet<i16>) -> (HashSet<Chord>, Vec<HashSet<Chord>>) {
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
        let intersection_with_bad_notes_set = chord.notes_set().intersect(&bad_notes_set);
        if intersection_with_bad_notes_set.len() == 0 {
            chord.get_notes()
                .iter()
                .for_each(|n| {
                    chord_table_sets[*n as usize % 12].insert(chord.clone());
                });
        }
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
    use crate::music_modules_v2::{chord_type::{all_roots_chord_types, default_chord_types, expand_chord_types}, music::notes::*, Music};

    use super::*;

    #[test]
    fn get_good_notes() {
        let good_notes_set = get_notes_set_for_scale("pentatonic", 1).unwrap();
        assert!(good_notes_set.eq(&HashSet::from([1, 4, 6, 8, 11])));
        let good_notes_set = get_notes_set_for_scale("pentatonic", 7).unwrap();
        assert!(good_notes_set.eq(&HashSet::from_iter([1i16, 4, 6, 8, 11].transpose(6))));
    }

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
    fn get_bad_notes_set() {
        let key = 3;
        let good_notes_set: HashSet<i16> = HashSet::from_iter([0i16, 3, 5, 7, 10].transpose(key));

        assert_eq!(good_notes_set, HashSet::from([0i16 + key, 3+key, 5+key, 7+key, (10+key)%12]));

        let bad_notes_set: HashSet<i16> = HashSet::from_iter(0..12).sub(&good_notes_set);
        assert_eq!(bad_notes_set.intersect(&good_notes_set).len(), 0);
    }

    #[test]
    fn expand_chords_test() {
        let good_notes_set = get_notes_set_for_scale("pentatonic", 0).unwrap();
        let bad_notes_set = HashSet::from_iter(0..12).sub(&good_notes_set);

        let chord_types = all_roots_chord_types();
        //let chord_types = all_roots_chord_types();
        let (mut chord_list, mut chord_table) = expand_chord_types(&chord_types, 0);

        let mut chord_set: HashSet<Chord> = HashSet::with_capacity(chord_list.capacity());

        let (mut chord_set, mut chord_table_set) = expand_chords(&mut chord_list, bad_notes_set);

        let c = &chord_table_set;
        println!("chord_table column lengths:");
        for (col_index, col) in c.iter().enumerate() {
            println!("{}: {}", col_index, col.len());
        }
        assert!(chord_table_set[1].len() == 0);
        assert!(chord_table_set[2].len() == 0);
        assert!(chord_table_set[4].len() == 0);
        assert!(chord_table_set[6].len() == 0);
        assert!(chord_table_set[8].len() == 0);
        assert!(chord_table_set[9].len() == 0);
        assert!(chord_table_set[11].len() == 0);


        assert!(chord_table_set[0].len() != 0);
        assert!(chord_table_set[3].len() != 0);
        assert!(chord_table_set[5].len() != 0);
        assert!(chord_table_set[7].len() != 0);
        assert!(chord_table_set[10].len() != 0);
    }

    #[test]
    fn pruning_c_minor() {
        let mut musician = Music::smoke_hash(
            Default::default(),
            "Cmin",
            &HashSet::new(),
            "default",
            "natural"
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

        assert!(musician.chord_table[1].len() == 0, "1 had some chords in it");
        assert!(musician.chord_table[6].len() == 0, "6 had some notes in it");
        assert!(musician.chord_table[9].len() == 0, "9 had some notes in it");
        assert!(musician.chord_table[11].len() == 0, "11 had some notes in it");

        assert!(musician.chord_table[0].len() != 0, "0 was empty");
        assert!(musician.chord_table[2].len() != 0, "2 was empty");
        assert!(musician.chord_table[3].len() != 0, "3 was empty");
        assert!(musician.chord_table[4].len() != 0, "4 was empty");
        assert!(musician.chord_table[5].len() != 0, "5 was empty");
        assert!(musician.chord_table[7].len() != 0, "7 was empty");        assert!(musician.chord_table[8].len() != 0, "8 was empty");
        assert!(musician.chord_table[10].len() != 0, "10 was empty");


    }

    #[test]
    fn prune_in_fsharp_min() {
        let good_notes = get_notes_set_for_scale("natural", parse_key("F#min")).unwrap();

        let mut sorted_good_notes = good_notes.to_vec();
        sorted_good_notes.sort();
        println!("Good notes: {:?}", sorted_good_notes);

        let bad_notes = HashSet::from_iter(0..12).sub(&good_notes);
        let mut sorted_bad_notes = bad_notes.to_vec();
        sorted_bad_notes.sort();
        println!("Bad notes: {:?}", sorted_bad_notes);

        let chord_types = all_roots_chord_types();
        let (mut chord_list, mut chord_table) = expand_chord_types(&chord_types, parse_key("F#min"));
        let (mut chord_set, mut chord_table_set) = expand_chords(&mut chord_list, bad_notes.clone());

        //prune_chords(&mut musician.chord_table, &mut musician.chord_list, "pentatonic", parse_key("F#min"));
        

        let c = &chord_table_set;
        println!("chord_table column lengths:");
        for (col_index, col) in c.iter().enumerate() {
            println!("{}: {}", col_index, col.len());
        }

        for good_note in good_notes {
            let col_len = chord_table_set[good_note as usize].len();
            assert!(col_len != 0, "{} chords had note {} when there were supposed to be some", col_len, good_note);
        }
        for bad_note in bad_notes.iter() {
            let col_len = chord_table_set[*bad_note as usize].len();
            assert!(col_len == 0, "{} chords had note {} when there were supposed to be 0", col_len, bad_note);
        }

        for chords in chord_table_set.iter() {
            for chord in chords.iter() {
                for note in chord.get_notes() {
                    assert!(!bad_notes.contains(&note));
                }
            }
        }
    }
}