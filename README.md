# Midi Machine

## Setup

* Install `Node`/`npm`
* Install `Rust`

Install dependencies: 

```sh
cd frontend
npm install
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
npm run dev
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