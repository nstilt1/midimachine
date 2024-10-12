use midly::TrackEvent;
use sha2::Sha256;

use crate::my_modules::error::HttpError;

use super::{chord_type::ChordType, chord::Chord, utils::{MathMagician, add_octaves}, midi::MidiFile};

const DEFAULT_NUM_CHORDS: usize = 20;

#[derive(Debug)]
pub struct Music {
    math_magician: MathMagician,
    midi_file: MidiFile,
    key: u16,
    _chord_types: Vec<ChordType>,
    notes_of_chords: Vec<Vec<Chord>>,
    _chords_of_scale: Vec<Chord>
}

/// This macro picks a chord based on the user's request.
/// 
/// This could have been written prettier by checking the user's input inside 
/// the for-loop, but then every iteration of the for-loop would have at least 
/// one extra comparison. And potentially more than one if more output types 
/// are added later.
macro_rules! pick_chord_placement_method {
    ($music_obj:expr, $user_selected_type:expr, $num_chords:expr, $should_use_same_chords:expr, $(($chord_placement_str:expr, $method:ident)),*) => {
        if $should_use_same_chords {
            let mut chords = vec![Chord::default(); $num_chords];
            for chord in chords.iter_mut() {
                *chord = $music_obj.pick_chord()?;
            }

            match $user_selected_type {
                $(
                    $chord_placement_str => {
                        for (i, chord) in chords.iter().enumerate() {
                            $music_obj.$method(&chord, 4, (i as u32 * 4).into());
                        }
                    },
                )*
                _ => { (); }
            }
        } else {
            match $user_selected_type {
                $(
                    $chord_placement_str => {
                        for i in 0..$num_chords {
                            let chord = $music_obj.pick_chord()?;
                            $music_obj.$method(&chord, 4, (i as u32 * 4).into());
                        }
                    }
                )*
                _ => { (); }
            }
        }
    };
}

impl Music {
    pub fn smoke_hash(hash: sha2::digest::Output<Sha256>) -> Result<Music, HttpError> {
        let mut stash = [0u8; 32];
        stash.copy_from_slice(&hash);
        let mut math_magician = MathMagician::share_hash(stash);

        let key = math_magician.pick_note();
        let minor7 = ChordType::new(&[0, 10, 15, 19], &[0, 2, 5, 6, 10], None);
        let major7 = ChordType::new(&[0, 11, 16, 19], &[3, 8], None);
        let diminished = ChordType::new(&[0, 3, 6], &[3, 6], None);
        let augmented = ChordType::new(&[0,4,8], &[2,6,10], Some(&[12]));
        //let major6 = ChordType::new(&[0, 4, 7, 9], &[3, 10], None);
        let major6 = ChordType::new(&[0, 9, 16, 19], &[3, 8, 10], Some(&[23]));

        //let major = ChordType::new([0, 3, 7].into(), [0, 2, 5, 7].into(), None);
        //let minor = ChordType::new([0, 4, 7].into(), [3, 8, 10].into(), None);

        let minor6 = ChordType::new(&[0, 9, 15, 19], &[0, 2, 5, 7], None);
        let major9 = ChordType::new(&[0, 4, 10, 14], &[0, 5, 7], None);
        let major7sharp9 = ChordType::new(&[0, 4, 10, 15], &[0, 2, 7, 9], None);
        let major7flat5sharp9 = ChordType::new(&[0, 4, 10, 15, 18], &[0, 9], None);
        let major9flat5 = ChordType::new(&[0, 4, 10, 15, 17], &[0, 9], None);
        let major7flat9 = ChordType::new(&[0, 4, 10, 13], &[0, 2], None);

        //let major13 = ChordType::new([0, 5, 10, 21, 26, 31].into(), [0].into(), Some([0, 5].into()));
        //let dominant9 = ChordType::new([0, 4, 9, 14, 18].into(), [1].into(), None);
        //let major6add9 = ChordType::new([])

        // add high e string version of minor6, major9, and major7sharp9,
        // as well as the low E string version of mjaor 9, major7sharp9
        // also add major6add9

        //let chord_types = vec![minor7, major7, major, minor, diminished, augmented, major6];

        let chord_types = vec![
            minor7,
            major7,
            diminished,
            augmented,
            major6,
            minor6,
            major9,
            major7sharp9,
            major7flat5sharp9,
            major9flat5,
            major7flat9
        ];

        let mut notes_of_chords: Vec<Vec<Chord>> = (0..12).map(|_| Vec::new()).collect();
        let mut chords_of_scale: Vec<Chord> = Vec::new();

        for ct in chord_types.iter() {
            for r in ct.clone().roots {
                let chord = Chord::new(r, &ct);
                chords_of_scale.push(chord.clone());
                for note in chord.get_notes() {
                    notes_of_chords[(note % 12) as usize].push(chord.clone());
                }
            }
        }

        return Ok(Music {
            math_magician,
            midi_file: MidiFile::new(),
            key: key,
            notes_of_chords: notes_of_chords,
            _chord_types: chord_types,
            _chords_of_scale: chords_of_scale
        })
    }

