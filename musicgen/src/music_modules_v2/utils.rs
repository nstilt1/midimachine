static PPQ: u32 = 96;

use std::{collections::HashSet, hash::Hash, ops::Add, time::SystemTime};

use rand::{rngs::StdRng, Rng, SeedableRng};

use super::music::KEYS;

/**
 * Convert beats to ticks
 */
pub fn beats(amount: f64) -> u32 {
    return (amount * (PPQ as f64)).round() as u32;
}

#[allow(unused)]
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
    pub fn pick_note(&mut self) -> i16 {
        return self.noggin.gen_range(0..=11);
    }
}

#[allow(unused)]
pub fn time_ms() -> u128 {
    return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
}

/// Returns the index of the `note_lengths` array where `note_lengths[i] == 4.0 
/// \- total_time`.
/// 
/// `note_lengths = [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0]`
/// 
/// The original way that I did this was like so:
/// 
/// `return 6 - note_lengths.iter().position(|&r| r == total_time).unwrap() as i32;`
/// 
/// This could be done faster by calculating the index.
/// ```ignore
/// note_lengths(i) = 0.5 * i + 0.5
/// note_lengths(i) - 0.5 = 0.5 * i;
/// (note_lengths(i) - 0.5) / 0.5 = i
/// 2*note_lengths(i) - 1 = i
/// 
/// i = 2 * note_lengths(i) - 1
/// ```
pub fn get_max_note_length_index(total_time: f64) -> u16 {
    let max_value = 4f64 - total_time;
    return (max_value * 2f64 - 1f64) as u16;
}

/// The key is expected to be of this form
/// 
/// ```txt
/// Dmaj
/// D#maj
/// Bmin
/// ```
/// 
/// The keys cannot be flat, such as `Bb` or `Eb`.
pub fn parse_key(key: &str) -> i16 {
    if key.ne("random") {
        let is_sharp = key.chars().nth(1).unwrap_or(' ') == '#';
        for (i, k) in KEYS.iter().enumerate() {
            let len = k.len();
            if key[..len] == **k {
                let is_major = key[len..] == *"maj";
                return  (i as i16 + is_major as i16 * 3 + is_sharp as i16) % 12;
            }
        }
    }
    return 0;
}

/// Math operators for HashSets
pub trait HashSetMath {
    type Inner: Eq + Hash + Clone;
    fn add(&self, other: &Self) -> Self;
    fn add_assign(&mut self, other: &Self);
    fn intersect(&self, other: &Self) -> Vec<Self::Inner>;
    fn sub(&self, other: &Self) -> Self;
    fn sub_assign(&mut self, other: &Self);
    fn to_vec(&self) -> Vec<Self::Inner>;
}

impl<T: Eq + Hash + Clone> HashSetMath for HashSet<T> {
    type Inner = T;
    fn add(&self, other: &Self) -> Self {
        self.union(other).cloned().collect()
    }
    fn add_assign(&mut self, other: &Self) {
        *self = self.add(other);
    }
    fn intersect(&self, other: &Self) -> Vec<Self::Inner> {
        self.intersection(other).cloned().collect()
    }
    fn sub(&self, other: &Self) -> Self {
        self.difference(other).cloned().collect()
    }
    fn sub_assign(&mut self, other: &Self) {
        *self = self.sub(other);
    }
    fn to_vec(&self) -> Vec<Self::Inner> {
        self.iter().cloned().collect()
    }
}

pub trait CustomIterators {
    type Item;
    
    /// Returns an iterator that transposes each element by the given amount,
    /// wrapping around at 12
    fn transpose(&self, transpose_amount: i16) -> impl Iterator<Item = Self::Item> + '_;
}

impl CustomIterators for [usize] {
    type Item = usize;
    
    fn transpose(&self, transpose_amount: i16) -> impl Iterator<Item = Self::Item> + '_ {
        self.iter().map(move |&n| (n + transpose_amount as usize) % 12)
    }
}

impl CustomIterators for [i16] {
    type Item = i16;
    
    fn transpose(&self, transpose_amount: i16) -> impl Iterator<Item = Self::Item> + '_ {
        self.iter().map(move |&n| (n + transpose_amount as i16) % 12)
    }
}

impl CustomIterators for [i32] {
    type Item = i32;
    
    fn transpose(&self, transpose_amount: i16) -> impl Iterator<Item = Self::Item> + '_ {
        self.iter().map(move |&n| (n + transpose_amount as i32) % 12)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn hashset_math() {
        let a = HashSet::from([1, 2, 3]);
        let b = HashSet::from([3, 4, 5]);

        assert_eq!(a.sub(&b), HashSet::from([1, 2]));
        assert_eq!(b.sub(&a), HashSet::from([4, 5]));

        assert_eq!(a.add(&b), b.add(&a));
        assert_eq!(a.add(&b), HashSet::from([1, 2, 3, 4, 5]));
    }

    #[test]
    fn transpose_test() {
        let test_1: [i32; 7] = [0, 2, 4, 6, 8, 9, 11];
        let transposed: Vec<i32> = test_1.transpose(1).collect();
        assert_eq!(transposed, [1, 3, 5, 7, 9, 10, 0]);

        let set: HashSet<i32> = HashSet::from_iter([0, 3, 6, 9].transpose(2));
        assert_eq!(set, HashSet::from([2, 5, 8, 11]));
    }
}