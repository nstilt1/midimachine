# Run cargo tests
cargo test

# Check if the tests were successful
if [ $? -eq 0 ]; then
    echo "No mutations found"
    
    # compile rust code to webassembly
    wasm-pack build --target web --release

    # move the files to the public folder
    mv pkg/musicgen_bg.wasm ../frontend/public/musicgen_bg.wasm
    mv pkg/musicgen.js ../frontend/public/musicgen.js

    echo "Build successful"
else
    echo "Mutation tests failed. Run `cargo test` to see which tests failed."
fi