    /**
     * Makes some music midily
     */
    pub fn make_music(&mut self, num_chords: usize, generation_mode: &str, should_use_same_chords: bool) -> Result<Vec<TrackEvent>, HttpError> {
        let mut last_chords: Vec<Vec<u8>> = Vec::new();

        // determine chords before any other RNG calls are made so that the 
        // same chords are used for all output types

        pick_chord_placement_method!(
            self,
            generation_mode, 
            num_chords,
            should_use_same_chords,
            ("melody", original_place),
            ("chords", place_chord_regular)
        );

        /*
        while j < 4 {
            i = 0;
            while i < 8 {
                let chord = self.pick_chord()?;
                //let octave = self.math_magician.big_decision(0, 1);
                //chord.place_fixed_variable_len(&mut track, octave as u8, (i) as f32, Some(true));
                //self.place_chord_bug_v3(&chord, i.into());
                last_chords.push(self.get_modified_notes(&chord));
                i += 1;
            }
            i = 0;
            while i < 8 {
                self.place_chord_bug_v3(&last_chords[i], j as u32*32 + i as u32 * 4);
                i += 1;
            }
            last_chords = Vec::new();
            j += 1;
        }
        */

        

        return Ok(self.midi_file.finalize());

        
    }

    fn pick_chord(&mut self) -> Result<Chord, HttpError> {
        let mut i = 0;
        let mut note = self.math_magician.pick_note();
        loop {
            let chord_list = self.notes_of_chords[note as usize].to_owned();
            if chord_list.len() != 0 {
                return Ok(chord_list[self.math_magician.big_decision(0, (chord_list.len() - 1) as u16) as usize].to_owned());
            }
            i += 1;
            note = (note + 1) % 12;
            if i > 12 {
                return Err("Error M94: notes_of_chords not populated".into());
            }
        }
    }

    /**
     * The original implementation of `def place(self, octave, initTime, isHighPos = True)
     */
    pub fn original_place(&mut self, chord: &Chord, octave: i16, initial_time: u32) {
        let notes = chord.get_notes();
        let note_lengths = vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
        //println!("Beggining for loop");
        for note in notes.iter() {
            let t_note = (note + 12 * octave) as u8;
            
            let mut total_time = 0.0;
            //println!("Beggining inner loop");
            loop {
                let mut max_length: i32;
                //println!("Beggining inner inner if statement");
                if total_time < 4.0 {
                    if total_time == 0.0 {
                        max_length = 4;
                    }else{
                        max_length = -1;
                        //println!("Beginning inner inner while loop");
                        while max_length < 0 {
                            // the following line is equivalent to 
                            // max_length = 6 - noteLengths.index(totalTime)
                            max_length = 6 - note_lengths.iter().position(|&r| r == total_time).unwrap() as i32;
                        }
                    }
                    let i = self.math_magician.big_decision(0, max_length as u16);
                    total_time += note_lengths[i as usize];
                    self.midi_file.add_note_beats(t_note, initial_time as f64 + total_time, total_time, 80);

                }else{
                    break;
                }
            }
        }
    }

    pub fn place_chord_regular(&mut self, chord: &Chord, octave: i16, initial_time: u32) {
        let notes = chord.get_notes();
        let note_length = 4.0;
        for note in notes.iter() {
            let t_note = (note + 12 * octave + self.key as i16) as u8;

            self.midi_file.add_note_beats(t_note, initial_time as f64, note_length, 80);
        }
        let optional_notes = chord.get_optional_notes();
        for note in optional_notes.iter() {
            if self.math_magician.big_decision(0, 100) > 69 {
                let t_note = (note + 12 * octave) as u8;
                self.midi_file.add_note_beats(t_note, initial_time as f64, note_length, 80);
            }
        }
    }


