#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
# The library uses generic_const_exprs and requires a nightly Rust toolchain.
cargo +nightly test generation_mode_mutations
cargo +nightly build --release --target wasm32-unknown-unknown
wasm-bindgen --target web --out-dir ../frontend/public --no-typescript \
  target/wasm32-unknown-unknown/release/musicgen.wasm
wasm-opt ../frontend/public/musicgen_bg.wasm -o ../frontend/public/musicgen_bg.wasm \
  -Oz --enable-bulk-memory --enable-sign-ext --enable-nontrapping-float-to-int
bash ../crates/temple-godspeak/build_wasm.sh ../../frontend/public
echo "Build successful"
