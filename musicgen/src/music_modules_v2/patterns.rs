//! Miscellaneous code for patterns.

#[deprecated(note = "This module is too restrictive. Users should be able to pick chords in any order... all that time... wasted")]
pub mod old_validation {
    /// The maximum digit that can be fed into the pattern validation.
    const MAX: usize = 16;

    /// Validates whether a pattern is of the form:
    /// 
    /// * `X-X-...-X`
    /// 
    /// Where `X` is any number between 1-16, and each time a number is used, all 
    /// of the previous digits must have already appeared.
    pub fn validate_pattern(input: &str) -> bool {
        let mut set = LimitedSet::<MAX>::new();
        let mut chars = input.chars();
        loop {
            let mut n = 0;
            'inner: loop {
                let next = match chars.next() {
                    Some(c) => c,
                    None => {
                        if n == 0 {
                            return false;
                        }
                        return set.update(n);
                    }
                };
                if next == '-' {
                    // number is complete, process n
                    if !set.update(n) {
                        return false;
                    }
                    break 'inner;
                }
                if (next as u8) < '0' as u8 {
                    return false;
                }
                let digit = next as u8 - '0' as u8;
                if digit > 9 {
                    return false;
                }
                n = n * 10 + digit;
            }
        }
    }

    /// A hashset-type structure that keeps track of the numbers we come across.
    /// This is slightly modified to check if we have come across 1..n-1 already.
    /// This could probably be improved to `O(1)` space by just keeping track of 
    /// the highest number we have come across, but I like the idea of only using
    /// two bytes to keep track of the numbers. It's good enough. It's about 
    /// `O(n/8)` space where `n` = 16.
    struct LimitedSet<const NUMBER_OF_DIGITS: usize>
    where [(); (NUMBER_OF_DIGITS >> 3) + ((NUMBER_OF_DIGITS & 0b111) > 0) as usize]:
    {
        data: [u8; (NUMBER_OF_DIGITS >> 3) + ((NUMBER_OF_DIGITS & 0b111) > 0) as usize]
    }

    impl<const NUMBER_OF_DIGITS: usize> LimitedSet<NUMBER_OF_DIGITS> 
    where [(); (NUMBER_OF_DIGITS >> 3) + ((NUMBER_OF_DIGITS & 0b111) > 0) as usize]:
    {
        /// Creates a new `LimitedSet`.
        fn new() -> Self {
            Self {
                data: [0u8; (NUMBER_OF_DIGITS >> 3) + ((NUMBER_OF_DIGITS & 0b111) > 0) as usize]
            }
        }

        /// Updates the `LimitedSet` with the input `n`. If `1..n` digits have 
        /// not been fed into it, it will return false, otherwise true.
        fn update(&mut self, n: u8) -> bool {
            if n == 1 {
                self.data[0] |= 0b1;
                return true;
            }
            let byte = ((n - 1) >> 3) as usize;
            let bit = (n - 1) & 0b111;

            let mask = 1 << (bit - 1);
            if self.data[byte] & mask == 0 {
                return false;
            }
            let byte = (n >> 3) as usize;
            let bit = n & 0b111;
            let mask = 1 << (bit - 1);
            self.data[byte] |= mask;
            return true;
        }
    }

    #[cfg(test)]
    #[allow(deprecated)]
    mod tests {
        use super::*;

        #[test]
        fn basic_validation() {
            let tests = [
                ("1-2-3-4", true),
                ("1-1-1-1", true),
                ("1-2-3-2", true),
                ("1-2-3-4-5-6-2-2-2-1-4", true),
                ("2-3-4-1", false),
                ("3-4-5-6", false),
                ("1,2,4,3", false),
                ("1,2,2,4", false),
            ];
            for (input, output) in tests {
                let actual = validate_pattern(input);
                assert!(actual == output, "Test failed; input = '{}'; expected = {}; actual = {}", input, output, actual)
            }
        }

        #[test]
        fn iterators() {
            let input = "1-2-3-4";
            let mut chars = input.chars();
            assert_eq!(chars.next().unwrap(), '1');
        }

        #[test]
        fn sets() {
            let mut set = LimitedSet::<16>::new();
            assert_eq!(set.update(2), false);
            assert_eq!(set.update(1), true);
            assert_eq!(set.update(2), true);
            assert_eq!(set.update(4), false);
        }
    }
}

pub mod validation {
    pub fn validate_pattern(input: &str) -> (bool, Vec<u8>) {
        let mut result = Vec::with_capacity(input.len() / 2 + 5);
        let mut chars = input.chars();
        loop {
            let mut n: u8 = 0;
            'inner: loop {
                let next = match chars.next() {
                    Some(c) => c,
                    None => {
                        result.push(n);
                        return (n != 0, result)
                    }
                };
                if next == '-' {
                    if n == 0 {
                        return (false, result);
                    }
                    result.push(n);
                    break 'inner;
                }
                if (next as u8) < '0' as u8 {
                    return (false, result);
                }
                let digit = next as u8 - '0' as u8;
                if digit > 9 {
                    return (false, result);
                }
                n = n * 10 + digit;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_1() {
            let data = [
                ("1-1-1-1", true),
                ("2-2-2-2", true),
                ("1-2-1-2", true),
                ("1-2-3-4", true),
                ("4-3-2-1", true),
                ("0-1-2-3", false),
                ("-1-2-3--4", false),
                ("-1-2-3", false),
                ("1-2-3--", false),
            ];

            for (input, output) in data {
                assert_eq!(validate_pattern(input).0, output)
            }
        }

        #[test]
        fn test_2() {
            let data = [
                ("1-2-3-4", vec![1,2,3,4]),
                ("2-3-4-5", vec![2,3,4,5]),
                ("2-3-2-1", vec![2,3,2,1]),
                ("1-2-3-4-1-2-3-5", vec![1,2,3,4,1,2,3,5]),
            ];

            for (input, output) in data {
                assert_eq!(validate_pattern(input).1, output);
            }
        }
    }
}