    pub fn place_chord_bug_combo_1(
        &mut self,
        chord: &Chord, 
        initial_time: u32, 
        is_high_pos: bool
    ) {
        let notes = chord.get_notes();
        let octave = 4;
        let mut total_time: f64 = 0.0;
        let note_lengths: Vec<f64> = vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
        loop {
            for note in notes.iter() {
                if total_time < 4.0 {
                    let max_index: usize;
                    if total_time != 0.0 {
                        let max_value = 4 as f64 - total_time;

                        max_index = (max_value * 2 as f64 - 1 as f64).round() as usize;
                    }else{
                        max_index = note_lengths.len() - 1;
                    }

                    
                    let duration_index = 7;
                    //let duration_index = self.math_magician.big_decision(0, max_index as u16);
                    let duration = note_lengths[duration_index as usize];
                    
                    total_time += duration;

                    self.midi_file.add_note_beats(
                        add_octaves(*note, octave), 
                        initial_time as f64 + total_time, 
                        duration, 
                        80
                    );
                }
            }
            if total_time >= 4.0 {
                break;
            }
        }
    }

    pub fn place_chord_bug_v2(
        &mut self, chord: &Chord, initial_time: u32, is_high_pos: bool
    ) {
        //let octave = self.math_magician.pick_note() % 2 + 4;
        let note_lengths = vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
        let mut note_index = 0;
        let notes = chord.get_notes();
        let octave: u16;
        if notes[0] < 6 {
            octave = self.math_magician.pick_note() %2 + 4;
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
            let t_note = (12 * octave as i8 + oct_shift) as u8 + note as u8;
            let mut total_time = 0.0;

            loop {
                if total_time >= 4.0 {
                    break;
                }
                let mut max_length = 4.0;
                let max_index: usize;
                if total_time != 0.0 {
                    let max_value = 4 as f64 - total_time;

                    max_index = (max_value * 2 as f64 - 1 as f64).round() as usize;
                    
                    
                }else{
                    max_index = note_lengths.len()-1;
                }

                let i = self.math_magician.big_decision(0, max_index as u16);
                let duration = note_lengths[i as usize];

                self.midi_file.add_note_beats(
                    t_note + self.key as u8, 
                    initial_time as f64 + total_time, 
                    duration,
                    self.math_magician.big_decision(70, 90) as u8
                );
                total_time += duration;
            }
        }
    }

    pub fn place_chord_bug_v3(
        &mut self, notes: &Vec<u8>, initial_time: u32
    ) {
        let note_lengths = vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
        
        //let notes = self.get_modified_notes(chord);
        
        for note in notes {
            
            
            
            let mut total_time = 0.0;

            loop {
                if total_time >= 4.0 {
                    break;
                }
                let mut max_length = 4.0;
                let max_index: usize;
                if total_time != 0.0 {
                    let max_value = 4 as f64 - total_time;

                    max_index = (max_value * 2 as f64 - 1 as f64).round() as usize;
                    
                    
                }else{
                    max_index = note_lengths.len()-1;
                }

                let i = self.math_magician.big_decision(0, max_length as u16);
                let duration = note_lengths[i as usize];

                self.midi_file.add_note_beats(
                    note + self.key as u8, 
                    initial_time as f64 + total_time, 
                    duration,
                    self.math_magician.big_decision(70, 90) as u8
                );
                total_time += duration;
            }
        }
    }

    

    pub fn get_modified_notes(&mut self, chord: &Chord) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let intervals = chord.chord_type.note_intervals.to_owned();
        let octave: i16;
        if intervals[0] + chord.root < 6 {
            octave = (self.math_magician.big_decision(4, 5) * 12) as i16;
        }else{
            octave = (self.math_magician.big_decision(3, 4) * 12) as i16;
        }
        for n in 0..intervals.len() {
            let i = intervals[n];
            let n2 = (i + chord.root) as i16;
            let magic_number = self.math_magician.pick_note();
            let octave_shift: i16;
            if i < 6 {
                if magic_number < 3 {
                    octave_shift = 12;
                }else{
                    octave_shift = 0;
                }
            }else{
                if magic_number < 3 {
                    octave_shift = -12;
                }else{
                    octave_shift = 0;
                }
            }
            result.push((octave_shift + n2 + octave) as u8);
        }
        return result;
    }
}