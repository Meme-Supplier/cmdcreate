/// Module for displaying a list of installed commands in cmdcreate
///
/// This module provides functionality to view all currently installed custom
/// commands in the system. It shows the total count and names of all commands
/// that have been created using cmdcreate.
use crate::{
    cmds::tools::retrieve_commands, // Command retrieval utility
    utils::colors::COLORS,          // Terminal color formatting
};

/// Lists all installed commands in the system
///
/// This function:
/// 1. Retrieves all installed commands
/// 2. Displays the total count
/// 3. Shows a formatted list of command names
///
/// # Usage
/// ```bash
/// cmdcreate list
/// ```
pub fn list() {
    // Initialize color codes for terminal output formatting
    let (blue, reset) = (COLORS.blue, COLORS.reset);

    // Get list of all installed commands
    let installed_scripts = retrieve_commands("installed");
    if installed_scripts.is_empty() {
        return; // Exit if no commands are installed
    }

    // Display header with total command count
    println!(
        "Installed commands: ({blue}{}{reset})\n--------",
        installed_scripts.len()
    );

    // Print each command name, extracting just the filename without path
    for script in installed_scripts {
        println!(
            "{}",
            script.file_stem().unwrap_or_default().to_string_lossy()
        )
    }
}
