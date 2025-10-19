use std::{fs, path::PathBuf, process::exit};

use crate::utils::{colors::COLORS, msgs::error, sys::args_contains};

pub fn is_command_installed(cmd: &str) {
    let mut count: i32 = 0;
    for script in retrieve_commands("installed") {
        if script.file_stem().unwrap_or_default().to_string_lossy() == *cmd {
            count += 1
        }
    }

    if count == 0 && !(args_contains("-f") || args_contains("--force")) {
        error("Command not installed:", cmd);
        exit(0)
    }
}

pub fn retrieve_commands(val: &str) -> Vec<PathBuf> {
    let install_dir = dirs::home_dir()
        .unwrap_or_else(|| panic!("{}Error: Home dir not found{}", COLORS.red, COLORS.reset))
        .join(".local/share/cmdcreate/files");

    if !install_dir.exists() {
        error("No commands are installed.", "");
        return Vec::new();
    }

    if val == "dir" {
        return vec![install_dir];
    }

    let installed_scripts: Vec<PathBuf> = fs::read_dir(&install_dir)
        .unwrap_or_else(|_| {
            panic!(
                "{}Error: Failed to read install directory{}",
                COLORS.red, COLORS.reset
            )
        })
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
        error("No commands are installed.", "");
        return Vec::new();
    }

    if val == "installed" {
        installed_scripts
    } else {
        Vec::new()
    }
}
