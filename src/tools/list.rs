use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn list() {
    let install_dir = dirs::home_dir()
        .expect("Home dir not found")
        .join(".local/share/cmdcreate/files");

    if !install_dir.exists() {
        println!("No installed commands found (directory doesn't exist)");
        return;
    }

    // Collect all installed scripts
    let installed_scripts: Vec<PathBuf> = fs::read_dir(&install_dir)
        .expect("Failed to read install directory")
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Check if directory exists but has no files
    if installed_scripts.is_empty() {
        println!("No installed commands found in: {}", install_dir.display());
        return;
    }

    println!("Installed commands:\n");

    let canonicalize_or = |p: &Path| fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    let mut script_map = HashMap::new();

    for script_path in &installed_scripts {
        let abs_path = canonicalize_or(script_path);
        script_map.insert(
            abs_path,
            script_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        );
    }

    for script in installed_scripts {
        let file_stem = script.file_stem().unwrap_or_default().to_string_lossy();

        println!("{file_stem}")
    }
}
