# temple-godspeak

A standalone Rust library and a small WebAssembly asset for Terry Davis's
TempleOS word oracle. It runs the original GodBitsIns, GodBits, and GodWordStr
HolyC routines compiled to WASM, with TempleOS services replaced by a small host.
It produces random word sequences suitable for seed text, not grammatical prose.

## Rust reuse

Copy this directory to another project, or use a path dependency:

```toml
[dependencies]
temple-godspeak = { path = "../midimachine/crates/temple-godspeak" }
```

```rust
use temple_godspeak::{Oracle, entropy_samples};
let oracle = Oracle::new()?;
// Example deterministic input; provide fresh OS randomness or event timing
// in a real application. Each sample contributes bits 4 through 27.
let samples = vec![0x12345678_u64; entropy_samples(16)];
let seed = oracle.sentence(16, &samples)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

The default `runtime` feature uses the wasmi interpreter, with a fuel limit.
The crate doesn't choose an RNG: the caller supplies raw entropy samples.
Repeated samples produce identical text. Counts outside 1–128 and insufficient
entropy return errors. Each call starts with an empty bit FIFO.

For a custom WASM host, use `default-features = false` and the `WASM` and
`VOCABULARY` constants. No JavaScript or compiler is needed to build/use the
Rust crate: the generated assets are included.

## Rebuild the HolyC asset

Requires Node.js and Binaryen's wasm-opt. Compiler and original sources are
vendored/pinned; rebuilding does not fetch from the network.

```sh
bash build_wasm.sh
cargo test
node tools/test.mjs
# In this repository, also install the browser assets:
bash build_wasm.sh ../../frontend/public
```

MidiMachine's `musicgen/build_wasm.sh` invokes this build automatically.
The JS browser adapter is `frontend/src/lib/godspeak.js` in the parent repository.
Run its integration check from the repository root with:
`bun frontend/scripts/testOracle.mjs`.

## WASM host contract

Only import: `env.HostPick() -> i64`, returning raw timestamp-like/random bits.
Exports use i64/BigInt (including pointers). Call `__rt_init` once. Copy NUL-ended
UTF-8 words into `OracleVocabAddress()` (100,000-byte buffer), and write their
64-bit little-endian pointers into `OracleWordsAddress()` (8,000 slots). Call
`OracleInit(word_count)` to set the vocabulary and reset the FIFO. Each
`OracleNext()` returns a pointer to a selected NUL-ended word. The memory is
compiler-managed (32 MiB initial); do not overwrite other locations. Generate
only bounded numbers of words and ensure the host can provide enough samples.

The browser uses Web Crypto rather than TempleOS's keyboard timer. See
[PROVENANCE.md](PROVENANCE.md) for exact sources, the precedence compatibility
fix, and all host adaptations. This crate has not been published to crates.io.
