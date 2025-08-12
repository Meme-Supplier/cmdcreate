use std::process::Command;

use crate::tools::utils::*;

pub const SUPPORTED_EDITORS: [&str; 11] = [
    "nvim",
    "vi",
    "vim",
    "nano",
    "micro",
    "code",
    "code-insiders",
    "gedit",
    "kate",
    "emacs",
    "vscodium",
];

fn is_editor_installed(editor: &str) -> bool {
    Command::new("which")
        .arg(editor)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn edit() {
    let args = return_args();

    let install_dir = retrieve_commands("dir").get(0).cloned();

    let Some(install_dir) = install_dir else {
        println!("Install dir not found.");
        return;
    };

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

    let file_path = install_dir.join(file_name);

    if !file_path.exists() {
        println!("File not found: {}", file_path.display());
        return;
    }

    let cmd = format!("{editor} {}", file_path.display());
    run_shell_command(&cmd, || {
        println!("Error: Unable to execute command: \n {}", &cmd);
        return;
    });
}
