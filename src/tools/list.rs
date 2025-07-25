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

    println!("Installed commands:\n");

    // Collect all installed scripts in a Vec<PathBuf>
    let installed_scripts: Vec<PathBuf> = fs::read_dir(&install_dir)
        .expect("Failed to read install directory")
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() { Some(path) } else { None }
        })
        .collect();

    // Helper to canonicalize path or fallback to original
    let canonicalize_or = |p: &Path| fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());

    // Build a map from installed script abs paths to their filenames for easy lookup
    use std::collections::HashMap;
    let mut script_map = HashMap::new();
    for script_path in &installed_scripts {
        let abs_path = canonicalize_or(script_path);
        script_map.insert(abs_path, script_path.file_name().unwrap().to_string_lossy().into_owned());
    }

    // Scan /usr/bin and /usr/local/bin symlinks, map target → symlink path
    let mut alias_map = HashMap::new();

    for bin_dir in ["/usr/bin", "/usr/local/bin"] {
        if let Ok(entries) = fs::read_dir(bin_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_symlink() {
                    if let Ok(target) = fs::read_link(&path) {
                        // Resolve target relative to symlink parent dir
                        let target_abs = canonicalize_or(&path.parent().unwrap().join(&target));

                        alias_map.insert(target_abs, path);
                    }
                }
            }
        }
    }

    // For each installed script, print its filename and the symlink alias if any
    for script_path in installed_scripts {
        let abs_script = canonicalize_or(&script_path);

        if let Some(alias_path) = alias_map.get(&abs_script) {
            println!("{} -> {}", script_path.file_name().unwrap().to_string_lossy(), alias_path.display());
        } else {
            println!("{} -> (no symlink found)", script_path.file_name().unwrap().to_string_lossy());
        }
    }
}
