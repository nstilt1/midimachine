# Run cargo tests
cargo test generation_mode_mutations

# Check if the tests were successful
if [ $? -eq 0 ]; then
    echo "No mutations found"
    
    cargo build --release --target wasm32-unknown-unknown

    wasm-bindgen --target web \
      --out-dir ../frontend/public \
      --no-typescript \
      target/wasm32-unknown-unknown/release/musicgen.wasm

    wasm-opt ../frontend/public/musicgen_bg.wasm \
      -o ../frontend/public/musicgen_bg.wasm \
      -Oz \
      --enable-bulk-memory --enable-sign-ext --enable-nontrapping-float-to-int

    echo "Build successful"


else
    echo "Mutation tests failed. Run `cargo test` to see which tests failed."
fi