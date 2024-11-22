use std::collections::{HashSet, VecDeque};

use midly::TrackEvent;
use sha2::Sha256;

use crate::Error;

use super::chord_type::default_chord_types;

use super::pruning::prune_chords;
use super::utils::{get_max_note_length_index, parse_key};
use super::{chord_type::ChordType, chord::Chord, utils::MathMagician, midi::MidiFile};

const NOTE_LENGTHS: [f64; 8] = [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];

macro_rules! define_consts {
    ($(($const_name:ident, $value:literal)),*) => {
        pub mod notes {
            $(
                #[allow(unused)]
                pub const $const_name: u8 = $value;
            )*
        }
    };
}

define_consts!(
    (C, 0),
    (CSHARP, 1),
    (D, 2),
    (DSHARP, 3),
    (E, 4),
    (F, 5),
    (FSHARP, 6),
    (G, 7),
    (GSHARP, 8),
    (A, 9),
    (ASHARP, 10),
    (B, 11)
);

use notes::*;

#[derive(Debug)]
pub struct Music {
    math_magician: MathMagician,
    midi_file: MidiFile,
    pub key: i16,
    _chord_types: Vec<ChordType>,
    pub chord_table: Vec<Vec<Chord>>,
    pub chord_list: Vec<Chord>
}

macro_rules! enforce_unique_chord {
    (
        $music_obj:expr, 
        $chord_picking_method:ident, 
        $previous_n_chords:expr,
        $chord:expr
    ) => {
        if $previous_n_chords.capacity() > 0 {
            let mut i = 0;
            while $previous_n_chords.contains(&$chord) {
                $chord = $music_obj.$chord_picking_method();
                i += 1;
                if i > 420 {
                    // potential infinite loop
                    break;
                }
            }
            if $previous_n_chords.len() == $previous_n_chords.capacity() {
                $previous_n_chords.pop_front();
            }
            $previous_n_chords.push_back($chord.clone());
        }
    };
}

/// This macro picks chords to play and places them in the track.
/// 
/// There are two ways that chords can be picked:
/// 
/// * `original` - chords are randomly picked from a 2D array. The rows of the 
/// array are notes, and the columns are lists of chords that contain the 
/// row's note. Chords with more notes are somewhat more likely to be picked.
/// * `1D` - chords are randomly picked from a 1D array. Each chord has a 
/// roughly equal probability of getting picked.
/// 
/// This could have been written prettier by checking the user's input inside 
/// the for-loops, but then every iteration of the for-loop would have at 
/// least one extra comparison. And potentially more than one if more modes are 
/// added later.
macro_rules! pick_chord_placement_method {
    (
        $music_obj:expr, 
        $user_selected_type:expr, 
        $num_chords:expr, 
        $should_use_same_chords:expr, 
        $chord_picking_method:expr, 
        $minimum_number_of_unique_chords:expr,
        $(($chord_placement_str:expr, $placement_method:ident)),*
    ) => {
        let mut previous_n_chords: VecDeque<Chord> = VecDeque::with_capacity($minimum_number_of_unique_chords as usize);
        if $should_use_same_chords {
            let mut chords = vec![Chord::default(); $num_chords];

            if $chord_picking_method == "original" {
                for chord in chords.iter_mut() {
                    *chord = $music_obj.pick_chord();
                    enforce_unique_chord!($music_obj, pick_chord, previous_n_chords, *chord);
                }
            } else if $chord_picking_method == "1D" {
                for chord in chords.iter_mut() {
                    *chord = $music_obj.pick_chord_1d();
                    enforce_unique_chord!($music_obj, pick_chord_1d, previous_n_chords, *chord);
                }
            }

            match $user_selected_type {
                $(
                    $chord_placement_str => {
                        for (i, chord) in chords.iter().enumerate() {
                            $music_obj.$placement_method(&chord, 4, (i as u32 * 4).into());
                        }
                    },
                )*
                _ => { (); }
            }
        } else {
            match $user_selected_type {
                $(
                    $chord_placement_str => {
                        if $chord_picking_method == "original" {
                            for i in 0..$num_chords {
                                let mut chord = $music_obj.pick_chord();
                                enforce_unique_chord!($music_obj, pick_chord, previous_n_chords, chord);
                                $music_obj.$placement_method(&chord, 4, (i as u32 * 4).into());
                            }
                        } else if $chord_picking_method == "1D" {
                            for i in 0..$num_chords {
                                let mut chord = $music_obj.pick_chord_1d();
                                enforce_unique_chord!($music_obj, pick_chord, previous_n_chords, chord);
                                $music_obj.$placement_method(&chord, 4, (i as u32 * 4).into());
                            }
                        }
                    }
                )*
                _ => { (); }
            }
        }
    };
}

