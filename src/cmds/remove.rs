/// Module for handling command removal functionality in cmdcreate
///
/// This module provides functionality to safely remove custom commands
/// that were previously installed using cmdcreate. It includes safety
/// checks and user confirmation before removal.
use crate::{
    cmds::tools::{is_command_installed, retrieve_commands}, // Command validation tools
    utils::{
        colors::COLORS,                              // Terminal color formatting
        fs::delete_file,                             // File system operations
        msgs::ask_for_confirmation,                  // User interaction
        sys::{return_args, run_shell_command, VARS}, // System operations and variables
    },
};

/// Removes a specified command from the system
///
/// This function:
/// 1. Validates the command exists
/// 2. Asks for user confirmation
/// 3. Removes both the command file and system symlink
///
/// # Usage
/// ```bash
/// cmdcreate remove <command_name>
/// ```
pub fn remove() {
    // Initialize color codes for terminal output formatting
    let (blue, yellow, red, green, reset) = (
        COLORS.blue,
        COLORS.yellow,
        COLORS.red,
        COLORS.green,
        COLORS.reset,
    );

    // Get command line arguments and validate argument count
    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}remove {yellow}<command>{reset}");
        return;
    }

    // Extract the command name to be removed
    let name = args.get(1).unwrap();

    // Check if there are any installed commands
    if retrieve_commands("installed").is_empty() {
        return; // Exit if no commands are installed
    }

    // Verify that the specified command exists
    is_command_installed(name);

    // Request user confirmation before deletion
    ask_for_confirmation(&format!(
        "{red}Are you sure you want to delete command{yellow} \"{name}\"{red}?{reset}"
    ));

    // Remove the command file from cmdcreate's storage
    delete_file(&format!(
        "{}/.local/share/cmdcreate/files/{name}",
        VARS.home
    ));

    // Remove the system-wide command symlink
    run_shell_command(&format!("sudo rm -f /usr/bin/{name}"));

    // Confirm successful removal to user
    println!("\n{green}Removed command \"{name}\"");
}
