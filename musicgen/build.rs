use std::process::Command;

fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    if target_arch != "wasm32" {
        println!("cargo:rerun-if-changed=src/bin/generate_midi.rs");
        Command::new("cargo")
            .args(&["run", "--bin", "generate_midi"])
            .status()
            .expect("Failed to generate MIDI test data");
    }
}