macro_rules! add_chord_types {
    ($vec:expr, $selected_types:expr, $(($chord_type_str:expr, $chord_type_obj:expr)),*) => {
        $(
            if $selected_types.contains(&$chord_type_str.to_string()) {
                $vec.push($chord_type_obj)
            }
        )*
    };
}

pub const KEYS: [&str; 12] = [
    "C",
    "C#",
    "D",
    "D#",
    "E",
    "F",
    "F#",
    "G",
    "G#",
    "A",
    "A#",
    "B"
];

impl Music {
    pub fn smoke_hash(
        hash: sha2::digest::Output<Sha256>, 
        chosen_key: &str, 
        chord_selections: &HashSet<String>, 
        chord_type_group: &str,
        scale: &str
    ) -> Result<Music, Error> {
        let mut stash = [0u8; 32];
        stash.copy_from_slice(&hash);
        let mut math_magician = MathMagician::share_hash(stash);

        // initialize key with RNG first so that the output remains the same as 
        // when the key is randomly chosen
        let mut key = math_magician.pick_note();
        if chosen_key.ne("random") {
            key = parse_key(chosen_key);
        }

        let chord_types = match chord_type_group {
            "default" => {
                default_chord_types()
            },
            "major and minor" => {
                let major = ChordType::new("major", &[0, 4, 7], &[DSHARP, GSHARP, ASHARP], None);
                let minor = ChordType::new("minor", &[0, 3, 7], &[C, D, F, G], None);
                vec![major, minor]
            },
            "original" => {
                let minor7_og = ChordType::new("minor 7", &[0, 3, 6, 10], &[0, 2, 5, 7], None);
                let augmented_og = ChordType::new("augmented", &[0, 4, 8], &[10], Some(&[12]));
                let major7_og = ChordType::new("major 7", &[0, 4, 7, 11], &[3, 8], None);
                let diminished_og = ChordType::new("diminished", &[0, 3, 6], &[3], None);
                let major6_og = ChordType::new("major 6", &[0, 4, 7, 9], &[10], None);
                let minor9 = ChordType::new("minor 9", &[0, 3, 7, 10, 14], &[G], None);

                vec![minor7_og, minor9, augmented_og, major7_og, diminished_og, major6_og]
            }
            "custom" => {
                let minor7 = ChordType::new("minor 7", &[0, 10, 15, 19], &[C, D, F, FSHARP, ASHARP], None);
                let major7 = ChordType::new("major 7", &[0, 11, 16, 19], &[DSHARP, GSHARP], None);
                let diminished = ChordType::new("diminished", &[0, 3, 6], &[DSHARP, FSHARP], None);
                let augmented = ChordType::new("augmented", &[0,4,8], &[D, FSHARP, ASHARP], Some(&[12]));
                //let major6 = ChordType::new(&[0, 4, 7, 9], &[3, 10], None);
                let major6 = ChordType::new("major 6", &[0, 9, 16, 19], &[DSHARP, GSHARP, ASHARP], Some(&[23]));

                let minor6 = ChordType::new("minor 6", &[0, 9, 15, 19], &[C, D, F, G], None);
                let major9 = ChordType::new("major 9", &[0, 4, 10, 14], &[C, F, G], None);
                let major7sharp9 = ChordType::new("major 7#9", &[0, 4, 10, 15], &[C, D, G, A], None);
                let major7flat5sharp9 = ChordType::new("major 7b5#9", &[0, 4, 10, 15, 18], &[C, A], None);
                let major9flat5 = ChordType::new("major 9b5", &[0, 4, 10, 15, 17], &[C, A], None);
                let major7flat9 = ChordType::new("major 7b9", &[0, 4, 10, 13], &[C, D], None);

                // extra chord types
                let major = ChordType::new("major", &[0, 4, 7], &[DSHARP, GSHARP, ASHARP], None);
                let minor = ChordType::new("minor", &[0, 3, 7], &[C, D, F, G], None);

                let minor9 = ChordType::new("minor 9", &[0, 3, 7, 10, 14], &[G], None);

                let major13 = ChordType::new("major 13", &[0, 5, 10, 21, 26, 31], &[C], Some(&[0, 5]));
                let dominant9 = ChordType::new("dominant 9", &[0, 4, 9, 14, 18], &[CSHARP], None);
                
                let add9 = ChordType::new("add 9", &[0, 4, 7, 14], &[DSHARP, ASHARP], None);

                let mut chord_types: Vec<ChordType> = Vec::with_capacity(chord_selections.len());
                
                add_chord_types!(
                    chord_types, 
                    chord_selections,
                    ("minor7", minor7),
                    ("major7", major7),
                    ("diminished", diminished),
                    ("augmented", augmented),
                    ("major6", major6),
                    ("minor6", minor6),
                    ("major9", major9),
                    ("major7sharp9", major7sharp9),
                    ("major7flat5sharp9", major7flat5sharp9),
                    ("major9flat5", major9flat5),
                    ("major7flat9", major7flat9),
                    ("major", major),
                    ("minor", minor),
                    ("minor9", minor9),
                    ("major13", major13),
                    ("dominant9", dominant9),
                    ("add9", add9)
                );
                chord_types
            },
            "custom_pruning" => {
                let minor7 = ChordType::all_roots("minor 7", &[0, 10, 15, 19], None);
                let major7 = ChordType::all_roots("major 7", &[0, 11, 16, 19], None);
                let diminished = ChordType::all_roots("diminished", &[0, 3, 6], None);
                let augmented = ChordType::all_roots("augmented", &[0,4,8], Some(&[12]));
                //let major6 = ChordType::new(&[0, 4, 7, 9], &[3, 10], None);
                let major6 = ChordType::all_roots("major 6", &[0, 9, 16, 19], Some(&[23]));

                let minor6 = ChordType::all_roots("minor 6", &[0, 9, 15, 19], None);
                let major9 = ChordType::all_roots("major 9", &[0, 4, 10, 14], None);
                let major7sharp9 = ChordType::all_roots("major 7#9", &[0, 4, 10, 15], None);
                let major7flat5sharp9 = ChordType::all_roots("major 7b5#9", &[0, 4, 10, 15, 18], None);
                let major9flat5 = ChordType::all_roots("major 9b5", &[0, 4, 10, 15, 17], None);
                let major7flat9 = ChordType::all_roots("major 7b9", &[0, 4, 10, 13], None);

                // extra chord types
                let major = ChordType::all_roots("major", &[0, 4, 7], None);
                let minor = ChordType::all_roots("minor", &[0, 3, 7], None);

                let minor9 = ChordType::all_roots("minor 9", &[0, 3, 7, 10, 14], None);

                let major13 = ChordType::all_roots("major 13", &[0, 5, 10, 21, 26, 31], None);
                let dominant9 = ChordType::all_roots("dominant 9", &[0, 4, 9, 14, 18], None);
                
                let add9 = ChordType::all_roots("add 9", &[0, 4, 7, 14], None);

                let mut chord_types: Vec<ChordType> = Vec::with_capacity(chord_selections.len());
                
                add_chord_types!(
                    chord_types, 
                    chord_selections,
                    ("minor7", minor7),
                    ("major7", major7),
                    ("diminished", diminished),
                    ("augmented", augmented),
                    ("major6", major6),
                    ("minor6", minor6),
                    ("major9", major9),
                    ("major7sharp9", major7sharp9),
                    ("major7flat5sharp9", major7flat5sharp9),
                    ("major9flat5", major9flat5),
                    ("major7flat9", major7flat9),
                    ("major", major),
                    ("minor", minor),
                    ("minor9", minor9),
                    ("major13", major13),
                    ("dominant9", dominant9),
                    ("add9", add9)
                );
                chord_types
            },
            _ => vec![ChordType::default()]
        };

        let mut chord_table: Vec<Vec<Chord>> = (0..12).map(|_| Vec::new()).collect();
        let mut chord_list: Vec<Chord> = Vec::new();

        for ct in chord_types.iter() {
            for r in ct.clone().roots {
                let chord = Chord::new(r, &ct);
                chord_list.push(chord.clone());
                for note in chord.get_notes() {
                    chord_table[(note % 12) as usize].push(chord.clone());
                }
            }
        }

        prune_chords(&mut chord_table, &mut chord_list, scale, key);

        return Ok(Music {
            math_magician,
            midi_file: MidiFile::new(),
            key,
            chord_table,
            _chord_types: chord_types,
            chord_list
        })
    }

