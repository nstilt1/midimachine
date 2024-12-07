//! Methods related to pruning chords from the chord table and chord list.

use std::{collections::HashSet, hash::{DefaultHasher, Hash, Hasher}};

use super::{chord::{expand_chords, Chord}, utils::sets::SetOpsCollection, music::notes::*};

const fn slice_i16_to_u16<const N: usize>(slice: [i16; N]) -> u16 {
    const fn helper(slice: &[i16], acc: u16) -> u16 {
        if slice.is_empty() {
            acc
        } else {
            helper(&slice[1..], acc | (1 << slice[0]))
        }
    }
    helper(&slice, 0)
}

const fn slice_u8_to_u16<const N: usize>(slice: [u8; N]) -> u16 {
    const fn helper(slice: &[u8], acc: u16) -> u16 {
        if slice.is_empty() {
            acc
        } else {
            helper(&slice[1..], acc | (1 << slice[0]))
        }
    }
    helper(&slice, 0)
}

const NATURAL: [i16; 7] = [0, 2, 3, 5, 7, 8, 10];
const MELODIC: [i16; 7] = [0, 2, 3, 5, 7, 9, 11];
const HARMONIC: [i16; 7] = [0, 2, 3, 5, 7, 8, 11];
const PENTATONIC: [i16; 5] = [0, 3, 5, 7, 10];
const ROMANIAN: [i16; 7] = [0, 2, 3, 6, 7, 9, 10];
const HUNGARIAN: [i16; 7] = [0, 2, 3, 6, 7, 8, 11];
const ALL_NOTES: [i16; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
const HALF_WHOLE: [u8; 8] = [C, CSHARP, DSHARP, E, FSHARP, G, A, ASHARP];
const WHOLE_HALF: [u8; 8] = [C, D, DSHARP, F, FSHARP, GSHARP, A, B];

/// Returns the good notes set and the bad notes set for a given scale in the 
/// key of C minor.
/// 
/// Returns `Option<(good_notes_set, bad_notes_set)>`
fn get_good_notes_set(scale: &str) -> Option<(HashSet<i16>, Vec<usize>)> {
    let good_notes_set: HashSet<i16> = match scale {
        "disabled" => return None,
        "natural" => HashSet::from(NATURAL),
        "melodic" => HashSet::from(MELODIC),
        "harmonic" => HashSet::from(HARMONIC),
        "pentatonic" => HashSet::from(PENTATONIC),
        "romanian" => HashSet::from(ROMANIAN),
        "hungarian" => HashSet::from(HUNGARIAN),
        // "all_notes" restructures `chord_table` and `chord_list` with the
        // optional notes vecs getting converted to new chords
        "all_notes" => HashSet::from(ALL_NOTES),
        "half_whole" => HashSet::from_iter(HALF_WHOLE.iter().map(|note| *note as i16)),
        "whole_half" => HashSet::from_iter(WHOLE_HALF.iter().map(|note| *note as i16)),
        _ => return None
    };
    let bad_notes = HashSet::from_iter(0..12).difference(&good_notes_set).map(|v| *v as usize).collect();
    return Some((good_notes_set, bad_notes))
}

/// Returns the good notes u16 and the bad notes u16 for a given scale in the 
/// key of C minor
/// 
/// Returns `Option<(good notes, bad notes)>`
fn get_good_notes_u16(scale: &str) -> Option<(u16, u16)> {
    let good_notes_u16: u16 = match scale {
        "disabled" => return None,
        "natural" => slice_i16_to_u16(NATURAL),
        "melodic" => slice_i16_to_u16(MELODIC),
        "harmonic" => slice_i16_to_u16(HARMONIC),
        "pentatonic" => slice_i16_to_u16(PENTATONIC),
        "romanian" => slice_i16_to_u16(ROMANIAN),
        "hungarian" => slice_i16_to_u16(HUNGARIAN),
        "all_notes" => slice_i16_to_u16(ALL_NOTES),
        "half_whole" => slice_u8_to_u16(HALF_WHOLE),
        "whole_half" => slice_u8_to_u16(WHOLE_HALF),
        _ => return None
    };
    let bad_notes_u16: u16 = 0b111111111111 ^ good_notes_u16;
    Some((good_notes_u16, bad_notes_u16))
}

pub fn prune_chords_u16(
    chord_table: &mut Vec<Vec<Chord>>,
    chord_list: &mut Vec<Chord>,
    scale: &str,
    is_reproducible: bool
) {
    let (good_notes, _bad_notes) = match get_good_notes_u16(scale) {
        Some(v) => v,
        None => { return; }
    };

    let mut result: Vec<Chord> = Vec::with_capacity(chord_list.len());
    for chord in chord_list.iter() {
        let notes = chord.get_notes_u16();
        if notes & good_notes == notes {
            result.push(chord.clone());
        }
    }

    if is_reproducible {
        result.sort_by_key(|chord| {
            let mut hasher = DefaultHasher::new();
            chord.hash(&mut hasher);
            hasher.finish()
        });
        for col in chord_table.iter_mut() {
            col.sort_by_key(|chord| {
                let mut hasher = DefaultHasher::new();
                chord.hash(&mut hasher);
                hasher.finish()
            });
        }
    }

    *chord_list = result;
}

/// Removes chords that have notes outside of the chosen scale
/// The base key is C Minor, so for the natural minor scale, we will remove:
/// * C# - 1
/// * E  - 4
/// * F# - 6
/// * A  - 9
/// * B - 11
pub fn prune_chords(
    chord_table: &mut Vec<Vec<Chord>>, 
    chord_list: &mut Vec<Chord>, 
    scale: &str, 
    is_reproducible: bool
) {
    let (_good_notes_set, bad_notes) = match get_good_notes_set(scale) {
        Some(v) => v,
        None => { return; }
    };
    
    // turn chords with optional notes into new chords
    let (chord_set, mut chord_table_sets) = expand_chords(chord_list);

    let mut bad_chords: HashSet<Chord> = HashSet::new();

    for mut bad_note in bad_notes {
        bad_note = bad_note % 12;
        bad_chords = bad_chords.union(&chord_table_sets[bad_note]).cloned().collect();
        chord_table[bad_note] = Vec::new();
        chord_table_sets[bad_note] = HashSet::new();
    }

    for note in 0..11 {
        chord_table[note] = chord_table_sets[note]
            .difference(&bad_chords)
            .to_vec()
    }

    *chord_list = chord_set.difference(&bad_chords).cloned().collect();

    // sort the chords by their hash to ensure that they will (nearly) always be
    // in the same order for every run of this function
    if is_reproducible {
        chord_list.sort_by_key(|chord| {
            let mut hasher = DefaultHasher::new();
            chord.hash(&mut hasher);
            hasher.finish()
        });
        for col in chord_table.iter_mut() {
            col.sort_by_key(|chord| {
                let mut hasher = DefaultHasher::new();
                chord.hash(&mut hasher);
                hasher.finish()
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::music_modules_v2::{utils::{parse_key, sets::ToSet}, Music};

    use super::*;

    macro_rules! print_chord_counts {
        ($musician:expr, $test_name:expr) => {
            println!("Test: {}", $test_name);
            println!("Chord table column lengths:");
            for (col_index, col) in $musician.chord_table.iter().enumerate() {
                println!("{}: {}", col_index, col.len());
            }
        };
    }

    #[test]
    fn get_bad_notes() {
        let good_notes = HashSet::from([0, 1, 2, 3, 10]);
        let mut bad_notes: Vec<usize> = HashSet::from_iter(0..12)
            .difference(&good_notes)
            .to_vec();
        bad_notes.sort();
        assert_eq!(bad_notes, [4, 5, 6, 7, 8, 9, 11]);
    }

    #[test]
    fn num_chords_equal_in_list_and_table() {
        let musician = Music::smoke_hash(
            Default::default(),
            "Cmin",
            &HashSet::new(),
            "default",
            "pentatonic",
            true,
            false
        ).unwrap();

        let mut chords = musician.chord_table[0].to_set();
        musician.chord_table.iter().skip(1).for_each(|vec| {
            chords = chords.union(&vec.to_set()).to_set()
        });
        println!("Num chords found in num_chords_equal_in_list_and_table: {}", chords.len());
        assert_eq!(chords.len(), musician.chord_list.len());
    }

    #[test]
    fn pruning_c_minor() {
        let musician = Music::smoke_hash(
            Default::default(),
            "Cmin",
            &HashSet::new(),
            "default",
            "pentatonic",
            true,
            false
        ).unwrap();

        assert!(musician.chord_table[CSHARP as usize].len() == 0, "C# had some chords in it");
        assert!(musician.chord_table[D as usize].len() == 0, "D had some notes in it");
        assert!(musician.chord_table[E as usize].len() == 0, "E had some notes in it");
        assert!(musician.chord_table[FSHARP as usize].len() == 0, "F# had some notes in it");
        assert!(musician.chord_table[GSHARP as usize].len() == 0, "G# had some notes in it");
        assert!(musician.chord_table[A as usize].len() == 0, "A had some notes in it");
        assert!(musician.chord_table[B as usize].len() == 0, "B had some notes in it");

        assert!(musician.chord_table[C as usize].len() != 0, "C was empty");
        assert!(musician.chord_table[DSHARP as usize].len() != 0, "D# was empty");
        //assert!(musician.chord_table[F as usize].len() != 0, "F was empty");
        assert!(musician.chord_table[G as usize].len() != 0, "G was empty");
        assert!(musician.chord_table[ASHARP as usize].len() != 0, "A# was empty");

        // check for chords that shouldn't be there
        let (_good_notes, bad_notes) = get_good_notes_set("pentatonic").unwrap();
        let bad_notes_set: HashSet<i16> = bad_notes.iter().map(|n| *n as i16).collect();
        let mut bad_chords: HashSet<Chord> = HashSet::new();
        for chords in musician.chord_table.iter() {
            for chord in chords.iter() {
                let notes = chord.get_notes_set();
                let bad_notes_amount = bad_notes_set.intersection(&notes).count();
                if bad_notes_amount > 0 {
                    bad_chords.insert(chord.clone());
                }
            }
        }
        println!("Num chords: {}", musician.chord_list.len());
        assert_eq!(bad_chords.len(), 0);
    }

    #[test]
    fn prune_in_fsharp_min() {
        let mut musician = Music::smoke_hash_all_pruning_chords("Cmin", "pentatonic");
        print_chord_counts!(musician, "prune_in_fsharp_min");

        musician.rotate_chords("F#min");

        print_chord_counts!(musician, "prune_in_fsharp_min");

        let (_good_notes, bad_notes) = get_good_notes_set("pentatonic").unwrap();

        let bad_notes_set: HashSet<i16> = HashSet::from_iter(bad_notes.iter().map(|n| (*n as i16 + parse_key("F#min")) % 12));

        // bad notes
        assert!(musician.chord_table[G as usize].len() == 0, "G had some chords in it");
        assert!(musician.chord_table[GSHARP as usize].len() == 0, "G# had some notes in it");
        assert!(musician.chord_table[ASHARP as usize].len() == 0, "A# had some notes in it");
        assert!(musician.chord_table[C as usize].len() == 0, "C had some notes in it");
        assert!(musician.chord_table[D as usize].len() == 0, "D had some notes in it");
        assert!(musician.chord_table[DSHARP as usize].len() == 0, "D# had some notes in it");
        assert!(musician.chord_table[F as usize].len() == 0, "F had some notes in it");

        // good notes
        assert!(musician.chord_table[FSHARP as usize].len() != 0, "F# was empty");
        assert!(musician.chord_table[A as usize].len() != 0, "A was empty");
        assert!(musician.chord_table[B as usize].len() != 0, "B was empty");
        assert!(musician.chord_table[CSHARP as usize].len() != 0, "C# was empty");
        assert!(musician.chord_table[E as usize].len() != 0, "E was empty");

        let mut chords: HashSet<Chord> = HashSet::new();
        println!("F#min col lengths:");
        musician.chord_table.iter().enumerate().for_each(|(col_index, vec)| {
            chords = chords.union(&vec.to_set()).to_set();
            println!("{}: {}", col_index, vec.len());
        });

        println!("F#min chord count: {}", chords.len());
        assert_eq!(chords.len(), musician.chord_list.len());
        
        let mut bad_chords: HashSet<Chord> = HashSet::new();
        for chords in musician.chord_table.iter() {
            for chord in chords.iter() {
                let notes = chord.get_notes_set();
                let num_bad_notes = notes.intersection(&bad_notes_set).count();
                if num_bad_notes > 0 {
                    bad_chords.insert(chord.clone());
                }
            }
        }
        assert_eq!(bad_chords.len(), 0);
    }

    #[test]
    fn prune_c_minor_natural() {
        let musician = Music::smoke_hash(
            Default::default(),
            "Cmin",
            &HashSet::new(),
            "default",
            "natural",
            true,
            false
        ).unwrap();

        assert!(musician.chord_table[CSHARP as usize].len() == 0, "C# had some chords in it");
        assert!(musician.chord_table[E as usize].len() == 0, "E had some chords in it");
        assert!(musician.chord_table[FSHARP as usize].len() == 0, "F# had some chords in it");
        assert!(musician.chord_table[A as usize].len() == 0, "A had some chords in it");
        assert!(musician.chord_table[B as usize].len() == 0, "B had some chords in it");
        

        assert!(musician.chord_table[C as usize].len() != 0, "C was empty");
        assert!(musician.chord_table[D as usize].len() != 0, "D was empty");
        assert!(musician.chord_table[DSHARP as usize].len() != 0, "D# was empty");
        assert!(musician.chord_table[F as usize].len() != 0, "F was empty");
        assert!(musician.chord_table[G as usize].len() != 0, "G was empty");
        assert!(musician.chord_table[GSHARP as usize].len() != 0, "G# was empty");
        assert!(musician.chord_table[ASHARP as usize].len() != 0, "A# was empty");

        // check for chords that shouldn't be there
        let (_good_notes, bad_notes) = get_good_notes_set("natural").unwrap();
        let bad_notes_set: HashSet<i16> = bad_notes.iter().map(|n| *n as i16).collect();
        let mut bad_chords: HashSet<Chord> = HashSet::new();
        for chords in musician.chord_table.iter() {
            for chord in chords.iter() {
                let notes = chord.get_notes_set();
                let bad_notes_amount = bad_notes_set.intersection(&notes).count();
                if bad_notes_amount > 0 {
                    bad_chords.insert(chord.clone());
                }
            }
        }
        println!("Num chords: {}", musician.chord_list.len());
        assert_eq!(bad_chords.len(), 0);
    }

    #[test]
    fn slice_to_u16() {
        assert_eq!(slice_i16_to_u16([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]), 0b111111111111);
        assert_eq!(slice_i16_to_u16([0, 2, 3, 5]), 0b000000101101)
    }

    #[test]
    fn equivalence_of_prune_functions() {
        let mut musician = Music::smoke_hash_all_pruning_chords(
            "Cmin", 
            "all_notes"
        );
        let mut musician_2 = musician.clone();
        prune_chords(&mut musician.chord_table, &mut musician.chord_list, "natural", true);

        prune_chords_u16(&mut musician_2.chord_table, &mut musician_2.chord_list, "natural", true);

        assert_eq!(musician.chord_list, musician_2.chord_list)
    }
}