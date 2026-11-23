use musicgen::drums::{Config, Pattern, NAMES};
use std::{
    env,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
fn save(path: PathBuf, bytes: &[u8]) -> io::Result<()> {
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(bytes)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: drum_batch CONFIG.json OUTPUT_DIR [COUNT] [--zip] [--individual]");
        std::process::exit(2);
    }
    let mut config: Config = serde_json::from_slice(&fs::read(&args[1])?)?;
    config.validate()?;
    let count: u32 = args
        .get(3)
        .filter(|s| !s.starts_with("--"))
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(1);
    if count == 0 {
        return Err("COUNT must be positive".into());
    }
    let zip = args.iter().any(|a| a == "--zip");
    let individual = zip || config.individual_files || args.iter().any(|a| a == "--individual");
    let output = PathBuf::from(&args[2]);
    if zip && output.with_extension("zip").exists() {
        return Err("ZIP already exists".into());
    }
    if output.exists() && fs::read_dir(&output)?.next().is_some() {
        return Err("Output directory must be empty".into());
    }
    fs::create_dir_all(&output)?;
    let base_seed = config.seed;
    let base_text = config.seed_text.clone();
    let mut manifest = serde_json::json!({"generator_version":env!("CARGO_PKG_VERSION"),"generated_unix_seconds":SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),"config":config,"count":count,"seed_labels":[]});
    for index in 0..count {
        if base_text.is_empty() {
            config.seed = base_seed.wrapping_add(index as u64);
        } else {
            config.seed_text = if index == 0 {
                base_text.clone()
            } else {
                format!("{base_text}#{:06}", index + 1)
            };
        }
        let pattern = Pattern::generate(config.clone())?;
        let folder = if individual {
            let p = output.join(format!("pattern_{:06}", index + 1));
            fs::create_dir(&p)?;
            p
        } else {
            output.clone()
        };
        let name = if individual {
            "full_pattern.mid".to_string()
        } else {
            format!(
                "{}_{:?}_{:03}bpm_{:06}.mid",
                config.filename_prefix,
                config.style,
                config.bpm,
                index + 1
            )
            .to_lowercase()
        };
        save(folder.join(name), &pattern.midi(None)?)?;
        if individual {
            for (i, label) in NAMES.iter().enumerate() {
                if config.enabled[i] {
                    save(folder.join(format!("{label}.mid")), &pattern.midi(Some(i))?)?;
                }
            }
        }
        if config.metadata {
            save(
                folder.join(if individual {
                    "metadata.json".to_string()
                } else {
                    format!("pattern_{:06}.json", index + 1)
                }),
                &serde_json::to_vec_pretty(&config)?,
            )?;
        }
        manifest["seed_labels"]
            .as_array_mut()
            .unwrap()
            .push(config.seed_label().into());
        if (index + 1) % 100 == 0 || index + 1 == count {
            eprintln!("Generated {}/{}", index + 1, count);
        }
    }
    save(
        output.join("manifest.json"),
        &serde_json::to_vec_pretty(&manifest)?,
    )?;
    if zip {
        let archive = output.with_extension("zip");
        if archive.exists() {
            return Err("ZIP already exists".into());
        }
        let parent = output.parent().unwrap_or_else(|| std::path::Path::new("."));
        let base = output.file_name().ok_or("Invalid output directory")?;
        let status = Command::new("zip")
            .current_dir(parent)
            .arg("-q")
            .arg("-r")
            .arg(&archive)
            .arg(base)
            .status()?;
        if !status.success() {
            return Err("zip command failed".into());
        }
        eprintln!("Archive: {}", archive.display());
    }
    Ok(())
}