    /**
     * Makes some music midily
     */
    pub fn make_music(
        &mut self, 
        num_chords: usize, 
        generation_mode: &str, 
        should_use_same_chords: bool, 
        chord_picking_method: &str, 
        minimum_number_of_unique_chords: u32,
    ) -> Result<Vec<TrackEvent>, Error> {
        pick_chord_placement_method!(
            self,
            generation_mode, 
            num_chords,
            should_use_same_chords,
            chord_picking_method,
            minimum_number_of_unique_chords,
            ("melody", original_placement_algorithm),
            ("chords", place_chord_regular),
            ("melody v2", place_chord_bug_v2),
            ("melody v3", place_chord_bug_v3),
            ("intended", place_variable_len_fixed)
        );

        return Ok(self.midi_file.finalize());
    }

    /// Rotates the chords in the chord table.
    pub fn rotate_chords(&mut self, key: &str) {
        let k = parse_key(key);
        self
            .chord_list
            .iter_mut()
            .for_each(|chord| chord.key = k);
        self
            .chord_table
            .iter_mut()
            .for_each(|chords| chords
                .iter_mut()
                .for_each(|chord| chord.key = k));
        self.chord_table.rotate_right(k as usize);

    }

    /// Picks a random chord from the 2-dimensional list of chords.
    fn pick_chord(&mut self) -> Chord {
        let mut i = 0;
        let mut note = self.math_magician.pick_note();
        loop {
            let chord_list = self.chord_table[note as usize].to_owned();
            if chord_list.len() != 0 {
                return chord_list[self.math_magician.big_decision(0, (chord_list.len() - 1) as u16) as usize].to_owned();
            }
            i += 1;
            note = (note + 1) % 12;
            if i > 24 {
                return Chord::default();
            }
        }
    }

