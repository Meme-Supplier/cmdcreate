use crate::tools::utils::*;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub fn search() {
    let args = return_args();

    if let Some(name) = args.get(1) {
        let install_dir = dirs::home_dir()
            .expect("Home dir not found")
            .join(".local/share/cmdcreate/files");

        if !install_dir.exists() {
            println!("No installed commands found (directory doesn't exist)");
            return;
        }

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

        if installed_scripts.is_empty() {
            println!("No installed commands found in: {}", install_dir.display());
            return;
        }

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

        let mut i: i16 = 0;

        for script in installed_scripts {
            let file_stem = script.file_stem().unwrap_or_default().to_string_lossy();

            if file_stem.contains(name) {
                println!("{file_stem}");
                i += 1;
            }
        }

        if i == 0 {
            println!("No installed commands found contain: \"{name}\"")
        }
    } else {
        crate::display_usage();
    }
}
