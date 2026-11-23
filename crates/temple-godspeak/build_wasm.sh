#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
node tools/compile.mjs
wasm-opt assets/oracle.wasm -o assets/oracle.wasm -Oz --enable-bulk-memory --enable-sign-ext --enable-nontrapping-float-to-int
node tools/test.mjs
# Optional destination, so this crate also builds independently of MidiMachine.
if [ "$#" -gt 0 ]; then
  mkdir -p "$1"
  cp assets/oracle.wasm "$1/godspeak.wasm"
  cp assets/vocabulary.txt "$1/godspeak-vocabulary.txt"
fi