    /// Picks a random chord from the `chord_list` 1-dimensional list of chords.
    fn pick_chord_1d(&mut self) -> Chord {
        if self.chord_list.len() == 0 {
            return Chord::default();
        }
        let chord_index = self.math_magician.big_decision(0, (self.chord_list.len() - 1) as u16);
        
        self.chord_list[chord_index as usize].to_owned()
    }
    
    /// The original implementation of `def place(self, octave, initTime, isHighPos = True)
    /// 
    /// "melody" mode
    pub fn original_placement_algorithm(&mut self, chord: &Chord, octave: i16, initial_time: u32) {
        for note in chord.get_notes().iter() {
            let note_to_play = (note + 12 * octave + self.key) as u8;
            
            // pick note lengths such that total_time reaches 4.0
            let mut total_time = 0.0;
            while total_time < 4.0 {
                // pick a random note length that is between [0.5, 4.0 - total_time]
                let max_index: u16;
                if total_time == 0.0 {
                    max_index = 4; // this is technically a bug; it's supposed to be 7
                }else{
                    max_index = get_max_note_length_index(total_time);
                }
                let chosen_index = self.math_magician.big_decision(0, max_index);
                total_time += NOTE_LENGTHS[chosen_index as usize];
                self.midi_file.add_note_beats(
                    note_to_play, 
                    initial_time as f64 + total_time,
                    total_time, 
                    80
                );
            }
        }
    }

