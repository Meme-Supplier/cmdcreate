/// Module for handling command creation functionality in cmdcreate
///
/// This module provides functionality to create new custom commands and install
/// them system-wide. It handles the creation of command files, setting proper
/// permissions, and creating system symlinks.
use crate::utils::{
    colors::COLORS,                              // Terminal color formatting
    fs::write_to_file,                           // File system operations
    sys::{return_args, run_shell_command, VARS}, // System operations and variables
};

/// Creates a new command in the system
///
/// This function:
/// 1. Creates a new command file with the specified contents
/// 2. Sets executable permissions on the file
/// 3. Creates a system-wide symlink for easy access
///
/// # Usage
/// ```bash
/// cmdcreate create <command_name> <command_contents>
/// ```
pub fn create() {
    // Initialize color codes for terminal output formatting
    let (blue, yellow, green, reset) = (COLORS.blue, COLORS.yellow, COLORS.green, COLORS.reset);

    // Get command line arguments and validate argument count
    let args = return_args();
    if args.len() < 3 {
        println!("Usage:\ncmdcreate {blue}create {yellow}<command> <contents>{reset}");
        return;
    }

    // Extract command name and contents from arguments
    let (name, contents) = (args.get(1).unwrap(), args.get(2).unwrap());

    // Construct the path for the new command file
    let script = &format!("{}/.local/share/cmdcreate/files/{name}", VARS.home);

    // Write the command contents to the file
    write_to_file(script, contents);

    // Make the script executable and create a system-wide symlink
    run_shell_command(&format!(
        "
            chmod +x {script}; \
            sudo ln -sf {script} /usr/bin/{name}
            ",
    ));

    // Confirm successful creation to user
    println!("\n{green}Success! Created command: {blue}\"{name}\"{reset}",);
}
