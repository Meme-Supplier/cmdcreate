//! Module for importing commands from backup files in cmdcreate
//!
//! This module provides functionality to restore commands from a backup file
/// created by the export function. It processes the backup file, recreates
/// the commands in the system, and sets up proper permissions and symlinks.
///
/// The import process expects files in the format:
/// ```text
/// command_name, command_contents
/// ```
/// Where each line represents one command to be restored.
use crate::utils::{
    colors::COLORS,                              // Terminal color formatting
    fs::{read_file_to_string, write_to_file},    // File operations
    sys::{return_args, run_shell_command, VARS}, // System operations and variables
};

/// Imports commands from a backup file
///
/// This function:
/// 1. Reads the specified backup file
/// 2. Parses each line for command name and contents
/// 3. For each command:
///    - Creates the command file
///    - Sets executable permissions
///    - Creates system symlinks
///
/// # Usage
/// ```bash
/// cmdcreate import <backup_file>
/// ```
///
/// # File Format
/// Each line in the backup file should be in the format:
/// ```text
/// command_name, command_contents
/// ```
pub fn import() {
    // Initialize color codes for terminal output formatting
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);

    // Get command line arguments and validate argument count
    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}import {yellow}<input file>{reset}");
        return;
    }

    // Get the backup file path and read its contents
    let import_file = &args[1];
    let contents = read_file_to_string(import_file);

    // Process each line in the backup file
    for line in contents.lines() {
        // Skip invalid lines that don't contain the separator
        if !line.contains(',') {
            continue;
        }

        // Split the line into command name and contents
        let mut parts = line.splitn(2, ',');
        let name = parts.next().unwrap().trim();
        let data = parts.next().unwrap().trim();

        // Display progress to user
        println!("{blue}Installing command: \"{green}{name}{reset}\"");

        // Create the command file in the installation directory
        let script = format!("{}/.local/share/cmdcreate/files/{}", VARS.home, name);
        write_to_file(&script, data);

        // Set executable permissions and create system symlink
        run_shell_command(&format!(
            "
            chmod +x {script}; \
            sudo ln -sf {script} /usr/bin/{name}
            ",
        ));
    }

    // Confirm successful import to user with the source file
    println!(
        "\n{green}Successfully imported commands from: {blue}\"{import_file}\"{green}.{reset}"
    );
}
