use midly::{Timing::Metrical, Smf, MidiMessage, TrackEvent, TrackEventKind, num::u7};
use rand::Rng;

use crate::my_modules::{utils::to_base64::Base64String, error::HttpError};

use super::{chord_type::ChordType, chord::Chord, utils::MathMagician};

#[derive(Debug)]
pub struct Music {
    math_magician: MathMagician,
    chord_types: Vec<ChordType>,
    notes_of_chords: Vec<Vec<Chord>>,
    chords_of_scale: Vec<Chord>
}

impl Music {
    pub fn smoke_hash(hash: String) -> Result<Music, HttpError> {
        let stash = hash.from_base64();
        if stash.is_err() {
            return Err(stash.unwrap_err());
        }
        let stash = stash.unwrap();
        if stash.len() < 32 {
            return Err("Error: Not enough hash.".into());
        }
        if stash.len() > 32 {
            return Err("Error: Too much hash.".into());
        }

        let mut math_magician = MathMagician::share_hash(stash);

        let key = math_magician.big_decision(0, 11);
        // define chord types:
        let minor7 = ChordType::new([0, 3, 7, 10].into(), [0, 2, 5, 7].into(), None);
        let major7 = ChordType::new([0, 4, 7, 11].into(), [3].into(), None);
        //let major = ChordType::new([0, 3, 7].into(), [0, 2, 5, 7].into(), None);
        //let minor = ChordType::new([0, 4, 7].into(), [3, 8, 10].into(), None);
        let diminished = ChordType::new([0, 3, 6].into(), [3].into(), None);
        let augmented = ChordType::new([0,4,8].into(), [10].into(), Some([12].into()));
        let major6 = ChordType::new([0, 4, 7, 9].into(), [3, 10].into(), None);

        let minor6 = ChordType::new([0, 9, 15, 19].into(), [0, 2, 5, 7].into(), None);
        let major9 = ChordType::new([0, 4, 10, 14].into(), [0, 2, 5, 7].into(), None);
        let major7sharp9 = ChordType::new([0, 4, 10, 15].into(), [5, 7].into(), None);
        // add high e string version of minor6, major9, and major7sharp9,
        // as well as the low E string version of mjaor 9, major7sharp9
        // also add major6add9

        //let chord_types = vec![minor7, major7, major, minor, diminished, augmented, major6];
        let chord_types = vec![minor7, major7, diminished, augmented, major6];
        // figure out what chords a note can belong to
        // also get a list of some chords that might be in a scale
        let mut notes_of_chords: Vec<Vec<Chord>> = (0..12).map(|_| Vec::new()).collect();
        let mut chords_of_scale: Vec<Chord> = Vec::new();

        for ct in chord_types.to_owned() {
            for r in ct.clone().roots {
                let chord = Chord::new(r, ct.clone());
                chords_of_scale.push(chord.clone());
                for note in chord.get_notes() {
                    notes_of_chords[(note % 12) as usize].push(chord.clone());
                }
            }
        }
        return Ok(Music {
            math_magician,
            notes_of_chords: notes_of_chords,
            chord_types: chord_types,
            chords_of_scale: chords_of_scale
        });
    }

    pub fn make_music(&mut self, buds: u16) -> Result<Vec<TrackEvent>, HttpError> {
        
        
        let mut track: Vec<TrackEvent> = Vec::new();

        let mut last_chord: Chord;

        let mut i = 0;
        while i < buds {
            let chord = self.pick_chord()?;
            let octave = self.math_magician.big_decision(0, 1);
            chord.place_fixed_variable_len(&mut track, octave as u8, (i) as f32, Some(true));
            i += 1;
        }

        return Ok(track);
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
}