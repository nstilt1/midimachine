use std::collections::HashSet;

use super::{chord::Chord, chord_type::ChordType, utils::parse_key};

/// Removes chords that have notes outside of the chosen scale
/// The base key is C Minor, so for the natural minor scale, we will remove:
/// * C# - 1
/// * E  - 4
/// * F# - 6
/// * A  - 9
/// * B - 11
pub fn prune_chords(notes_of_chords: &mut Vec<Vec<Chord>>, all_chords: &mut Vec<Chord>, scale: &str) {
    let mut all_chords_set: HashSet<Chord> = HashSet::from_iter(all_chords.iter().cloned());

    let good_notes_set: HashSet<usize> = match scale {
        "disabled" => return,
        "natural" => HashSet::from([0, 2, 3, 4, 5, 7, 8, 10]),
        "melodic" => HashSet::from([0, 2, 3, 5, 7, 9, 11]),
        "harmonic" => HashSet::from([0, 2, 3, 5, 7, 8, 11]),
        "pentatonic" => HashSet::from([0, 3, 5, 7, 10]),
        "romanian" => HashSet::from([0, 2, 3, 6, 7, 9, 10]),
        "hungarian" => HashSet::from([0, 2, 3, 6, 7, 8, 11]),
        // "all_notes" restructures `notes_of_chords` and `all_chords` with the
        // optional notes vecs getting converted to new chords
        "all_notes" => HashSet::from([0,1,2,3,4,5,6,7,8,9,10,11]),
        _ => return
    };

    // turn chords with optional notes into new chords
    let mut notes_of_chords_sets: Vec<HashSet<Chord>> = vec![HashSet::new(); 12];
    for (n, chords) in notes_of_chords.iter().enumerate() {
        for chord in chords.iter() {
            let mut base_chord_type = chord.chord_type.clone();
            let optional_notes = base_chord_type.optional_notes;
            let root = chord.root;
            base_chord_type.optional_notes = Vec::new();
            let base_chord = Chord::new(chord.root, &base_chord_type);
            notes_of_chords_sets[n].insert(base_chord.clone());
            all_chords_set.insert(base_chord.clone());

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
                all_chords_set.insert(new_chord.to_owned());
                all_chords_set.insert(new_chord_2.to_owned());
                for c in &[&new_chord, &new_chord_2, &base_chord] {
                    for n_2 in c.get_notes().iter() {
                        let index = (*n_2 as u8) % 12;
                        notes_of_chords_sets[index as usize].insert(c.to_owned().to_owned());
                    }
                }
            }
            // accumulate chords with optional notes added in reverse order
            if optional_notes.len() > 2 {
                let mut cumulated_notes = base_chord_type.note_intervals.clone();
                for note in optional_notes.iter().rev() {
                    cumulated_notes.push(*note);
                    let new_chord_type = ChordType::new(&base_chord_type.name, &cumulated_notes, &[root], None);
                    let new_chord = Chord::new(chord.root, &new_chord_type);
                    all_chords_set.insert(new_chord.to_owned());
                    for n_2 in new_chord.get_notes().iter() {
                        let index = (*n_2 as u8) % 12;
                        notes_of_chords_sets[index as usize].insert(new_chord.to_owned());
                    }
                }
            }
        }
    }

    // prune the chords
    let bad_notes: Vec<usize> = HashSet::from_iter(0..12)
        .difference(&good_notes_set)
        .cloned()
        .collect();

    let mut bad_chords: HashSet<Chord> = HashSet::new();

    for bad_note in bad_notes {
        bad_chords = bad_chords.union(&notes_of_chords_sets[bad_note]).cloned().collect();
        notes_of_chords[bad_note] = Vec::new();
        notes_of_chords_sets[bad_note] = HashSet::new();
    }

    for note in 0..11 {
        // notes_of_chords_sets[note] = notes_of_chords_sets[note]
        //     .difference(&bad_chords)
        //     .cloned()
        //     .collect();
        // notes_of_chords[note] = notes_of_chords_sets[note].iter().cloned().collect();
        let chords: HashSet<Chord> = HashSet::from_iter(notes_of_chords[note].iter().cloned());
        let subtracted: Vec<Chord> = chords.difference(&bad_chords).cloned().collect();
        notes_of_chords[note] = subtracted;
    }

    *all_chords = all_chords_set.difference(&bad_chords).cloned().collect();
}

