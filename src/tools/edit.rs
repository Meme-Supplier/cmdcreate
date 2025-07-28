use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::tools::utils::run_shell_command;

pub const SUPPORTED_EDITORS: [&str; 7] = ["nvim", "vim", "nano", "micro", "code", "gedit", "kate"];

fn is_editor_installed(editor: &str) -> bool {
    Command::new("which")
        .arg(editor)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn edit() {
    let args = crate::return_args();

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

    let installed = SUPPORTED_EDITORS
        .iter()
        .find(|&&ed| is_editor_installed(ed));

    let Some(editor) = installed else {
        println!("No known editor found. Please install one like vim, nano, or gedit.");
        return;
    };

    let Some(file_name) = args.get(1) else {
        println!("No file name provided to edit.");
        return;
    };

    let mut file_path = install_dir.join(file_name);

    // Optional: add .sh if missing
    if file_path.extension().is_none() {
        file_path.set_extension("sh");
    }

    if !file_path.exists() {
        println!("File not found: {}", file_path.display());
        return;
    }

    let cmd = format!("sudo {editor} {}", file_path.display()); // remove sudo unless needed
    run_shell_command(&cmd);
}
