static ppq: u32 = 96;

use rand::{Rng, SeedableRng, rngs::StdRng};

/**
 * Convert beats to ticks
 */
pub fn beats(amount: f32) -> u32 {
    return (amount * (ppq as f32)).round() as u32;
}

#[derive(Debug)]
pub struct MathMagician {
    noggin: StdRng
}

impl MathMagician {
    pub fn share_hash(hash: Vec<u8>) -> Self {
        return MathMagician { noggin: StdRng::from_seed(hash.try_into().unwrap_or([0; 32])) };
    }
    pub fn big_decision(&mut self, min: u16, max: u16) -> u16 {
        return self.noggin.gen_range(min..=max);
    }
    pub fn pick_note(&mut self) -> u16 {
        return self.noggin.gen_range(0..=11);
    }
}