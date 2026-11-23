# Midi Machine

## Setup

* Install `Bun`
* Install `Rust`

Install dependencies: 

```sh
cd frontend
bun install
```

## Building the Rust WASM code

```sh
cd musicgen
chmod +x build_wasm.sh
./build_wasm.sh
```

After building the Rust code, the `musicgen/pkg/musicgen_bg.wasm` file will need to be moved to `frontend/public/` for changes to take effect. If you changed the function signature of the WASM function, then you will also need to move the `musicgen/pkg/musicgen.js` file to the same folder.

## Running the web server locally

```sh
cd frontend
bun run dev
```

## Generating test data

To prevent breaking changes to the default configurations, we can generate MIDI files to test the output against. It's somewhat of a mutation test.

Run this command to generate the midi files:

```sh
cargo run --bin generate_midi
```

Run this command to determine whether or not the output has changed:

```sh
cargo test
```

## Contributions

Any contributions will be licensed under the MIT license, unless specified otherwise.

PRs with breaking changes to the default configurations will not be merged. If you want to push breaking changes, either create a new:

* chord placement method
* chord group
* `ChordType` definition(s) under the `custom` chord group

## Hip-hop drum generator

Open the **Drums** tab in the web app. Choose BPM, style, bars, groove settings, active instruments, GM note mappings, and seed text. **Generate / regenerate** creates a playable full pattern and each enabled drum layer, with a full-pattern MIDI download button and a precise tick event list below the step preview. Reusing the same settings and seed text reproduces the pattern; **New pattern** selects new seed text. The full pattern and each enabled drum layer have soundfont MIDI playback controls. The browser can also export a ZIP batch containing the full pattern and a MIDI file for every enabled instrument, with optional metadata.

For streaming batch generation, create a JSON config (omitted fields use defaults):

```json
{"bpm":90,"bars":8,"style":"boom_bap","seed_text":"my drum idea","metadata":true}
```

```sh
cd musicgen
RUSTC_BOOTSTRAP=1 cargo run --release --bin drum_batch -- config.json ./drum_patterns 1000 --zip
```

`COUNT` may be omitted for one pattern. The command writes each pattern before generating the next, reports progress every 100 patterns, derives reproducible text seeds from the base text and pattern index, and refuses to overwrite existing files. `--individual` writes a full MIDI file and one file per enabled instrument for each pattern. `--zip` includes every enabled instrument file, manifest, and optional metadata, using the system `zip` executable after generation. For large batches, allow sufficient disk space for both the directory and its archive.

The files use 960 ticks per quarter note, 4/4, the requested tempo, and GM percussion channel 10 by default. In FL Studio, import `full_pattern.mid` into a Piano Roll or import individual instrument files onto separate channels. The import retains each instrument's position within the full pattern. FL Studio does not automatically route GM drum notes to separate Channel Rack instruments; set the note mappings in the Drums tab or JSON configuration for your instrument setup. The text preview uses a 16-step grid plus a complete tick event list for rolls and humanized timing.

Current limits: 4/4 only; browser ZIP export holds the archive parts in memory, so use the CLI for very large batches or a chosen output directory. ZIP export requires the `zip` command installed on the host. The existing Rust crate uses `generic_const_exprs` and currently needs `RUSTC_BOOTSTRAP=1` on stable Rust.

The drum generator hashes UTF-8 seed text with SHA-256 and initializes `ChaCha8Rng` from the 32-byte digest. The original chord generator also hashes its input with SHA-256; its existing RNG remains unchanged to preserve prior outputs. The drum preview uses the app’s existing SGM Plus soundfont URL and GM percussion channel 10 for the full pattern and each enabled layer.
