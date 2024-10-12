# Midi Machine

## Setup

* Install `Node`/`npm`
* Install `Rust`

Install dependencies: 

```sh
cd frontent-non-ts
npm install
```

## Building the Rust WASM code

```sh
cd musicgen
chmod +x build_wasm.sh
./build_wasm.sh
```

After building the Rust code, the `musicgen/pkg/musicgen_bg.wasm` file will need to be moved to `frontend-non-ts/public/` for changes to take effect. If you changed the function signature of the WASM function, then you will also need to move the `musicgen/pkg/musicgen.js` file to the same folder.

## Running the web server locally

```sh
npm run dev
```