    /// Fixed version of original placement algorithm.
    /// 
    /// "intended" generation mode
    fn place_variable_len_fixed(&mut self, chord: &Chord, octave: i16, initial_time: u32) {
        let notes = chord.get_notes();

        // pick note lengths such that total_time reaches 4.0
        let mut total_time = 0.0;
        while total_time < 4.0 {
            // pick a random note length that is between [0.5, 4.0 - total_time]
            let max_index = get_max_note_length_index(total_time);
            let chosen_index = self.math_magician.big_decision(0, max_index);
            let note_length = NOTE_LENGTHS[chosen_index as usize];

            // apply note length to all notes
            for note in notes.iter() {
                let note_to_play = (note + 12 * octave + self.key) as u8;
                self.midi_file.add_note_beats(
                    note_to_play, 
                    initial_time as f64 + total_time, 
                    note_length, 
                    80
                );
            }

            total_time += note_length;
        }
    }

    /// Places chords in a regular manner.
    /// 
    /// "chords" generation mode
    pub fn place_chord_regular(&mut self, chord: &Chord, octave: i16, initial_time: u32) {
        let notes = chord.get_notes();
        let note_length = 4.0;
        for note in notes.iter() {
            let note_to_play = (note + 12 * octave + self.key) as u8;

            self.midi_file.add_note_beats(note_to_play, initial_time as f64, note_length, 80);
        }
        let optional_notes = chord.get_optional_notes();
        // optionally play optional notes
        for note in optional_notes.iter() {
            if self.math_magician.big_decision(0, 100) > 69 {
                let note_to_play = (note + 12 * octave + self.key as i16) as u8;
                self.midi_file.add_note_beats(note_to_play, initial_time as f64, note_length, 80);
            }
        }
    }

    /// Another buggy chord placement algorithm.
    /// 
    /// "melody v2"
    pub fn place_chord_bug_v2(
        &mut self, 
        chord: &Chord,
        _octave: i16, 
        initial_time: u32
    ) {
        //let octave = self.math_magician.pick_note() % 2 + 4;
        let mut note_index = 0;
        let notes = chord.get_notes();
        let octave: i16;
        if notes[0] < 6 {
            octave = self.math_magician.pick_note() % 2 + 4;
        }else{
            octave = self.math_magician.pick_note() % 2 + 3;
        }
        for note in notes {
            let oct_shift: i8 = if note_index < 2 && self.math_magician.pick_note() < 2 {
                12
            }else if note_index >= 2 && self.math_magician.pick_note() < 2 {
                -12
            }else{0};
            note_index += 1;
            let note_to_play = (12 * octave as i8 + oct_shift) as u8 + note as u8;
            let mut total_time = 0.0;

            loop {
                if total_time >= 4.0 {
                    break;
                }

                let max_index = get_max_note_length_index(total_time);

                let chosen_index = self.math_magician.big_decision(0, max_index as u16);
                let duration = NOTE_LENGTHS[chosen_index as usize];

                self.midi_file.add_note_beats(
                    note_to_play + self.key as u8, 
                    initial_time as f64 + total_time, 
                    duration,
                    self.math_magician.big_decision(70, 90) as u8
                );
                total_time += duration;
            }
        }
    }

    /// Another buggy chord placement algorithm.
    /// 
    /// "melody v3"
    pub fn place_chord_bug_v3(
        &mut self, 
        chord: &Chord, 
        octave: i16,
        initial_time: u32
    ) { 
        //let notes = self.get_modified_notes(chord);
        
        for note in chord.get_notes() {
            let mut total_time = 0.0;

            loop {
                if total_time >= 4.0 {
                    break;
                }

                let max_index = get_max_note_length_index(total_time);

                let chosen_index = self.math_magician.big_decision(0, max_index as u16);
                let duration = NOTE_LENGTHS[chosen_index as usize];

                self.midi_file.add_note_beats(
                    note as u8 + self.key as u8 + (octave * 12) as u8, 
                    initial_time as f64 + total_time, 
                    duration,
                    self.math_magician.big_decision(70, 90) as u8
                );
                total_time += duration;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! init_music {
        ($chosen_key:expr) => {
            Music::smoke_hash(Default::default(), $chosen_key, &HashSet::new(), "default", "disabled").unwrap()
        };
    }

    #[test]
    fn key_parsing() {
        let m = init_music!("Cmin");
        assert_eq!(m.key, 0);
        let m = init_music!("Cmaj");
        assert_eq!(m.key, 3);
        let m = init_music!("Dmin");
        assert_eq!(m.key, 2);
        let m = init_music!("Dmaj");
        assert_eq!(m.key, 5);
    }
}