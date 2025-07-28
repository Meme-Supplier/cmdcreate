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

    println!("Installed commands:\n\n<command> <symlink>\n");

    let canonicalize_or = |p: &Path| fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());

    use std::collections::HashMap;
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

    let mut alias_map = HashMap::new();

    for bin_dir in ["/usr/bin"] {
        if let Ok(entries) = fs::read_dir(bin_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_symlink() {
                    if let Ok(target) = fs::read_link(&path) {
                        let target_abs = canonicalize_or(&path.parent().unwrap().join(&target));
                        alias_map.insert(target_abs, path);
                    }
                }
            }
        }
    }

    for script_path in installed_scripts {
        let abs_script = canonicalize_or(&script_path);
        let file_stem = script_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();

        if let Some(alias_path) = alias_map.get(&abs_script) {
            println!("{file_stem} -> {}", alias_path.display());
        } else {
            println!("{file_stem} -> (no symlink found)");
        }
    }
}
