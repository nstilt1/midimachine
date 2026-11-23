//! Seeded, bar-aware hip-hop drum generation and 960 PPQ MIDI export.
use crate::music_modules_v2::midi::MidiFile;
use midly::{Format, Header, MetaMessage, Smf, Timing, TrackEvent, TrackEventKind};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const PPQ: u32 = 960;
pub const BAR: u32 = PPQ * 4;
pub const NAMES: [&str; 10] = [
    "kick",
    "hihat_closed",
    "hihat_open",
    "hihat_pedal",
    "snare_low",
    "snare_high",
    "clap",
    "tom_low",
    "tom_mid",
    "tom_high",
];
const DEFAULT_NOTES: [u8; 10] = [36, 42, 46, 44, 38, 40, 39, 45, 47, 50];

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Style {
    BoomBap,
    Trap,
    LoFi,
    Modern,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub bpm: u16,
    pub bars: u16,
    pub time_signature: [u8; 2],
    pub style: Style,
    pub complexity: f32,
    pub density: f32,
    pub swing: f32,
    pub humanization: f32,
    pub variation: f32,
    pub fill_probability: f32,
    pub ghost_probability: f32,
    pub roll_probability: f32,
    pub seed: u64,
    pub seed_text: String,
    pub enabled: [bool; 10],
    pub notes: [u8; 10],
    pub velocity_ranges: [[u8; 2]; 10],
    pub channel: u8,
    pub individual_files: bool,
    pub metadata: bool,
    pub filename_prefix: String,
    pub half_time: bool,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            bpm: 90,
            bars: 4,
            time_signature: [4, 4],
            style: Style::BoomBap,
            complexity: 0.5,
            density: 0.5,
            swing: 0.15,
            humanization: 0.12,
            variation: 0.4,
            fill_probability: 0.35,
            ghost_probability: 0.2,
            roll_probability: 0.25,
            seed: 0,
            seed_text: String::new(),
            enabled: [
                true, true, true, false, true, false, true, false, false, false,
            ],
            notes: DEFAULT_NOTES,
            velocity_ranges: [
                [40, 127],
                [30, 110],
                [40, 110],
                [30, 100],
                [20, 120],
                [20, 120],
                [40, 120],
                [40, 115],
                [40, 115],
                [40, 115],
            ],
            channel: 9,
            individual_files: false,
            metadata: true,
            filename_prefix: "drums".into(),
            half_time: false,
        }
    }
}
impl Config {
    pub fn seed_label(&self) -> String {
        if self.seed_text.is_empty() {
            self.seed.to_string()
        } else {
            self.seed_text.clone()
        }
    }
    pub fn seed_digest(&self) -> [u8; 32] {
        Sha256::digest(self.seed_label().as_bytes()).into()
    }
    pub fn validate(&self) -> Result<(), String> {
        if !(30..=300).contains(&self.bpm)
            || !(1..=1024).contains(&self.bars)
            || self.time_signature != [4, 4]
            || self.channel > 15
            || self.notes.iter().any(|n| *n > 127)
            || self
                .velocity_ranges
                .iter()
                .any(|range| range[0] == 0 || range[0] > range[1] || range[1] > 127)
        {
            return Err("Invalid BPM, bars, signature, channel, or mapping".into());
        }
        if self.filename_prefix.is_empty()
            || self.filename_prefix.len() > 64
            || !self
                .filename_prefix
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err("Invalid filename prefix".into());
        }
        if [
            self.complexity,
            self.density,
            self.swing,
            self.humanization,
            self.variation,
            self.fill_probability,
            self.ghost_probability,
            self.roll_probability,
        ]
        .iter()
        .any(|x| !x.is_finite() || !(0.0..=1.0).contains(x))
        {
            return Err("Groove controls must be between 0 and 1".into());
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Hit {
    pub instrument: usize,
    pub tick: u32,
    pub duration: u32,
    pub velocity: u8,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub config: Config,
    pub hits: Vec<Hit>,
}
fn chance(r: &mut ChaCha8Rng, p: f32) -> bool {
    r.gen::<f32>() < p.clamp(0., 1.)
}
fn add(h: &mut Vec<Hit>, instrument: usize, tick: u32, velocity: u8) {
    h.push(Hit {
        instrument,
        tick,
        duration: if instrument == 2 { 180 } else { 90 },
        velocity,
    });
}
fn velocity(r: &mut ChaCha8Rng, lo: u8, hi: u8) -> u8 {
    r.gen_range(lo..=hi)
}
impl Pattern {
    pub fn generate(config: Config) -> Result<Self, String> {
        config.validate()?;
        let mut rng = ChaCha8Rng::from_seed(config.seed_digest());
        let mut hits = Vec::new();
        let fast = (110. / config.bpm as f32).clamp(0.45, 1.35);
        let half = config.half_time || config.style == Style::Trap;
        let snare_beats: &[u32] = if half { &[2] } else { &[1, 3] };
        // Phrase templates preserve a recognisable pocket; randomness decorates it.
        let templates: &[&[u32]] = match config.style {
            Style::BoomBap => &[&[0, 6, 10], &[0, 7, 8, 14], &[0, 3, 10]],
            Style::LoFi => &[&[0, 7, 10], &[0, 6, 11], &[0, 10]],
            Style::Trap => &[&[0, 6, 11], &[0, 3, 14], &[0, 7, 12, 15]],
            Style::Modern => &[&[0, 6, 8, 14], &[0, 3, 10, 14], &[0, 7, 10]],
        };
        let template = templates[rng.gen_range(0..templates.len())];
        let base_kicks: Vec<u32> = template.iter().enumerate()
            .filter(|(i, _)| *i == 0 || chance(&mut rng, 0.55 + config.density * 0.45))
            .map(|(_, step)| step * 240).collect();
        for bar in 0..config.bars as u32 {
            let offset = bar * BAR;
            let transition = (bar + 1) % 4 == 0 || bar + 1 == config.bars as u32;
            let variation = if bar == 0 {
                0.
            } else {
                config.variation * if transition { 1. } else { 0.55 }
            };
            let mut kicks = base_kicks.clone();
            if chance(&mut rng, variation * 0.75) {
                let options = [3, 5, 7, 10, 13, 15];
                kicks.push(options[rng.gen_range(0..options.len())] * 240);
            }
            if kicks.len() > 1 && chance(&mut rng, variation * 0.35) {
                kicks.remove(rng.gen_range(1..kicks.len()));
            }
            for tick in kicks {
                if config.enabled[0] {
                    add(&mut hits, 0, offset + tick, velocity(&mut rng, 82, 123));
                }
            }
            for &beat in snare_beats {
                let primary_snare = if config.enabled[4] {
                    Some(4)
                } else if config.enabled[5] {
                    Some(5)
                } else {
                    None
                };
                if let Some(snare) = primary_snare {
                    add(
                        &mut hits,
                        snare,
                        offset + beat * PPQ,
                        velocity(
                            &mut rng,
                            if config.style == Style::LoFi { 72 } else { 88 },
                            if config.style == Style::LoFi {
                                102
                            } else {
                                118
                            },
                        ),
                    );
                }
                if config.enabled[6] && (primary_snare.is_none() || (config.style == Style::Modern && chance(&mut rng, 0.65))) {
                    add(
                        &mut hits,
                        6,
                        offset + beat * PPQ,
                        velocity(&mut rng, 72, 102),
                    );
                }
            }
            let hat_step = if config.style == Style::Trap && chance(&mut rng, 0.7 * fast) {
                240
            } else if chance(&mut rng, (config.density * 0.6 * fast).min(0.85)) {
                240
            } else {
                480
            };
            for tick in (0..BAR).step_by(hat_step as usize) {
                if chance(
                    &mut rng,
                    if config.style == Style::LoFi {
                        0.86
                    } else {
                        0.95
                    },
                ) {
                    let accent = tick % PPQ == 0;
                    if config.enabled[1] {
                        add(
                            &mut hits,
                            1,
                            offset + tick,
                            velocity(
                                &mut rng,
                                if accent { 65 } else { 40 },
                                if accent { 100 } else { 75 },
                            ),
                        );
                    }
                }
            }
            if config.enabled[2] && chance(&mut rng, (0.18 + config.density * 0.25) * fast) {
                add(&mut hits, 2, offset + BAR - 480, velocity(&mut rng, 65, 96));
            }
            if config.enabled[3] && chance(&mut rng, 0.22) {
                add(
                    &mut hits,
                    3,
                    offset + PPQ * 2 - 240,
                    velocity(&mut rng, 45, 75),
                );
            }
            if config.enabled[4] && chance(&mut rng, config.ghost_probability * fast) {
                add(
                    &mut hits,
                    4,
                    offset + snare_beats[0] * PPQ - 240,
                    velocity(&mut rng, 28, 59),
                );
            }
            if config.enabled[5] && chance(&mut rng, variation * 0.5) {
                add(&mut hits, 5, offset + BAR - 240, velocity(&mut rng, 65, 95));
            }
            if config.enabled[1]
                && chance(
                    &mut rng,
                    config.roll_probability * fast * (if transition { 1.4 } else { 0.7 }),
                )
            {
                let start = offset + BAR - 480;
                hits.retain(|h| !(h.instrument == 1 && h.tick >= start && h.tick < offset + BAR));
                let step = if config.style == Style::Trap {
                    [60, 80, 120][rng.gen_range(0..3)]
                } else {
                    120
                };
                for tick in (start..offset + BAR).step_by(step as usize) {
                    add(&mut hits, 1, tick, velocity(&mut rng, 48, 95));
                    hits.last_mut().unwrap().duration = step.min(90);
                }
            }
            if transition && chance(&mut rng, config.fill_probability) {
                for (i, inst) in [7, 8, 9, 4].iter().enumerate() {
                    if config.enabled[*inst] && chance(&mut rng, 0.55 + config.complexity * 0.3) {
                        add(
                            &mut hits,
                            *inst,
                            offset + BAR - 480 + i as u32 * 120,
                            velocity(&mut rng, 58, 102),
                        );
                    }
                }
            }
        }
        // Swing delays alternate eighth-note positions. Humanization is bounded by the bar.
        for h in &mut hits {
            let bar_start = h.tick / BAR * BAR;
            let local = h.tick - bar_start;
            let swing = if local % 480 == 0 && local % 960 == 480 {
                (config.swing * 160.) as i32
            } else {
                0
            };
            let scale = match h.instrument {
                0 => 0.55,
                4 | 5 => 0.8,
                1..=3 => 1.,
                _ => 0.9,
            };
            let max = (config.humanization * 25. * scale * fast) as i32;
            let jitter = if max > 0 {
                rng.gen_range(-max..=max)
            } else {
                0
            };
            let shifted = (local as i32 + swing + jitter).clamp(0, BAR as i32 - 1) as u32;
            h.tick = bar_start + shifted;
            h.duration = h.duration.min(config.bars as u32 * BAR - h.tick).max(1);
        }
        hits.retain(|h| config.enabled[h.instrument]);
        for hit in &mut hits {
            let range = config.velocity_ranges[hit.instrument];
            hit.velocity = hit.velocity.clamp(range[0], range[1]);
        }
        hits.sort_by_key(|h| (h.tick, h.instrument));
        // At the same tick, closed, open and pedal hats are exclusive; open wins.
        for tick in hits
            .iter()
            .filter(|h| h.instrument == 2)
            .map(|h| h.tick)
            .collect::<Vec<_>>()
        {
            hits.retain(|h| {
                !((h.instrument == 1 || h.instrument == 3) && h.tick.abs_diff(tick) < 120)
            });
        }
        Ok(Self { config, hits })
    }
    pub fn variation(&self, seed_text: &str) -> Result<Self, String> {
        let mut result = self.clone();
        result.config.seed_text = seed_text.to_owned();
        let mut rng = ChaCha8Rng::from_seed(result.config.seed_digest());
        for bar in 0..result.config.bars as u32 {
            let start = bar * BAR;
            if result.config.enabled[0] && chance(&mut rng, result.config.variation) {
                let choices = [720, 1440, 2640, 3360];
                let tick = start + choices[rng.gen_range(0..choices.len())];
                if !result
                    .hits
                    .iter()
                    .any(|h| h.instrument == 0 && h.tick.abs_diff(tick) < 120)
                {
                    add(&mut result.hits, 0, tick, velocity(&mut rng, 75, 105));
                }
            }
            if result.config.enabled[4]
                && chance(
                    &mut rng,
                    result.config.ghost_probability * result.config.variation,
                )
            {
                let tick = start + 1680;
                if !result
                    .hits
                    .iter()
                    .any(|h| h.instrument == 4 && h.tick.abs_diff(tick) < 120)
                {
                    add(&mut result.hits, 4, tick, velocity(&mut rng, 25, 55));
                }
            }
            for hit in result
                .hits
                .iter_mut()
                .filter(|h| h.tick >= start && h.tick < start + BAR)
            {
                if chance(&mut rng, result.config.variation * 0.4) {
                    hit.velocity =
                        (hit.velocity as i16 + rng.gen_range(-10..=10)).clamp(1, 127) as u8;
                }
            }
        }
        for hit in &mut result.hits {
            let range = result.config.velocity_ranges[hit.instrument];
            hit.velocity = hit.velocity.clamp(range[0], range[1]);
        }
        result.hits.sort_by_key(|h| (h.tick, h.instrument));
        Ok(result)
    }
    pub fn midi(&self, instrument: Option<usize>) -> Result<Vec<u8>, String> {
        let mut writer = MidiFile::with_channel(self.config.channel);
        for h in self
            .hits
            .iter()
            .filter(|h| instrument.map_or(true, |i| i == h.instrument))
        {
            writer.add_note_ticks(
                self.config.notes[h.instrument],
                h.tick,
                h.duration,
                h.velocity,
            );
        }
        let mut track = vec![
            TrackEvent {
                delta: 0.into(),
                kind: TrackEventKind::Meta(MetaMessage::Tempo(
                    (60_000_000u32 / self.config.bpm as u32).into(),
                )),
            },
            TrackEvent {
                delta: 0.into(),
                kind: TrackEventKind::Meta(MetaMessage::TimeSignature(4, 2, 24, 8)),
            },
        ];
        let mut last = 0;
        for event in writer.finalize() {
            let absolute = last + event.delta.as_int();
            last = absolute;
            track.push(event);
        }
        let remaining = self.config.bars as u32 * BAR - last;
        track.push(TrackEvent {
            delta: remaining.into(),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });
        let smf = Smf {
            header: Header {
                format: Format::SingleTrack,
                timing: Timing::Metrical((PPQ as u16).into()),
            },
            tracks: vec![track],
        };
        let mut bytes = Vec::new();
        smf.write(&mut bytes).map_err(|e| e.to_string())?;
        Ok(bytes)
    }
    pub fn preview(&self) -> String {
        let mut output = format!(
            "{} BPM | {:?} | seed {}\n",
            self.config.bpm,
            self.config.style,
            self.config.seed_label()
        );
        for bar in 0..self.config.bars as u32 {
            output.push_str(&format!("Bar {}\n", bar + 1));
            for (i, name) in NAMES.iter().enumerate() {
                if !self.config.enabled[i] {
                    continue;
                }
                let steps: String = (0..16)
                    .map(|s| {
                        if self.hits.iter().any(|h| {
                            h.instrument == i && h.tick / BAR == bar && (h.tick % BAR) / 240 == s
                        }) {
                            '●'
                        } else {
                            '·'
                        }
                    })
                    .collect();
                output.push_str(&format!("{name:14} {steps}\n"));
            }
        }
        output.push_str("Events (exact ticks):\n");
        for h in &self.hits {
            output.push_str(&format!(
                "{} {} {} {}\n",
                NAMES[h.instrument], h.tick, h.duration, h.velocity
            ));
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn subdivisions_and_reproducibility() {
        assert_eq!(
            [PPQ, PPQ / 2, PPQ / 4, PPQ / 8, PPQ / 16, PPQ / 3],
            [960, 480, 240, 120, 60, 320]
        );
        for bpm in [60, 75, 90, 110, 140, 160, 180] {
            for bars in [1, 4, 8, 64] {
                let c = Config {
                    bpm,
                    bars,
                    seed: 456,
                    ..Config::default()
                };
                let a = Pattern::generate(c.clone()).unwrap();
                let b = Pattern::generate(c).unwrap();
                assert_eq!(a.hits, b.hits);
                assert!(a
                    .hits
                    .iter()
                    .all(|h| h.tick + h.duration <= bars as u32 * BAR
                        && h.duration > 0
                        && h.velocity > 0));
                assert!(a.hits.windows(2).all(|w| w[0].tick <= w[1].tick));
            }
        }
    }
    #[test]
    fn text_seed_uses_sha256_and_chacha8() {
        let c = Config {
            seed_text: "abc".into(),
            seed: 7,
            ..Config::default()
        };
        assert_eq!(
            c.seed_digest(),
            [
                0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
                0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
                0xf2, 0x00, 0x15, 0xad
            ]
        );
        let a = Pattern::generate(c.clone()).unwrap();
        let b = Pattern::generate(Config {
            seed: 999,
            ..c.clone()
        })
        .unwrap();
        assert_eq!(a.hits, b.hits);
        let d = Pattern::generate(Config {
            seed_text: "abd".into(),
            ..c
        })
        .unwrap();
        assert_ne!(a.hits, d.hits);
    }
    #[test]
    fn midi_round_trip_and_mapping() {
        let c = Config {
            seed: 15,
            bars: 4,
            ..Config::default()
        };
        let p = Pattern::generate(c).unwrap();
        let bytes = p.midi(None).unwrap();
        let smf = Smf::parse(&bytes).unwrap();
        assert_eq!(smf.header.timing, Timing::Metrical((PPQ as u16).into()));
        let mut tick = 0;
        let mut on = 0;
        let mut tempo = false;
        for event in &smf.tracks[0] {
            tick += event.delta.as_int();
            match event.kind {
                TrackEventKind::Midi {
                    channel,
                    message: midly::MidiMessage::NoteOn { key, vel },
                } if vel.as_int() > 0 => {
                    assert_eq!(channel.as_int(), 9);
                    assert!(p.config.notes.contains(&key.as_int()));
                    on += 1;
                }
                TrackEventKind::Meta(MetaMessage::Tempo(value)) => tempo = value.as_int() == 666666,
                _ => {}
            }
        }
        assert_eq!(on, p.hits.len());
        assert_eq!(tick, 4 * BAR);
        assert!(tempo);
        let kick_bytes = p.midi(Some(0)).unwrap();
        let kick = Smf::parse(&kick_bytes).unwrap();
        assert_eq!(
            kick.tracks[0]
                .iter()
                .filter(|e| matches!(
                    e.kind,
                    TrackEventKind::Midi {
                        message: midly::MidiMessage::NoteOn { .. },
                        ..
                    }
                ))
                .count(),
            p.hits.iter().filter(|h| h.instrument == 0).count()
        );
    }
    #[test]
    fn mutation_changes_pattern() {
        let p = Pattern::generate(Config::default()).unwrap();
        assert_ne!(p.hits, p.variation("variation-123").unwrap().hits);
    }
}
