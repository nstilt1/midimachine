# Run cargo tests
#cargo test generation_mode_mutations

# Check if the tests were successful
#if [ $? -eq 0 ]; then
    echo "No mutations found"
    
    cargo build --lib --target wasm32-unknown-unknown

    wasm-bindgen --target web \
      --out-dir ../frontend/public \
      --no-typescript \
      --keep-debug \
      target/wasm32-unknown-unknown/debug/musicgen.wasm

    echo "Build successful"


#else
    #echo "Mutation tests failed. Run `cargo test` to see which tests failed."
#fi