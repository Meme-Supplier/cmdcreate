/// Module for handling command editing functionality in cmdcreate
///
/// This module provides functionality to edit existing commands using various
/// text editors. It supports multiple popular editors and handles editor
/// detection, command validation, and secure editing of system commands.
use crate::{
    cmds::tools::retrieve_commands, // Command management utilities
    utils::{
        colors::COLORS,                        // Terminal color formatting
        msgs::error,                           // Error message handling
        sys::{return_args, run_shell_command}, // System operations
    },
};
use std::process::Command;

/// List of supported text editors for command editing
///
/// These editors are checked in order until a installed editor is found.
/// The list includes both terminal-based and GUI editors.
pub const SUPPORTED_EDITORS: [&str; 13] = [
    "nvim",
    "vi",
    "vim",
    "nano",
    "micro",
    "code",
    "code-insiders",
    "gedit",
    "kate",
    "kwrite",
    "emacs",
    "vscodium",
    "vscodium-insiders",
];

/// Checks if a specific text editor is installed on the system
///
/// # Arguments
/// * `editor` - The name of the editor to check
///
/// # Returns
/// * `bool` - true if the editor is installed, false otherwise
pub fn is_editor_installed(editor: &str) -> bool {
    Command::new("which")
        .arg(editor)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Edits a specified command using the first available supported editor
///
/// This function:
/// 1. Validates the command exists
/// 2. Finds an installed editor
/// 3. Opens the command file in the editor with proper permissions
///
/// # Usage
/// ```bash
/// cmdcreate edit <command_name>
/// ```
pub fn edit() {
    // Initialize color codes for terminal output formatting
    let (blue, yellow, reset) = (COLORS.blue, COLORS.yellow, COLORS.reset);

    // Get command line arguments and validate argument count
    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}remove{yellow} <command>{reset}");
        return;
    }

    // Get the installation directory for commands
    let binding = retrieve_commands("dir");
    let install_dir = binding.first();

    // Validate installation directory exists
    let Some(install_dir) = install_dir else {
        error("Installation directory not found.", "");
        return;
    };

    // Find the first installed editor from the supported list
    let Some(editor) = SUPPORTED_EDITORS
        .iter()
        .find(|&&ed| is_editor_installed(ed))
    else {
        error("No known editor is installed on your device.", "");
        return;
    };

    // Construct the full path to the command file
    let file_path = install_dir.join(args.get(1).unwrap());

    // Validate the command exists
    if !file_path.exists() {
        error(
            "Command not installed:",
            &format!("{yellow}\"{}\"{reset}", &args.get(1).unwrap()),
        )
    }

    // Open the command file in the selected editor with sudo permissions
    run_shell_command(&format!("sudo {editor} {}", file_path.display()))
}
