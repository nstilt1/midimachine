use midly::{MidiMessage, TrackEvent, TrackEventKind, num::{u7, u28}};
use rand::{rngs::StdRng, Rng};

use super::{chord_type::*, utils::*};

#[derive(Clone, Debug)]
pub struct Chord {
    pub chord_type: ChordType,
    pub root: u8,
    _chords_to_not_play_next: Vec<Chord>,
}

impl Default for Chord {
    fn default() -> Self {
        Self {
            chord_type: ChordType::default(),
            root: 0,
            _chords_to_not_play_next: Vec::new()
        }
    }
}

impl Chord {
    pub fn new(root_index: u8, chord_type: &ChordType) -> Self {
        Chord {
            chord_type: chord_type.to_owned(),
            root: root_index,
            _chords_to_not_play_next: Vec::new()
        }
    }

    /**
     * Gets an array of pitches of the notes for this chord
     * returns note_intervals.map(|n| n + self.root)
     */
    pub fn get_notes(&self) -> Vec<i16> {
        let mut result: Vec<i16> = Vec::new();
        for n in self.chord_type.to_owned().note_intervals {
            result.push((n + self.root) as i16);
        }
        return result
    }

    /**
     * Gets an array of optional notes for this chord.
     */
    pub fn get_optional_notes(&self) -> Vec<i16> {
        let mut result: Vec<i16> = Vec::new();
        for n in self.chord_type.optional_notes.iter() {
            result.push((n + self.root) as i16);
        }
        return result
    }

    /**
     * Places this chord into a midi track 'rhythmically'
     */
    #[allow(unused)]
    pub fn place_fixed_variable_len(
        &self, 
        track: &mut Vec<TrackEvent>, 
        octave_addition: u8, 
        start_beats: f64, 
        is_high_pos: Option<bool>,
        rng: &mut StdRng
    ) {
        let octave = 4;// + octave_addition;
        let start_ticks = beats(start_beats);
        let mut total_time: f64 = 0.0;
        let note_lengths: Vec<f64> = vec![0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0];
        loop {
            let mut max_index: usize;
            if total_time >= 3.9 {
                break;
            }
            if total_time == 0.0 {
                max_index = 7;
            }else{
                // find the index of the element, where the element + total_time <= 4
                let max_value = 4f64 - total_time;

                // find the index of max_value
                // note_lengths[i] = f(x)
                // f(x) = 0.5*i + 0.5
                // f(x) - 0.5 = 0.5*i;
                // (f(x) - 0.5)/0.5 = i
                // i = f(x) * 2 - 1
                max_index = (max_value * 2f64 - 1f64).round() as usize;

                // original fix attempt: it is inefficient
                /*
                max_index = -1;
                i = 6
                while max_index < 0 {
                    max_index = i - note_lengths.index_of(total_time);
                }
                */
            }
            
            // randomize length of chord
            //let note_length_beats = note_lengths[rng.gen_range(0..=max_index)];
            let note_length_beats = 4.0;

            // uncomment the next line to remove rhythm

            
            // elevate the notes to start in the octave
            let mut elevated_notes = self.get_notes();
            for note in elevated_notes.iter_mut() {
                *note = *note + ((12 * octave) as i16);
            }

            // invert one of the notes in the chord by chance
            if rng.gen_range(0..=5) > 3 {
                elevated_notes[1] -= 12;
            }
            // invert another note
            if rng.gen_range(0..=5) > 3 {
                elevated_notes[2] -= 12;
            }

            // place notes
            let mut has_placed_note = false;
            let mut vels: Vec<u7> = Vec::new();
            for note in elevated_notes.to_owned() {
                let final_note = (note + self.root as i16) as u8;
                let vel: u7 = rng.gen_range(65..100).into();
                vels.push(vel);
                let delta: u28;
                //if has_placed_note || start_beats == 0.0 {
                    delta = 0.into();
                //}else{
                //    delta = (beats(4.0)).into();
                //}
                track.push(TrackEvent { 
                    delta, 
                    kind: TrackEventKind::Midi { 
                        channel: 0.into(), 
                        message: MidiMessage::NoteOn { 
                            key: final_note.into(), 
                            vel: vel.to_owned(), 
                        }, 
                    },
                });
                has_placed_note = true;
            }
            vels.reverse();
            has_placed_note = false;
            for note in elevated_notes {
                let final_note = (note + self.root as i16) as u8;
                let vel: u7 = vels.pop().unwrap();
                let delta: u28;
                if has_placed_note {
                    delta = 0.into();
                }else{
                    delta = (beats(4.0)).into();
                }
                track.push(TrackEvent {
                    delta,
                    kind: TrackEventKind::Midi { 
                        channel: 0.into(), 
                        message: MidiMessage::NoteOff { 
                            key: final_note.into(), 
                            vel: vel.to_owned() 
                        }, 
                    },
                });
                has_placed_note = true;
                
                total_time += 4.0;
            }
            total_time += note_length_beats;
            if total_time > 3.9 {
                break;
            }
        }
    }
}