//! Terry Davis's HolyC oracle, compiled to WASM. The host supplies entropy;
//! this library makes no claim to reproduce TempleOS's hardware timer.
//! Word sequences are random vocabulary selections, not grammatical sentences.
pub const WASM: &[u8] = include_bytes!("../assets/oracle.wasm");
pub const VOCABULARY: &str = include_str!("../assets/vocabulary.txt");
pub const MAX_WORDS: usize = 128;

/// Number of 24-bit GodPick contributions required for 17 bits per word.
pub fn entropy_samples(word_count: usize) -> usize {
    (word_count * 17 + 23) / 24
}

#[cfg(feature = "runtime")]
mod runtime {
    use super::*;
    use std::{collections::VecDeque, fmt};
    use wasmi::{Caller, Config, Engine, Linker, Module, Store};

    #[derive(Debug)]
    pub struct Error(String);
    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }
    impl std::error::Error for Error {}
    impl From<wasmi::Error> for Error {
        fn from(error: wasmi::Error) -> Self {
            Self(error.to_string())
        }
    }

    pub struct Oracle {
        engine: Engine,
        module: Module,
    }
    impl Oracle {
        pub fn new() -> Result<Self, Error> {
            let mut config = Config::default();
            config.consume_fuel(true);
            let engine = Engine::new(&config);
            let module = Module::new(&engine, WASM)?;
            Ok(Self { engine, module })
        }

        /// Generate a repeatable phrase from raw timestamp-like values. The HolyC
        /// GodPick adapter discards four low bits, then GodBitsIns takes 24 bits.
        /// Supply fresh OS randomness or measured event timestamps in production.
        /// Extra samples are ignored. Insufficient entropy returns an error.
        pub fn sentence(&self, count: usize, entropy: &[u64]) -> Result<String, Error> {
            if !(1..=MAX_WORDS).contains(&count) {
                return Err(Error(format!("Choose 1–{MAX_WORDS} words")));
            }
            if entropy.len() < entropy_samples(count) {
                return Err(Error("Insufficient entropy samples".into()));
            }
            let mut store = Store::new(
                &self.engine,
                VecDeque::from(entropy[..entropy_samples(count)].to_vec()),
            );
            store.set_fuel(1_000_000)?;
            let mut linker = Linker::new(&self.engine);
            linker
                .func_wrap(
                    "env",
                    "HostPick",
                    |mut caller: Caller<'_, VecDeque<u64>>| -> Result<i64, wasmi::Error> {
                        caller
                            .data_mut()
                            .pop_front()
                            .map(|v| v as i64)
                            .ok_or_else(|| wasmi::Error::new("Entropy exhausted"))
                    },
                )
                .map_err(|e| Error(e.to_string()))?;
            let instance = linker
                .instantiate(&mut store, &self.module)?
                .start(&mut store)?;
            let memory = instance
                .get_memory(&store, "memory")
                .ok_or_else(|| Error("Missing WASM memory".into()))?;
            instance
                .get_typed_func::<(), ()>(&store, "__rt_init")?
                .call(&mut store, ())?;
            let slots = instance
                .get_typed_func::<(), i64>(&store, "OracleWordsAddress")?
                .call(&mut store, ())? as usize;
            let mut cursor = instance
                .get_typed_func::<(), i64>(&store, "OracleVocabAddress")?
                .call(&mut store, ())? as usize;
            let words: Vec<_> = VOCABULARY.lines().collect();
            for (i, word) in words.iter().enumerate() {
                memory
                    .write(&mut store, slots + i * 8, &(cursor as u64).to_le_bytes())
                    .map_err(|e| Error(e.to_string()))?;
                memory
                    .write(&mut store, cursor, word.as_bytes())
                    .map_err(|e| Error(e.to_string()))?;
                cursor += word.len();
                memory
                    .write(&mut store, cursor, &[0])
                    .map_err(|e| Error(e.to_string()))?;
                cursor += 1;
            }
            instance
                .get_typed_func::<i64, ()>(&store, "OracleInit")?
                .call(&mut store, words.len() as i64)?;
            let next = instance.get_typed_func::<(), i64>(&store, "OracleNext")?;
            let mut output = Vec::with_capacity(count);
            for _ in 0..count {
                let ptr = next.call(&mut store, ())? as usize;
                let data = memory.data(&store);
                let rest = data
                    .get(ptr..)
                    .ok_or_else(|| Error("Invalid word pointer".into()))?;
                let len = rest
                    .iter()
                    .position(|b| *b == 0)
                    .ok_or_else(|| Error("Unterminated word".into()))?;
                output.push(
                    std::str::from_utf8(&rest[..len])
                        .map_err(|e| Error(e.to_string()))?
                        .to_string(),
                );
            }
            Ok(output.join(" "))
        }
    }
}
#[cfg(feature = "runtime")]
pub use runtime::{Error, Oracle};

#[cfg(all(test, feature = "runtime"))]
mod tests {
    use super::*;
    #[test]
    fn original_word_selection_matches_reference_bit_fifo() {
        let entropy: Vec<u64> = (0..entropy_samples(32))
            .map(|i| (i as u64 + 17).wrapping_mul(0x9e3779b97f4a7c15))
            .collect();
        let mut bits = Vec::new();
        for sample in &entropy {
            for b in 0..24 {
                bits.push((sample >> (b + 4)) & 1);
            }
        }
        let vocabulary: Vec<_> = VOCABULARY.lines().collect();
        let expected = bits
            .chunks(17)
            .take(32)
            .map(|chunk| {
                let value = chunk.iter().fold(0, |n, b| (n << 1) + b);
                vocabulary[value as usize % vocabulary.len()]
            })
            .collect::<Vec<_>>()
            .join(" ");
        let oracle = Oracle::new().unwrap();
        assert_eq!(oracle.sentence(32, &entropy).unwrap(), expected);
        assert_eq!(oracle.sentence(32, &entropy).unwrap(), expected);
        assert!(oracle.sentence(0, &entropy).is_err());
        assert!(oracle.sentence(129, &entropy).is_err());
        assert!(oracle.sentence(32, &[]).is_err());
    }
}