pub fn translate_and_prune(
    notes_of_chords: &mut Vec<Vec<Chord>>, 
    all_chords: &mut Vec<Chord>, 
    key: &str,
    scale: &str
) {
    let key_int = parse_key(key);
    for chord in all_chords.iter_mut() {
        chord.key = key_int;
    }
    *notes_of_chords = vec![Vec::new(); 12];

    for chord in all_chords.iter() {
        for note in chord.get_notes() {
            let n = (note + key_int) % 12;
            notes_of_chords[n as usize].push(chord.clone());
        }
    }

    let mut notes_of_chords_set: Vec<HashSet<Chord>> = vec![HashSet::with_capacity(16); 12];
    for (i, col) in notes_of_chords.iter().enumerate() {
        notes_of_chords_set[i] = HashSet::from_iter(col.iter().cloned());
    }

    let good_notes_set: HashSet<usize> = match scale {
        "disabled" => return,
        "natural" => HashSet::from([0, 2, 3, 4, 5, 7, 8, 10]),
        "melodic" => HashSet::from([0, 2, 3, 5, 7, 9, 11]),
        "harmonic" => HashSet::from([0, 2, 3, 5, 7, 8, 11]),
        "pentatonic" => HashSet::from([0, 3, 5, 7, 10]),
        "romanian" => HashSet::from([0, 2, 3, 6, 7, 9, 10]),
        "hungarian" => HashSet::from([0, 2, 3, 6, 7, 8, 11]),
        // "all_notes" restructures `notes_of_chords` and `all_chords` with the
        // optional notes vecs getting converted to new chords
        "all_notes" => HashSet::from([0,1,2,3,4,5,6,7,8,9,10,11]),
        _ => return
    };

    let bad_notes: Vec<usize> = HashSet::from_iter(0..12)
        .difference(&good_notes_set)
        .cloned()
        .collect();

    let mut bad_chords: HashSet<Chord> = HashSet::with_capacity(12 * 8);
    for bad_note in bad_notes {
        bad_chords = bad_chords
            .union(&notes_of_chords_set[bad_note])
            .cloned()
            .collect();
        notes_of_chords[bad_note] = Vec::new();
        notes_of_chords_set[bad_note] = HashSet::new();
    }

    for note in 0..11 {
        notes_of_chords_set[note] = notes_of_chords_set[note]
            .difference(&bad_chords)
            .cloned()
            .collect();
    }
    let all_chords_set: HashSet<Chord> = HashSet::from_iter(all_chords.iter().cloned());
    *all_chords = all_chords_set
        .difference(&bad_chords)
        .cloned()
        .collect();
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
    fn pruning() {
        let mut musician = Music::smoke_hash(
            Default::default(),
            "Cmin",
            &HashSet::new(),
            "default",
            "disabled"
        ).unwrap();

        prune_chords(&mut musician.notes_of_chords, &mut musician.all_chords, "pentatonic");

        // for chord in musician.all_chords.iter_mut() {
        //     chord.key = 0;
        // }

        // let mut result: Vec<Vec<Chord>> = vec![Vec::new(); 12];
        // for chord in musician.all_chords.iter() {
        //     for note in chord.get_notes() {
        //         let n = note % 12;
        //         result[n as usize].push(chord.clone());
        //     }
        // }
        //
        //musician.notes_of_chords = result;

        assert!(musician.notes_of_chords[CSHARP as usize].len() == 0, "C# had some chords in it");
        assert!(musician.notes_of_chords[D as usize].len() == 0, "D had some notes in it");
        assert!(musician.notes_of_chords[E as usize].len() == 0, "E had some notes in it");
        assert!(musician.notes_of_chords[FSHARP as usize].len() == 0, "F# had some notes in it");
        assert!(musician.notes_of_chords[GSHARP as usize].len() == 0, "G# had some notes in it");
        assert!(musician.notes_of_chords[A as usize].len() == 0, "A had some notes in it");
        assert!(musician.notes_of_chords[B as usize].len() == 0, "B had some notes in it");

        assert!(musician.notes_of_chords[C as usize].len() != 0, "C was empty");
        assert!(musician.notes_of_chords[DSHARP as usize].len() != 0, "D# was empty");
        assert!(musician.notes_of_chords[F as usize].len() != 0, "F was empty");
        assert!(musician.notes_of_chords[G as usize].len() != 0, "G was empty");
        assert!(musician.notes_of_chords[ASHARP as usize].len() != 0, "A# was empty");
    }
}