static PPQ: u32 = 96;

use std::time::SystemTime;

use rand::{distributions::uniform::{SampleRange, SampleUniform}, rngs::StdRng, Rng, SeedableRng};

/**
 * Convert beats to ticks
 */
pub fn beats(amount: f64) -> u32 {
    return (amount * (PPQ as f64)).round() as u32;
}

pub fn add_octaves(n: i16, octaves: i16) -> u8 {
    return (n + octaves * 12) as u8;
}

#[derive(Debug)]
pub struct MathMagician {
    noggin: StdRng
}

impl MathMagician {
    /// Shares hash with the math magician. The math magician's calculations 
    /// will be influenced by the hash.
    pub fn share_hash(hash: [u8; 32]) -> Self {
        return MathMagician { noggin: StdRng::from_seed(hash.try_into().unwrap_or([0; 32])) };
    }
    /// Math magician cleverly picks a number between min and max, inclusive.
    /// 
    /// I know that it is possible to use generics to make this function an 
    /// alias of `gen_range` where `0..=11` could be a valid input. However, 
    /// changing this would be a breaking change and would cause all outputs to 
    /// be different.
    pub fn big_decision(&mut self, min: u16, max: u16) -> u16
    {
        return self.noggin.gen_range(min..=max);
    }
    /**
     * Math magician picks a note between 0 and 11, inclusive
     */
    pub fn pick_note(&mut self) -> u16 {
        return self.noggin.gen_range(0..=11);
    }
}

pub fn time_ms() -> u128 {
    return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
}