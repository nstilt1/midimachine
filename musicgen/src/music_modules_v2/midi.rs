// use midly::

use std::collections::HashMap;

use midly::{TrackEvent, TrackEventKind, MidiMessage};

use super::utils::beats;

type Track<'a> = Vec<TrackEvent<'a>>;

/**
 * Goal: have an object with a method addNote(pitch, startTime, duration, volume)
 * To work, it will need to have a data structure that can sort all items in it by 
 * startTime, while keeping all of the other values
 * There will be duplicate startTimes
 */
#[derive(Debug)]
pub struct MidiFile {
    notes: Vec<MidiNote>
}

#[derive(Debug)]
pub struct MidiNote {
    pitch: u8,
    note_on: bool,
    start_time: u32,
    volume: u8
}

impl MidiFile {
    #[inline(always)]
    pub fn new() -> Self {
        MidiFile{notes: Vec::new()}
    }
    #[inline(always)]
    pub fn add_note_beats(
        &mut self,
        pitch: u8,
        initial_time: f64,
        duration: f64,
        volume: u8
    ) {
        self.add_note(pitch, beats(initial_time), beats(duration), volume);
    }

    /**
     * Add a note to the midi file
     */
    #[inline(always)]
    fn add_note(
        &mut self, 
        pitch: u8,
        initial_time: u32, 
        duration: u32, 
        volume: u8
    ) {
        self.notes.push(MidiNote { 
            pitch: pitch.to_owned(),
            note_on: true,
            start_time: initial_time.to_owned(), 
            volume: volume.to_owned() 
        });
        self.notes.push(MidiNote {
            pitch: pitch.to_owned(),
            note_on: false,
            start_time: initial_time.to_owned() + duration.to_owned(),
            volume: volume.to_owned()
        });
    }

    /**
     * A little helper function to finish creating the Vec<TrackEvent>, aka the Track
     */
    #[inline(always)]
    pub fn finalize(&mut self) -> Track {
        self.notes.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut result: Track = Vec::new();
        let mut last_time = 0;

        for n in self.notes.iter() {
            result.push_track_event(n.start_time, last_time, n.pitch, n.volume, n.note_on);

            last_time = n.start_time;
        }

        return result;
    }

    /**
     * This is an alternate version of the finalize() function where the notes 
     * do not overlap. I don't think it sounds as good since there's more empty 
     * space and the timing seems more machine-made given the uniformity of 
     * them.
     */
    #[allow(unused)]
    fn finalize_no_overlap(&mut self) -> Vec<TrackEvent> {
        self.notes.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut result: Vec<TrackEvent> = Vec::new();
        let mut last_time = 0;

        let mut on_notes: HashMap<u8, u8> = HashMap::new();
        for n in self.notes.iter() {
            // if the note is supposed to be played, and it is already playing, end the note
            if n.note_on {
                if on_notes.contains_key(&n.pitch) {
                    result.push_track_event(n.start_time, last_time, n.pitch, *on_notes.get(&n.pitch).unwrap(), false);
                }else{
                    on_notes.insert(n.pitch, n.volume);
                }
                result.push_track_event(n.start_time, last_time, n.pitch, n.volume, true);
            }else{
                if on_notes.contains_key(&n.pitch) {
                    result.push_track_event(n.start_time, last_time, n.pitch, n.volume, false);
                    on_notes.remove(&n.pitch);
                }
            }

            last_time = n.start_time;
        }

        return result;
    }
}

trait TrackEventVecUtils {
    /**
     * A one-line way to add a track event to a vector
     */
    fn push_track_event(&mut self, start_time: u32, last_time: u32, pitch: u8, velocity: u8, is_on: bool);
}

impl TrackEventVecUtils for Vec<TrackEvent<'_>> {
    #[inline(always)]
    fn push_track_event(&mut self, start_time: u32, last_time: u32, pitch: u8, velocity: u8, is_on: bool) {
        self.push(TrackEvent { 
            delta: (start_time - last_time).into(), 
            kind: TrackEventKind::Midi {
                channel: 0.into(),
                message: if is_on {
                    MidiMessage::NoteOn { key: pitch.into(), vel: velocity.into() }
                }else{
                    MidiMessage::NoteOff { key: pitch.into(), vel: velocity.into() }
                }
            } 
        });
    }
}

impl PartialEq for MidiNote {
    fn eq(&self, other: &Self) -> bool {
        return self.pitch == other.pitch 
        && self.note_on == other.note_on 
        && self.start_time == other.start_time 
        && self.volume == other.volume;
    }
}

impl PartialOrd for MidiNote {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let start_comparison = self.start_time.partial_cmp(&other.start_time);
        if start_comparison == Some(std::cmp::Ordering::Equal) {
            if self.note_on != other.note_on {
                return Some(if self.note_on{
                    std::cmp::Ordering::Greater
                }else{
                    std::cmp::Ordering::Less
                })
            }
        }
        start_comparison
    }
}