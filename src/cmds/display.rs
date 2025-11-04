/// Module for displaying command contents in cmdcreate
///
/// This module provides functionality to view the contents of existing custom
/// commands. It allows users to inspect the actual script or code that makes
/// up a command without having to navigate to the command file directly.
use std::process::exit;

use crate::utils::{
    colors::COLORS,           // Terminal color formatting
    fs::read_file_to_string,  // File system operations
    sys::{return_args, VARS}, // System operations and variables
};

/// Displays the contents of a specified command
///
/// This function:
/// 1. Validates the command name is provided
/// 2. Reads the command file contents
/// 3. Displays the contents with formatting
///
/// # Usage
/// ```bash
/// cmdcreate display <command_name>
/// ```
pub fn display() {
    // Initialize color codes for terminal output formatting
    let (blue, yellow, reset) = (COLORS.blue, COLORS.yellow, COLORS.reset);

    // Get command line arguments and validate argument count
    let args = return_args();
    if args.len() < 2 {
        println!("Usage:\ncmdcreate {blue}display {yellow}<command>{reset}");
        exit(0); // Exit if no command name provided
    }

    // Extract the command name to display
    let cmd = args.get(1).unwrap();

    // Read and display the command's contents with formatting
    println!(
        "Contents of command: {blue}\"{cmd}\"{reset}\n--------\n{}",
        read_file_to_string(&format!("{}/.local/share/cmdcreate/files/{cmd}", VARS.home)).trim()
    );
}
