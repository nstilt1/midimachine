wasm-pack build --target web --release

mv pkg/musicgen_bg.wasm ../frontend/public/musicgen_bg.wasm
mv pkg/musicgen.js ../frontend/public/musicgen.js