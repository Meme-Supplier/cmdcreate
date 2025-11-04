/// Module for exporting installed commands in cmdcreate
///
/// This module provides functionality to export all installed commands
/// to a backup file. The export includes both command names and their
/// contents, allowing for later restoration or transfer to another system.
///
/// The exported file uses a simple format where each line contains:
/// `command_name, command_contents`
use crate::cmds::tools::retrieve_commands; // Command listing functionality
use crate::utils::{
    colors::COLORS,                           // Terminal color formatting
    fs::{read_file_to_string, write_to_file}, // File operations
    sys::{return_args, VARS},                 // System operations and variables
};

/// Exports all installed commands to a backup file
///
/// This function:
/// 1. Takes an output directory path from command arguments
/// 2. Retrieves all installed commands
/// 3. For each command:
///    - Reads its contents
///    - Writes name and contents to the export file
/// 4. Creates a single export.cmdcreate file containing all commands
///
/// # Usage
/// ```bash
/// cmdcreate export <output_directory>
/// ```
///
/// # Export Format
/// The export file contains one command per line in the format:
/// ```text
/// command_name, command_contents
/// ```
pub fn export() {
    // Initialize color codes for terminal output formatting
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);

    // Get command line arguments and validate argument count
    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}export {yellow}<output directory>{reset}");
        return;
    }

    // Extract the output directory path from arguments
    let output_path = args.get(1).unwrap();

    // Process each installed command and write to export file
    for script in retrieve_commands("installed") {
        // Extract command name from file path
        let cmd = script.file_stem().unwrap_or_default().to_string_lossy();

        // Read the command's contents from its installation location
        let cmd_contents =
            read_file_to_string(&format!("{}/.local/share/cmdcreate/files/{cmd}", VARS.home));

        // Write the command and its contents to the export file
        write_to_file(
            &format!("{output_path}/export.cmdcreate"),
            &format!("{cmd}, {cmd_contents}"),
        );
    }

    // Confirm successful export to user with the file location
    println!(
        "{green}Successfully exported installed commands to:{blue} \"{output_path}export.cmdcreate\"{green}.{reset}"
    )
}
